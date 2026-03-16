use std::collections::{HashMap, VecDeque};

use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::{
    script::Cmd, ser_s256_field::S256Field, ser_s256_point::S256Point, ser_signature::Signature,
};

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
            let last = result.last_mut().unwrap();
            *last |= 0x80;
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

    pub fn op_1negate(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(-1));
        true
    }

    pub fn op_1(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(1));
        true
    }
    pub fn op_2(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(2));
        true
    }
    pub fn op_3(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(3));
        true
    }
    pub fn op_4(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(4));
        true
    }
    pub fn op_5(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(5));
        true
    }
    pub fn op_6(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(6));
        true
    }
    pub fn op_7(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(7));
        true
    }
    pub fn op_8(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(8));
        true
    }
    pub fn op_9(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(9));
        true
    }
    pub fn op_10(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(10));
        true
    }
    pub fn op_11(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(11));
        true
    }
    pub fn op_12(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(12));
        true
    }
    pub fn op_13(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(13));
        true
    }
    pub fn op_14(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(14));
        true
    }
    pub fn op_15(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(15));
        true
    }
    pub fn op_16(
        stack: &mut Vec<Element>,
        _: &mut Vec<Element>,
        _: &mut VecDeque<Cmd>,
        _: &[u8],
    ) -> bool {
        stack.push(Self::encode_num(16));
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
        hasher.update(hash1);
        let hash2: Vec<u8> = hasher.finalize().to_vec();

        let new_element = Element(hash2);
        stack.push(new_element);
        true
    }

    pub fn op_checksig(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.len() < 2 {
            return false;
        }

        let signature_bytes = stack.pop().unwrap().0;
        let pubkey_bytes = stack.pop().unwrap().0;

        if signature_bytes.is_empty() {
            stack.push(Self::encode_num(0));
            return true;
        }

        // op_checksig strips the last byte of the signature before DER-parsing it.
        // In real Bitcoin transactions that byte is the sighash type (e.g. 0x01 = SIGHASH_ALL)
        // and is not part of the DER encoding. This is correct behaviour.
        let der_sig = &signature_bytes[0..(signature_bytes.len() - 1)];
        let signature = match Signature::from_der(der_sig) {
            Ok(sig) => sig,
            Err(_) => {
                stack.push(Self::encode_num(0));
                return true;
            }
        };

        let pubkey = S256Point::pubkey_from_ser(pubkey_bytes);

        let message = S256Field::from_bytes(_z);
        // let signature = Signature::from_der(&signature_bytes).unwrap();

        let is_valid = matches!(pubkey.verify_sig(message, signature), Ok(true));

        let num = if is_valid { 1 } else { 0 };

        stack.push(Self::encode_num(num));

        true
    }

    pub fn op_mul(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.len() < 2 {
            return false;
        }

        let top = Self::decode_num(stack.pop().unwrap());
        let next = Self::decode_num(stack.pop().unwrap());

        let product = top * next;

        stack.push(Self::encode_num(product));

        true
    }

    pub fn op_add(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.len() < 2 {
            return false;
        }

        let top = Self::decode_num(stack.pop().unwrap());
        let next = Self::decode_num(stack.pop().unwrap());

        let sum = top + next;

        stack.push(Self::encode_num(sum));

        true
    }

    pub fn op_equal(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.len() < 2 {
            return false;
        }

        let top = stack.pop().unwrap();
        let next = stack.pop().unwrap();

        let result = if top.0 == next.0 { 1 } else { 0 };

        stack.push(Self::encode_num(result));

        true
    }

    pub fn op_verify(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if stack.is_empty() {
            return false;
        }

        let element = stack.pop().unwrap();
        let value = Self::decode_num(element);

        if value == 0 {
            return false;
        }

        true
    }

    pub fn op_equal_verify(
        stack: &mut Vec<Element>,
        _altstack: &mut Vec<Element>,
        _cmds: &mut VecDeque<Cmd>,
        _z: &[u8],
    ) -> bool {
        if !Self::op_equal(stack, _altstack, _cmds, _z) {
            return false;
        }

        // let res = stack.pop().unwrap();
        // Self::decode_num(res) != 0
        Self::op_verify(stack, _altstack, _cmds, _z)
    }
}

pub fn opcode_functions() -> HashMap<u8, OpFn> {
    let mut map = HashMap::new();

    map.insert(0, OpCodes::op_0 as OpFn);
    map.insert(79, OpCodes::op_1negate as OpFn);
    map.insert(81, OpCodes::op_1 as OpFn);
    map.insert(82, OpCodes::op_2 as OpFn);
    map.insert(83, OpCodes::op_3 as OpFn);
    map.insert(84, OpCodes::op_4 as OpFn);
    map.insert(85, OpCodes::op_5 as OpFn);
    map.insert(86, OpCodes::op_6 as OpFn);
    map.insert(87, OpCodes::op_7 as OpFn);
    map.insert(88, OpCodes::op_8 as OpFn);
    map.insert(89, OpCodes::op_9 as OpFn);
    map.insert(90, OpCodes::op_10 as OpFn);
    map.insert(91, OpCodes::op_11 as OpFn);
    map.insert(92, OpCodes::op_12 as OpFn);
    map.insert(93, OpCodes::op_13 as OpFn);
    map.insert(94, OpCodes::op_14 as OpFn);
    map.insert(95, OpCodes::op_15 as OpFn);
    map.insert(96, OpCodes::op_16 as OpFn);
    map.insert(118, OpCodes::op_dup as OpFn);
    map.insert(169, OpCodes::op_hash160 as OpFn);
    map.insert(170, OpCodes::op_hash256 as OpFn);
    map.insert(172, OpCodes::op_checksig as OpFn);
    map.insert(149, OpCodes::op_mul as OpFn);
    map.insert(147, OpCodes::op_add as OpFn);
    map.insert(135, OpCodes::op_equal as OpFn);
    map.insert(105, OpCodes::op_verify as OpFn);
    map.insert(136, OpCodes::op_equal_verify as OpFn);

    map
}
