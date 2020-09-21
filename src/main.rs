mod npc;

use runestick::{Value, FromValue as _, Vm};
use npc::*;
use std::sync::Arc;

fn main() {
  let mut world = NpcStorage::new();

  let mut npc = Npc::new();
  npc.vit.hp = 5;
  npc.pos.x = 4;
  npc.pos.y = 8;
  npc.pos.map = 2;
  world.insert(npc);

  let mut npc = Npc::new();
  npc.vit.hp = 12;
  npc.pos.x = 6;
  npc.pos.y = 6;
  npc.pos.map = 1;
  world.insert(npc);

  let mut testscript = match RuneNPCObject::new("test.rs") {
    Some(n) => n,
    None => {
      println!("Error did not load script or something derp.");
      return;
    }
  };

  let vm = Vm::new(testscript.context.clone(), testscript.unit.clone());

  let mut npc: Npc = match vm.call(&["npc_init"], ()) {
    Ok(n) => Npc::from_value(n).unwrap(),
    Err(e) => {
      println!("Error: {}", e);
      return;
    }
  };

  println!("x: {}, y: {}, map: {}", npc.pos.x, npc.pos.y, npc.pos.map);

  world.insert(npc);
  let npc = world.npcs.get(&2).unwrap().borrow_mut();

  let vm = Vm::new(testscript.context.clone(), testscript.unit.clone());
  let execution = match vm.call(&["calculate"], (&*npc,)) {
    Ok(n) => n,
    Err(e) => {
      println!("Error: {}", e);
      return;
    }
  };

  let value = i64::from_value(execution).unwrap();
  println!("{}", value);

  for i in 0..world.ids.len() {
    let mut npc1 = world.npcs.get(&world.ids[i]).unwrap().borrow_mut();

    for x in 0..world.ids.len() {
      if x != i {
        let mut npc2 = world.npcs.get(&world.ids[x]).unwrap().borrow_mut();

        npc2.damage_npc(1);

        if npc2.id != 1 {
          npc1.damage_npc(2);
        }
      }
    }
  }

  for i in world.ids {
    let npc = world.npcs.get(&i).unwrap().borrow_mut();
    let vm = Vm::new(testscript.context.clone(), testscript.unit.clone());
    let name = match vm.call(&["npc_name"], (npc.id,)) {
      Ok(n) => String::from_value(n).unwrap(),
      Err(e) => {
        println!("Error: {}", e);
        return;
      }
    };

    println!("Name: {}, x: {}, y: {}, map: {}, hp: {}", name, npc.pos.x, npc.pos.y, npc.pos.map, npc.vit.hp);
  }

}
