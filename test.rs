use npc;

fn calculate(n) {
  let x = n.getx();
  println(`Hello World {x}`);
  n.setx(88);
  let map = n.getmap();
  println(`Hello World {map}`);
  n.getx() + n.gety()
}

fn npc_init() {
  let npc = npc::Npc::new();
  npc.setmap(5);
  npc.setx(10);
  npc.sety(5);
  npc
}

fn npc_name(id) {
  match id {
    1 => "Bob",
    2 => "Sally",
    3 => "Jynx",
  }
}

fn npc_speech(npc) {
 2
}

fn npc_move(npc) {
 2
}

fn npc_scene_move(npc) {
 2
}

fn npc_scene_speech(npc) {
 2
}

fn npc_block(npc) {
 2
}

fn npc_drops(npc) {
 2
}

fn npc_skills(npc) {
 2
}

fn npc_quests(npc) {
 3
}

fn npc_shop_items(npc) {
 0
}
