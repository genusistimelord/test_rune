use std::cell::RefCell;
use std::collections::HashMap;
use crate::npc::{Npc, RuneNPCObject};

pub struct NpcStorage {
  pub idmax: u64,
  /// used to iterate over all active npcs.
  pub ids: Vec<u64>,
  /// Do not iterate over this directly.
  pub npcs: HashMap<u64, RefCell<Npc>>,
  pub scripts: HashMap<u32, RefCell<RuneNPCObject>>,
}

impl NpcStorage {
  pub fn new() -> Self {
      Self {
          idmax: 1,
          ids: Vec::new(),
          npcs: HashMap::with_capacity(256),
          scripts: HashMap::with_capacity(64),
      }
  }

  pub fn insert(&mut self, mut npc: Npc) {
    npc.id = self.idmax;
    self.ids.push(npc.id.clone());
    self.idmax += 1;
    self.npcs.insert(npc.id, RefCell::new(npc));
  }
}