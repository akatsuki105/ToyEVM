extern crate ethereum_types;
extern crate serde;
extern crate serde_json;
extern crate sha3;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use super::util;
use sha3::{Digest, Sha3_256};

use ethereum_types::{H160, U256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldState {
    addresses: HashMap<H160, AccountState>,
    hash: String,
}

impl WorldState {
    pub fn new(config: &str) -> Self {
        // configファイルを読み込む
        let mut config_file = File::open(config).expect("config file not found");
        let mut config_json = String::new();
        config_file.read_to_string(&mut config_json).expect("something went wrong reading the file");

        // 構造体にパース
        let mut ws: WorldState = serde_json::from_str(&config_json).unwrap();

        // worldstateをアカウントから計算する
        let mut account_hashs = Vec::with_capacity(ws.addresses.len());
        for (_, val) in &ws.addresses {
            account_hashs.push(val.calc_hash());
        }
        ws.hash = ws.calc_hash(account_hashs);
        return ws;
    }

    /// 計算方法がよくわからない とりあえず連結してhashとる
    fn calc_hash(&self, account_hashs: Vec<String>) -> String {
        let hash = util::str_to_bytes(&account_hashs.join(""));
        let result = calc_hash(&hash);
        return result;
    }

    pub fn get_hash(&self) -> String {
        return self.hash.clone();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    nonce: usize,
    balance: U256,
    storage: HashMap<U256, U256>,
    code: String,
}

impl AccountState {
    pub fn new(code: String) -> Self {
        Self {
            nonce: 0,
            balance: Default::default(),
            storage: Default::default(),
            code,
        }
    }

    /// getter for balance
    pub fn get_balance(&self) -> U256 {
        self.balance
    }

    /// setter for balance
    pub fn set_balance(&mut self, balance: U256) {
        self.balance = balance;
    }

    /// getter for storage
    pub fn get_storage(&self, key: &U256) -> Option<&U256> {
        let value = self.storage.get(key);
        return value;
    }

    /// setter for storage
    pub fn set_storage(&mut self, key: U256, value: U256) {
        self.storage.insert(key, value);
    }

    /// getter for code
    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    fn calc_hash(&self) -> String {
        return calc_hash(&util::str_to_bytes(&self.code));
    }
}

fn calc_hash(bytes: &Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(&bytes); // write input message
    let result = hasher.result(); // read hash digest
    let hash = result.as_slice();
    return util::bytes_to_str(hash.into());
}