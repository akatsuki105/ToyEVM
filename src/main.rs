extern crate python_input;

use python_input::input;

mod state;
mod util;
mod vm;
use ethereum_types::H160;
use std::env;
use std::process::exit;

/// init
fn main() {
    exit(run());
}

/// execute application
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
            }
            "help" => {
                help();
                return 0;
            }
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
    println!("world state: {}", ws.get_hash());

    loop {
        let command = input("select next action: transaction(1) or deploy(2) => ")
            .trim_end()
            .to_string();

        match &command[..] {
            "transaction" | "1" => {
                let code_owner = input("contract address    > ").trim_end().to_string();
                let sender = input("sender address      > ").trim_end().to_string();
                transaction(
                    &mut ws,
                    util::to_h160(&code_owner),
                    util::to_h160(&sender),
                    1_000_000_000,
                    100_000_000_000_000_000,
                );
                println!();
            }
            "deploy" | "2" => {
                let code = &input("contract code      > ").trim_end().to_string();
                deploy(&mut ws, code);
                println!();
            }
            "exit" | "quit" => {
                return 0;
            }
            "help" => {
                println!("help");
                println!("transaction or 1: execute transaction");
                println!("deploy or 2: deploy contract");
                println!("exit or quit: quit EVM");
                println!("help: print help");
                println!();
            }
            c => {
                println!("{} is invalid command", c);
                continue;
            }
        }

        // ステートを更新
        ws.update_state();
        println!("world state: {}", ws.get_hash());
    }
    return 0;
}

/// execute transaction
fn transaction(
    ws: &mut state::WorldState,
    code_owner: H160,
    sender: H160,
    gas_price: usize,
    value: usize,
) {
    let mut env = vm::Environment::new(code_owner, sender, gas_price, value);
    let mut contract = ws.get_account_state(&code_owner);
    env.set_code(util::str_to_bytes(&contract.get_code()));
    let mut vm = vm::VM::new(env);
    vm.exec_transaction(&mut contract);

    let mut sender_account = ws.get_account_state(&sender);
    sender_account.increment_nonce();
}

/// deploy contract
fn deploy(ws: &mut state::WorldState, code: &str) {
    // TODO: 正確にする
    let account_state = state::AccountState::new(code.to_string());
    let address = H160::random();
    ws.push_account_state(address, account_state);
    println!("{} is deployed!", hex::encode(address));
}

/// print help
fn help() {
    println!("usage: toyevm <command> [<args>] ");
    println!();
    println!("run       start EVM");
    println!("disasm    disassemble EVM bytecode");
    println!("help      print help message");
}
