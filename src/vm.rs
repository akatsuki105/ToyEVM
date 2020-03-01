extern crate ethereum_types;

use ethereum_types::{U256};

#[allow(dead_code)]
pub struct VM {
    code: Vec<u8>,
    pc: usize,
    gas: usize,
    sp: usize,
    stack: Vec<U256>,
}

impl VM {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            pc: 0,
            gas: 10000000000,
            sp: 0,
            stack: Default::default(),
        }
    }

    fn push(&mut self, value: U256) {
        self.stack.push(value);
        self.sp += 1;
    }

    fn push1(&mut self) {
        let mut operand = [0; 32];
        for i in 0..1 {
            operand[31-i] = self.code[self.pc];
            self.pc += 1;
        }
        self.consume_gas(3);
        self.push(operand.into());
    }

    pub fn exec(&mut self) {
        let opcode = &self.code[self.pc];
        self.pc += 1;

        match opcode {
            1 => {
                // ADD
            }
            60 => {
                self.push1();
            }
            _ => {
                panic!("exec: invalid opcode.");
            }
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

#[test]
fn test_new() {
    let vm = VM::new(vec![60, 05, 60, 04, 01]);
    assert_eq!(vm.code, vec![60, 05, 60, 04, 01]);
    assert_eq!(vm.pc, 0);
    assert_eq!(vm.gas, 10000000000);
    assert_eq!(vm.sp, 0);
    assert_eq!(vm.stack, Vec::default());
}

#[test]
fn test_push1() {
    let mut vm = VM::new(vec![60, 05]);
    vm.exec();
    assert_eq!(vm.pc, 2);
    assert_eq!(vm.gas, 9999999997);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![5.into()]);
}