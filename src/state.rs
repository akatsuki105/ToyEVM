//! Ethereum state
//!
//! Ethereumにおけるステートを表現するモジュール

extern crate ethereum_types;
extern crate serde;
extern crate serde_json;
extern crate sha3;
use super::util;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use ethereum_types::{H160, U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorldState {
    addresses: HashMap<H160, AccountState>,
    hash: String,
}

impl WorldState {
    pub fn new(config: &str) -> Self {
        // configファイルを読み込む
        let mut config_file = File::open(config).expect("config file not found");
        let mut config_json = String::new();
        config_file
            .read_to_string(&mut config_json)
            .expect("something went wrong reading the file");

        // 構造体にパース
        let ws: WorldState = serde_json::from_str(&config_json).unwrap();
        return ws;
    }

    pub fn update_state(&mut self) {
        // worldstateをアカウントから計算する
        let mut account_hashs = Vec::with_capacity(self.addresses.len());
        for (_, val) in &self.addresses {
            account_hashs.push(val.calc_hash());
        }
        self.hash = self.calc_hash(account_hashs);
    }

    pub fn push_account_state(&mut self, address: H160, account_state: AccountState) {
        self.addresses.insert(address, account_state);
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

    pub fn get_account_state(&mut self, address: &H160) -> &mut AccountState {
        let account_state = self
            .addresses
            .get_mut(address)
            .expect("key is not found in storage.");
        return account_state;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    nonce: usize,                 // ナンス
    balance: U256,                // 残高(wei)
    storage: HashMap<U256, U256>, // storage
    code: String,                 // コントラクトコード
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

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
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
    pub fn get_storage(&self, key: &U256) -> &U256 {
        let value = self.storage.get(key).expect("key is not found in storage");
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
        let nonce = self.nonce.to_string();
        let balance = self.balance.to_string();
        let mut storage = "".to_string();
        for (key, val) in &self.storage {
            storage += &(key.to_string() + &val.to_string());
        }
        let code = self.code.clone();
        let hash = nonce + &balance + &storage + &code;
        return calc_hash(&util::str_to_bytes(&hash));
    }
}

fn calc_hash(bytes: &Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(&bytes); // write input message
    let result = hasher.result(); // read hash digest
    let hash = result.as_slice();
    return util::bytes_to_str(hash.into());
}
