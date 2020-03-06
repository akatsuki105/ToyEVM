extern crate ethereum_types;
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use super::util;

use ethereum_types::{H160, H256, U256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldState {
    addresses: HashMap<H160, AccountState>,
}

impl WorldState {
    pub fn new(config: &str) -> Self {
        // configファイルを読み込む
        let mut config_file = File::open(config).expect("config file not found");
        let mut config_json = String::new();
        config_file.read_to_string(&mut config_json).expect("something went wrong reading the file");

        // 構造体にパース
        let ws: WorldState = serde_json::from_str(&config_json).unwrap();
        return ws;
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

    fn calc_hash(&self) -> H256 {
        let hash = H256::from_slice(&util::str_to_bytes(&self.code)[..]);
        return hash;
    }
}