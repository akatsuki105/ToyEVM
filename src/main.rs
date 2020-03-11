mod vm;
mod state;
mod util;
use std::env;
use std::process::exit;
use ethereum_types::{H160};

fn main() {
    exit(run());
}

fn run() -> i32 {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let subcommand = &args[1][..];
        match subcommand {
            "disasm" => {
                if args.len() < 3 {
                    println!("please input EVMbytecode");
                    return 1;
                }
                let code = &args[2][..];
                vm::VM::disassemble(code);
                return 0;
            },
            "help" => {
                print_help();
                return 0;
            },
            "run" => {}
            "deploy" => {}
            _ => {
                println!("subcommand is needed");
                return 1;
            }
        }
    }

    let mut ws = state::WorldState::new("./config/config.json");
    // ステートの初期化
    ws.update_state();
    println!("initial world state: {}", ws.get_hash());

    let code_owner = util::to_h160("899C5C9bf8396Ba2c14f819C6D807b96990F86EE");
    let sender = util::to_h160("9C2b303267DcFc6F247E777f1e412a2b08E57998");
    transaction(&mut ws, code_owner, sender, 21000 , 100);

    // ステートを更新
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

fn print_help() {
    println!("usage: toyevm <command> [<args>] ");
    println!();
    println!("run       start EVM");
    println!("disasm    disassemble EVM bytecode");
    println!("help      print help message");
}