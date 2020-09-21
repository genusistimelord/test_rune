use runestick::Any;

pub fn npc_module() -> Result<runestick::Module, runestick::ContextError> {
  let mut module = runestick::Module::new(&["npc"]);

  module.ty::<Npc>()?;
  module.function(&["Npc", "new"], Npc::new)?;
  module.inst_fn("getx", Npc::getx)?;
  module.inst_fn("gety", Npc::gety)?;
  module.inst_fn("getmap", Npc::getmap)?;
  module.inst_fn("gethp", Npc::gethp)?;
  module.inst_fn("setx", Npc::setx)?;
  module.inst_fn("sety", Npc::sety)?;
  module.inst_fn("setmap", Npc::setmap)?;
  module.inst_fn("sethp", Npc::sethp)?;

  module.ty::<Position>()?;
  module.function(&["Position", "new"], Position::new)?;
  module.ty::<Vitals>()?;
  module.function(&["Vitals", "new"], Vitals::new)?;

  Ok(module)
}

/// A tag, telling us that the entity is an npc.
#[derive(Clone, Copy, Debug, PartialEq, Any, Default)]
pub struct Npc {
  pub id: u64,
  pub pos: Position,
  pub vit: Vitals,
}

#[derive(Clone, Copy, Debug, PartialEq, Any, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Any, Default)]
pub struct Vitals {
  pub hp: u32,
  pub sp: u32,
  pub mp: u32,
  pub maxhp: u32,
  pub maxsp: u32,
  pub maxmp: u32,
}

impl Npc {
  #[inline(always)]
  pub fn new() -> Npc {
    Npc::default()
  }

  pub fn getx(&self) -> i32 {
    self.pos.x
  }

  pub fn gety(&self) -> i32 {
    self.pos.y
  }

  pub fn getmap(&self) -> i64 {
    self.pos.map
  }

  pub fn gethp(&self) -> u32 {
    self.vit.hp
  }

  pub fn setx(&mut self, x: i32) {
    self.pos.x = x;
  }

  pub fn sety(&mut self, y: i32) {
    self.pos.y = y;
  }

  pub fn setmap(&mut self, map: i64) {
    self.pos.map = map;
  }

  pub fn sethp(&mut self, hp: u32)  {
    self.vit.hp = hp;
  }

  #[inline(always)]
  pub fn damage_npc(&mut self, damage: u32) {
    if self.vit.hp < damage  {
      self.vit.hp = 0;
    } else {
      self.vit.hp -= damage;
    }
  }
}

impl Position {
  #[inline(always)]
  pub fn new(x: i32, y: i32, map: i64 ) -> Position {
    Position {
      x,
      y,
      map
    }
  }
}

impl Vitals {
  #[inline(always)]
  pub fn new(maxhp: u32, maxsp: u32, maxmp: u32,) -> Self {
    Self {
      hp: maxhp,
      sp: maxsp,
      mp: maxmp,
      maxhp,
      maxsp,
      maxmp,
    }
  }
}