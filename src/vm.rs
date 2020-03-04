extern crate ethereum_types;
extern crate hex;

use ethereum_types::{H160, U256};

/// トランザクションを実行するのに必要となる環境変数
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
    /// 環境変数のコンストラクタ
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

    /// コードをセットする
    pub fn set_code(&mut self, code: Vec<u8>) {
        self.code = code;
    }

    /// インプットデータをセットする
    pub fn set_input(&mut self, input: Vec<u8>) {
        self.input = input;
    }
}

/// EVMインスタンス
#[allow(dead_code)]
pub struct VM {
    env: Environment, // 環境変数
    pc: usize, // Program Counter
    gas: usize, // gas残量
    sp: usize, // スタックポインタ
    stack: Vec<U256>, // トランザクションのライフサイクルの間保持される一時的なスタック領域
    memory: Vec<u8>, // トランザクションのライフサイクルの間保持される一時的なメモリ領域
}

/// Opcodeの実行で使われる汎用的な関数を実装している
#[allow(dead_code)]
impl VM {
    /// コンストラクタ gasは10000000000とする
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

    /// スタックへのpush
    fn push(&mut self, value: U256) {
        self.stack.push(value);
        self.sp += 1;
    }

    /// スタックからのpop
    fn pop(&mut self) -> U256 {
        let value = self.stack.pop().unwrap();
        self.sp -= 1;
        return value;
    }

    /// EVMバイトコードを1命令実行する
    fn exec(&mut self) {
        let opcode = &self.env.code[self.pc];
        self.pc += 1;

        // opcodeに対応するハンドラを呼び出す
        match opcode {
            0x00 => self.op_stop(),
            0x01 => self.op_add(),
            0x03 => self.op_sub(),
            0x04 => self.op_div(),
            0x0a => self.op_exp(),
            0x35 => self.op_calldataload(),
            0x36 => self.op_calldatasize(),
            0x51 => self.op_mload(),
            0x52 => self.op_mstore(),
            0x56 => self.op_jump(),
            0x57 => self.op_jumpi(),
            0x5b => self.op_jumpdest(),
            0x60 => self.op_push(1),
            0x61 => self.op_push(2),
            0x80 => self.op_dup(1),
            0x90 => self.op_swap(1),
            0xf3 => self.op_return(),
            _ => panic!("exec: invalid opcode. PC: {} Opcode: {}", self.pc-1, opcode),
        }
    }

    fn consume_gas(&mut self, gas: usize) {
        if self.gas >= gas {
            self.gas -= gas;
        } else {
            panic!("consume_gas: There is a shortage of gas.");
        }
    }

    /// トランザクションが終了するまでexecを繰り返す
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
    fn op_stop(&mut self) {}

    /// operand1(スタック1番目) + operand2(スタック2番目)
    fn op_add(&mut self) {
        self.consume_gas(3);
        let operand1 = self.pop();
        let operand2 = self.pop();
        let result = operand1 + operand2;
        self.push(result);
    }

    /// operand1(スタック1番目) - operand2(スタック2番目)
    fn op_sub(&mut self) {
        self.consume_gas(3);
        let operand1 = self.pop();
        let operand2 = self.pop();
        let result = operand1 - operand2;
        self.push(result);
    }

    /// operand1(スタック1番目) // operand2(スタック2番目)
    fn op_div(&mut self) {
        self.consume_gas(5);
        let operand1 = self.pop();
        let operand2 = self.pop();
        let result = operand1 / operand2;
        self.push(result);
    }

    /// operand1(スタック1番目) ** operand2(スタック2番目)
    fn op_exp(&mut self) {
        self.consume_gas(3);
        let operand1 = self.pop();
        let operand2 = self.pop();
        let result = operand1.pow(operand2);
        self.push(result);
    }

    /// lengthバイトpushする
    fn op_push(&mut self, length: usize) {
        let mut operand = [0; 32];
        for i in 0..length {
            operand[32-length+i] = self.env.code[self.pc];
            self.pc += 1;
        }
        self.consume_gas(3);
        self.push(operand.into());
    }

    fn op_dup(&mut self, index: usize) {
        self.consume_gas(3);
        println!("self.pc: {}", self.sp);
        let operand = self.stack[self.sp-1];
        if self.sp > 1 {
            self.stack[self.sp-index-1] = operand;
        } else {
            self.push(operand);
        }
    }

    fn op_swap(&mut self, index: usize) {
        self.consume_gas(3);
        let operand1 = self.stack[self.sp-1];
        let operand2 = self.stack[self.sp-index-1];
        self.stack[self.sp-1] = operand2;
        self.stack[self.sp-index-1] = operand1;
    }

    /// スタックからstart, valueをpop
    /// startを先頭アドレスしてstart+32までの32byteのメモリ領域にvalueを格納する
    fn op_mstore(&mut self) {
        self.consume_gas(6);
        let address = self.pop().as_u32() as usize;
        let value = self.pop();
        let bytes: [u8; 32] = value.into();
        for (i, b) in bytes.iter().enumerate() {
            self.memory.insert(address+i, *b);
        }
    }

