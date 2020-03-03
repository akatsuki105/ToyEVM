extern crate ethereum_types;
extern crate hex;

use ethereum_types::{H160, U256};

#[allow(dead_code)]
pub struct Environment {
    code_owner: H160, // 実行するコントラクトのオーナー
    sender: H160, // トランザクションの送信者
    gas_price: usize, // gasのETHレート
    value: usize, // トランザクションに添付されたEth
    code: Vec<u8>, // 実行されるEVMバイトコード
    input: Vec<u8>, // トランザクションに渡されるデータ(solidityでは引数として渡される)
}

#[allow(dead_code)]
impl Environment {
    pub fn new(code_owner: H160, sender: H160, gas_price: usize, value: usize) -> Self {
        return Self {
            code_owner,
            sender,
            gas_price,
            value,
            code: Default::default(),
            input: Default::default(),
        }
    }

    pub fn set_code(&mut self, code: Vec<u8>) {
        self.code = code;
    }

    pub fn set_input(&mut self, input: Vec<u8>) {
        self.input = input;
    }
}

#[allow(dead_code)]
pub struct VM {
    env: Environment,
    pc: usize,
    gas: usize,
    sp: usize,
    stack: Vec<U256>,
    memory: Vec<u8>,
}

/// Opcodeの実行で使われる汎用的な関数を実装している
#[allow(dead_code)]
impl VM {
    pub fn new(env: Environment) -> Self {
        Self {
            env,
            pc: 0,
            gas: 10000000000,
            sp: 0,
            stack: Default::default(),
            memory: Default::default(),
        }
    }

    fn push(&mut self, value: U256) {
        self.stack.push(value);
        self.sp += 1;
    }

    fn pop(&mut self) -> U256 {
        let value = self.stack.pop().unwrap();
        self.sp -= 1;
        return value;
    }

    fn exec(&mut self) {
        let opcode = &self.env.code[self.pc];
        self.pc += 1;

        match opcode {
            0x01 => self.op_add(),
            0x35 => self.op_calldataload(),
            0x51 => self.op_mload(),
            0x52 => self.op_mstore(),
            0x60 => self.op_push1(),
            0x61 => self.op_push2(),
            0xf3 => self.op_return(),
            _ => panic!("exec: invalid opcode."),
        }
    }

    fn consume_gas(&mut self, gas: usize) {
        if self.gas >= gas {
            self.gas -= gas;
        } else {
            panic!("consume_gas: There is a shortage of gas.");
        }
    }

    pub fn exec_transaction(&mut self) {
        loop {
            if self.pc >= self.env.code.len() {
                break;
            }

            self.exec();
        }
    }
}

/// 具体的なOpcodeハンドラの実装
#[allow(dead_code)]
impl VM {
    fn op_add(&mut self) {
        self.consume_gas(3);
        let operand1 = self.pop();
        let operand2 = self.pop();
        let result = operand1 + operand2;
        self.push(result);
    }

    fn op_push(&mut self, length: usize) {
        let mut operand = [0; 32];
        for i in 0..length {
            operand[32-length+i] = self.env.code[self.pc];
            self.pc += 1;
        }
        self.consume_gas(3);
        self.push(operand.into());
    }

    fn op_push1(&mut self) {
        self.op_push(1);
    }

    fn op_push2(&mut self) {
        self.op_push(2);
    }

    fn op_mstore(&mut self) {
        self.consume_gas(6);
        let address = self.pop().as_u32() as usize;
        let operand = self.pop();
        let bytes: [u8; 32] = operand.into();
        for (i, b) in bytes.iter().enumerate() {
            self.memory.insert(address+i, *b);
        }
    }

    fn op_mload(&mut self) {
        self.consume_gas(3);
        // startを先頭アドレスしてstart+32までの32byteの値をロード
        let start = self.pop().as_u32() as usize;
        let mut bytes: [u8; 32] = [0; 32];
        for i in 0..32 {
            let b = self.memory[start+i];
            bytes[i] = b;
        }
        
        // stackにpush
        self.push(bytes.into());
    }

    fn op_return(&mut self) {
        panic!("op_return: not implement error");
    }

    /// スタックからpopした値をstartとしてinputのstartの位置からstart+32の位置までの32byteのデータをstackの先頭に積む
    fn op_calldataload(&mut self) {
        self.consume_gas(3);
        let start = self.pop().as_u32() as usize;
        let bytes: [u8; 32] = slice_to_array(&self.env.input[start..start+32]);
        self.push(bytes.into());
    }
}

fn str_to_bytes(src: &str) -> Vec<u8> {
    let bytes = hex::decode(src).expect("str_to_bytes: decoding failed");
    return bytes;
}

fn slice_to_array(s: &[u8]) -> [u8; 32] {
    if s.len() != 32 {
        panic!("slice_to_array: length must be 32");
    }

    let mut result = [0; 32];
    for i in 0..32 {
        result[i] = s[i];
    }
    return result;
}

#[test]
fn test_new() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005600401"));
    let vm = VM::new(env);
    assert_eq!(vm.env.code, vec![0x60, 0x05, 0x60, 0x04, 0x01]);
    assert_eq!(vm.pc, 0);
    assert_eq!(vm.gas, 10000000000);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.stack, Vec::default());
}

#[test]
fn test_push1() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 2);
    assert_eq!(vm.gas, 9999999997);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![5.into()]);
}

#[test]
fn test_add() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005600401"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![9.into()]);
}

#[test]
fn test_mstore() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005600401600052"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 8);
    assert_eq!(vm.gas, 9999999982);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.memory[0x1f], 0x09);
}

#[test]
fn test_mload() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005600401600052600051"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 11);
    assert_eq!(vm.gas, 9999999976);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![0x09.into()]);
}

#[test]
fn test_add2() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("61010161010201"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 7);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![0x0203.into()]);
}


#[test]
fn test_calldataload() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("60003560203501"));
    env.set_input(str_to_bytes("00000000000000000000000000000000000000000000000000000000000000050000000000000000000000000000000000000000000000000000000000000004"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 7);
    assert_eq!(vm.gas, 9999999985);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![0x09.into()]);
}
