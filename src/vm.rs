extern crate ethereum_types;

use ethereum_types::{U256};

#[allow(dead_code)]
pub struct VM {
    code: Vec<u8>,
    pc: usize,
    gas: usize,
    sp: usize,
    stack: Vec<U256>,
    memory: Vec<u8>,
}

#[allow(dead_code)]
impl VM {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
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

    pub fn exec(&mut self) {
        let opcode = &self.code[self.pc];
        self.pc += 1;

        match opcode {
            0x01 => self.op_add(),
            0x52 => self.op_mstore(),
            0x60 => self.op_push1(),
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
}

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
            operand[31-i] = self.code[self.pc];
            self.pc += 1;
        }
        self.consume_gas(3);
        self.push(operand.into());
    }

    fn op_push1(&mut self) {
        self.op_push(1);
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

    fn op_return(&mut self) {
        panic!("op_return: not implement error");
    }
}

#[test]
fn test_new() {
    let vm = VM::new(vec![0x60, 0x05, 0x60, 0x04, 0x01]);
    assert_eq!(vm.code, vec![0x60, 0x05, 0x60, 0x04, 0x01]);
    assert_eq!(vm.pc, 0);
    assert_eq!(vm.gas, 10000000000);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.stack, Vec::default());
}

#[test]
fn test_push1() {
    let mut vm = VM::new(vec![0x60, 0x05]);
    vm.exec(); // PUSH1 5
    assert_eq!(vm.pc, 2);
    assert_eq!(vm.gas, 9999999997);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![5.into()]);
}

#[test]
fn test_add() {
    let mut vm = VM::new(vec![0x60, 0x05, 0x60, 0x04, 0x01]);
    vm.exec(); // PUSH1 5
    vm.exec(); // PUSH1 4
    vm.exec(); // ADD
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![9.into()]);
}

#[test]
fn test_mstore() {
    let mut vm = VM::new(vec![0x60, 0x05, 0x60, 0x04, 0x01, 0x60, 0x00, 0x52]);
    vm.exec(); // PUSH1 5
    vm.exec(); // PUSH1 4
    vm.exec(); // ADD
    vm.exec(); // PUSH1 0
    vm.exec(); // MSTORE
    assert_eq!(vm.pc, 8);
    assert_eq!(vm.gas, 9999999982);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.memory[0x1f], 0x09);
}
