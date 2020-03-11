//! Utility function
//! 
//! # util.rs
//! 
//! ユーティリティ関数を集めたモジュール

use ethereum_types::{H160};
use std::str::FromStr;

pub fn str_to_bytes(src: &str) -> Vec<u8> {
    let bytes = hex::decode(src).expect("str_to_bytes: decoding failed");
    return bytes;
}

pub fn bytes_to_str(src: Vec<u8>) -> String {
    hex::encode(src)
}

pub fn slice_to_array(s: &[u8]) -> [u8; 32] {
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

pub fn not_implement_panic() {
    panic!("not implement");
}

pub fn to_h160(s: &str) -> H160 {
    return H160::from_str(s).unwrap();
}
