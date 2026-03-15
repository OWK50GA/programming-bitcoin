use std::collections::HashMap;

use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::script::Cmd;

/*
stack
altstack
remaining cmds
z (signature hash)
*/
type OpFn = fn(
    &mut Vec<Elements>,
    &mut Vec<Elements>,
    &mut Vec<Cmd>,
    &[u8],
) -> bool;

#[derive(Debug, Clone)]
pub struct OpCodes {
    pub elements: Vec<Elements>,
}

#[derive(Debug, Clone)]
pub struct Elements(pub Vec<u8>);

impl OpCodes {
    pub fn op_dup(stack: &mut Vec<Elements>, _altstack: &mut Vec<Elements>, _cmds: &mut Vec<Cmd>, _z: &[u8]) -> bool {
        if stack.is_empty() {
            return false;
        }

        let top = stack.pop().unwrap();
        stack.push(top.clone());
        stack.push(top);
        true
    }

    pub fn op_hash256(stack: &mut Vec<Elements>, _altstack: &mut Vec<Elements>, _cmds: &mut Vec<Cmd>, _z: &[u8]) -> bool {
        if stack.is_empty() {
            return false;
        }

        let element = stack.pop().unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&element.0);
        let hash1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(&hash1);
        let hash2: Vec<u8> = hasher.finalize().to_vec();

        let new_element = Elements(hash2);
        stack.push(new_element);
        true
    }

    pub fn op_hash160(stack: &mut Vec<Elements>, _altstack: &mut Vec<Elements>, _cmds: &mut Vec<Cmd>, _z: &[u8]) -> bool {
        if stack.is_empty() {
            return false;
        }

        let element = stack.pop().unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&element.0);
        let hash1 = hasher.finalize();

        let mut hasher = Ripemd160::new();
        hasher.update(&hash1);
        let hash2: Vec<u8> = hasher.finalize().to_vec();

        let new_element = Elements(hash2);
        stack.push(new_element);
        true
    }
}

pub fn opcode_functions() -> HashMap<u8, OpFn> {
    let mut map = HashMap::new();

    map.insert(118, OpCodes::op_dup as OpFn);
    map.insert(170, OpCodes::op_hash256 as OpFn);
    map.insert(169, OpCodes::op_hash160 as OpFn);

    map
}