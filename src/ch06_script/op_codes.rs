use std::collections::{HashMap, VecDeque};

use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::script::Cmd;

/*
stack
altstack
remaining cmds
z (signature hash)
*/
type OpFn = fn(&mut Vec<Element>, &mut Vec<Element>, &mut VecDeque<Cmd>, &[u8]) -> bool;

#[derive(Debug, Clone)]
pub struct OpCodes {
    // pub element: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Element(pub Vec<u8>);

impl OpCodes {
    pub fn encode_num(num: i64) -> Element {
        if num == 0 {
            return Element(vec![]);
        }

        let mut absolute_num = num.abs();
        let is_negative = num < 0;

        let mut result = Vec::new();

        while absolute_num > 0 {
            result.push((absolute_num & 0xff) as u8);
            absolute_num >>= 8
        }

        if (result[result.len() - 1] & 0x80) > 0 {
            if is_negative {
                result.push(0x80);
            } else {
                result.push(0);
            }
        } else if is_negative {
            result.pop();
            result.push(0x80);
        }

        Element(result)
    }

    pub fn decode_num(element: Element) -> i64 {
        if element.0.is_empty() {
            return 0;
        }

        let mut result: i64;
        let is_negative;

        let mut cloned = element.0.clone();
        cloned.reverse();

        if cloned[0] & 0x80 != 0 {
            is_negative = true;
            result = (cloned[0] & 0x7f) as i64;
        } else {
            is_negative = false;
            result = cloned[0] as i64;
        }

        for c in &cloned[1..] {
            result <<= 8;
            result += *c as i64;
        }

        if is_negative { -result } else { result }
    }

    pub fn op_0(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(0));
        true
    }

    pub fn op_dup(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.is_empty() {
            return false;
        }

        let top = stack.pop().unwrap();
        stack.push(top.clone());
        stack.push(top);
        true
    }

    pub fn op_hash256(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.is_empty() {
            return false;
        }

        let element = stack.pop().unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&element.0);
        let hash1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let hash2: Vec<u8> = hasher.finalize().to_vec();

        let new_element = Element(hash2);
        stack.push(new_element);
        true
    }

    pub fn op_hash160(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
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

        let new_element = Element(hash2);
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
