use std::fs;
use std::sync::Arc;

use rune::termcolor::{ColorChoice, StandardStream};
use rune::EmitDiagnostics as _;
use runestick::{Hash, Source, Vm};

use crate::npc::npc_module;

#[derive( Debug)]
pub struct RuneNPCObject {
  pub unit: Arc<runestick::Unit>,
  pub context: Arc<runestick::Context>,
  pub sources: rune::Sources,
}

impl RuneNPCObject {
  pub fn new(path: &str) -> Option<RuneNPCObject> {
    let mut context = match rune::default_context() {
      Ok(context) => context,
      Err(_) => return None,
    };
    let options = rune::Options::default();
    let mut warnings = rune::Warnings::new();
    let mut sources = rune::Sources::new();
    let mut errors = rune::Errors::new();

    let contents = fs::read_to_string(path).expect("Something went wrong reading the script");
    sources.insert(Source::new("script", contents));

    match context.install(match &npc_module() {
      Ok(n) => n,
      Err(error) => {
        println!("Error did not load npc module {}", error);
        return None;
      }
    }) {
      Ok(_) => {}
      Err(error) => {
        println!("Error did not load npc module {}", error);
        return None;
      }
    }

    let context = Arc::new(context);
    let unit = match rune::load_sources(&*context, &options, &mut sources, &mut errors, &mut warnings) {
      Ok(unit) => unit,
      Err(_error) => {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        errors.emit_diagnostics(&mut writer, &sources).unwrap();
        return None;
      }
    };

    if !warnings.is_empty() {
      let mut writer = StandardStream::stderr(ColorChoice::Always);
      warnings.emit_diagnostics(&mut writer, &sources).unwrap();
    }

    let names = vec![
      "npc_init",
      "npc_speech",
      "npc_move",
      "npc_scene_move",
      "npc_scene_speech",
      "npc_block",
      "npc_drops",
      "npc_skills",
      "npc_quests",
      "npc_shop_items",
    ];

    for name in names {
      match unit.lookup(Hash::type_hash(&[name])) {
        Some(_) => continue,
        None => {
          println!(
            "Error did not load npc module {}, Script missing required function {}.",
            path,
            name
          );
          return None;
        }
      }
    }

    let unit = Arc::new(unit);

    Some(RuneNPCObject {
      unit,
      context,
      sources,
    })
  }
}
