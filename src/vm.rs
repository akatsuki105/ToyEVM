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

    pub fn exec(&mut self) {
        let opcode = &self.code[self.pc];
        match opcode {
            1 => {
                // ADD
            }
            60 => {
                // PUSH1
            }
            _ => {
                panic!("exec: invalid opcode.");
            }
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