mod vm;
mod state;
mod util;
use std::process::exit;
use ethereum_types::{H160};
use std::str::FromStr;

fn main() {
    exit(run());
}

fn run() -> i32 {
    let mut ws = state::WorldState::new("./config/config.json");
    println!("world state: {}", ws.get_hash());

    let code_owner = H160::from_str("899C5C9bf8396Ba2c14f819C6D807b96990F86EE").unwrap();
    let sender = H160::from_str("9C2b303267DcFc6F247E777f1e412a2b08E57998").unwrap();
    transaction(&mut ws, code_owner, sender, 21000 , 100);

    ws.update_state();
    println!("world state: {}", ws.get_hash());
    return 0;
}

fn transaction(ws: &mut state::WorldState, code_owner: H160, sender: H160, gas_price: usize, value: usize) {
    let mut env = vm::Environment::new(code_owner, sender, gas_price, value);
    let mut contract = ws.get_account_state(&code_owner);
    env.set_code(util::str_to_bytes(&contract.get_code()));
    let mut vm = vm::VM::new(env);
    vm.exec_transaction(&mut contract);

    let mut sender_account = ws.get_account_state(&sender);
    sender_account.increment_nonce();
}