    /// スタックからpopしたstartを先頭アドレスしてstart+32までの32byteの値をメモリからロード
    /// ロードした値をstackの先頭にpush
    fn op_mload(&mut self) {
        self.consume_gas(3);
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

    /// スタックからpopした値をstartとしてinputのstartの位置からstart+32の位置までの32byteのデータをstackにpush
    fn op_calldataload(&mut self) {
        self.consume_gas(3);
        let start = self.pop().as_u32() as usize;
        let bytes: [u8; 32] = slice_to_array(&self.env.input[start..]);
        self.push(bytes.into());
    }

    /// inputに格納されたデータサイズをstackにpush
    fn op_calldatasize(&mut self) {
        self.consume_gas(2);
        let size = self.env.input.len();
        self.push(size.into());
    }

    /// 動的ジャンプを行う際にスタックからpopした値が示すアドレスにジャンプするが、そのアドレスではこのop_jumpdestがオペコードでなければならない
    /// このオペコードはそのマーカーとなるだけで単体では意味を持たない
    fn op_jumpdest(&mut self) {
        self.consume_gas(1);
    }

    /// スタックからdestinationをpopしてジャンプ
    fn op_jump(&mut self) {
        self.consume_gas(8);
        let destination = self.pop().as_u32() as usize;
        
        // ジャンプ先のアドレスのオペコードはJUMPDESTでなければならない
        if self.env.code[destination] != 0x5b {
            panic!("op_jump: destination must be JUMPDEST");
        }

        self.pc = destination + 1; // TODO: +1が必要か調査する
    }

    /// スタックからdestination, conditionをpop
    /// conditionが0以外ならdestinationにジャンプ
    fn op_jumpi(&mut self) {
        self.consume_gas(10);
        let destination = self.pop().as_u32() as usize;
        let condition = self.pop().as_u32() as usize;
        
        // ジャンプ先のアドレスのオペコードはJUMPDESTでなければならない
        if self.env.code[destination] != 0x5b {
            panic!("op_jumpi: destination must be JUMPDEST");
        }

        // conditionか0ならジャンプする
        if condition != 0 {
            self.pc = destination + 1; // TODO: +1が必要か調査する
        }
    }
}

fn str_to_bytes(src: &str) -> Vec<u8> {
    let bytes = hex::decode(src).expect("str_to_bytes: decoding failed");
    return bytes;
}

fn slice_to_array(s: &[u8]) -> [u8; 32] {
    let mut result = [0; 32];
    if s.len() < 32 {
        for (i, b) in s.iter().enumerate() {
            result[i] = *b;
        }
    } else {
        for i in 0..32 {
            result[i] = s[i];
        }
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
fn test_sub() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6004600503"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![1.into()]);
}

#[test]
fn test_div() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6003600604"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999989);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![2.into()]);
}

#[test]
fn test_exp() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("600360020a"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![8.into()]);
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

#[test]
fn test_calldatasize() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("36"));
    env.set_input(str_to_bytes("0000000000000000000000000000000000000000000000000000000000000005"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 1);
    assert_eq!(vm.gas, 9999999998);
    assert_eq!(vm.sp, 1);
    assert_eq!(vm.stack, vec![32.into()]);
}

#[test]
fn test_jumpi() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6000356000525b600160005103600052600051600657"));
    env.set_input(str_to_bytes("0000000000000000000000000000000000000000000000000000000000000005"));
    let mut vm = VM::new(env);
    for _ in 0..14 {
        vm.exec();
    }
    assert_eq!(vm.pc, 21); // jumpi
    vm.exec(); // ここでジャンプ
    assert_eq!(vm.pc, 7);
}

#[test]
fn test_dup1() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005600480"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 2);
    assert_eq!(vm.stack, vec![0x04.into(), 0x04.into()]);
}

#[test]
fn test_swap1() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6005600490"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 5);
    assert_eq!(vm.gas, 9999999991);
    assert_eq!(vm.sp, 2);
    assert_eq!(vm.stack, vec![0x04.into(), 0x05.into()]);
}

#[test]
fn test_loop() {
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("6000355b6001900380600357"));
    env.set_input(str_to_bytes("0000000000000000000000000000000000000000000000000000000000000005"));
    let mut vm = VM::new(env);
    for _ in 0..8 {
        vm.exec();
    }
    assert_eq!(vm.pc, 11); // jumpi
    vm.exec(); // ここでジャンプ
    assert_eq!(vm.pc, 4);
    for _ in 0..5 {
        vm.exec();
    }
    assert_eq!(vm.pc, 11); // jumpi
    vm.exec(); // ここでジャンプ
    assert_eq!(vm.pc, 4);
}

#[test]
fn test_loop2() {
    /*
        0      CALLDATASIZE gas: 10000000000 - 2
        1      PUSH1  => 20
        3      SUB
        4      PUSH2  => 0100
        7      EXP
        8      PUSH1  => 00
        10     CALLDATALOAD
        11     DIV
        12     JUMPDEST gas: 9999999975 - 1
        13     PUSH1  => 01 gas: 9999999974 - 3
        15     SWAP1 gas: 9999999971 - 3
        16     SUB gas: 9999999968 - 3
        17     DUP1 gas: 9999999965 - 3
        18     PUSH1  => 0c gas: 9999999962 - 3
        20     JUMPI gas: 9999999959 - 3
     */
    let mut env = Environment::new(Default::default(), Default::default(), 1, 1);
    env.set_code(str_to_bytes("366020036101000a600035045b6001900380600c57"));
    env.set_input(str_to_bytes("05"));
    let mut vm = VM::new(env);
    vm.exec_transaction();
    assert_eq!(vm.pc, 21);
}