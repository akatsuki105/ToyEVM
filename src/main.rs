mod vm;
mod state;
mod util;

fn main() {
    let ws = state::WorldState::new("./config/config.json");
    println!("world state: {}", ws.get_hash());
}
