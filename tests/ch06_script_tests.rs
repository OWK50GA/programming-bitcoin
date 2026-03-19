// ============================================================
// CHAPTER 6: SCRIPT / OPCODES - INTEGRATION TESTS
// ============================================================

use programming_bitcoin::{
    opcodes::{Element, OpCodes},
    parse_opcodes,
    script::{Cmd, Script},
};

// ============================================================
// encode_num / decode_num
// Bitcoin Script uses a custom little-endian signed integer encoding where
// the sign bit is the high bit of the last byte (not two's complement).
// Zero is encoded as an empty byte vector.
// ============================================================

#[test]
fn test_encode_num_zero() {
    let e = OpCodes::encode_num(0);
    assert!(e.0.is_empty());
}

#[test]
fn test_encode_decode_positive() {
    for n in [1i64, 2, 9, 16, 127, 128, 255, 1000] {
        let encoded = OpCodes::encode_num(n);
        let decoded = OpCodes::decode_num(encoded);
        assert_eq!(decoded, n, "round-trip failed for {n}");
    }
}

#[test]
fn test_encode_decode_negative() {
    for n in [-1i64, -2, -127, -128, -1000] {
        let encoded = OpCodes::encode_num(n);
        let decoded = OpCodes::decode_num(encoded);
        assert_eq!(decoded, n, "round-trip failed for {n}");
    }
}

#[test]
fn test_decode_num_zero_element() {
    let e = Element(vec![]);
    assert_eq!(OpCodes::decode_num(e), 0);
}

// ============================================================
// Script::parse via parse_opcodes
// Note: parse_opcodes prepends the varint length prefix before calling
// Script::parse. If you call Script::parse directly, you must include
// the length prefix yourself.
// ============================================================

#[test]
fn test_parse_all_opcodes() {
    // OP_4 OP_5 OP_ADD OP_9 OP_EQUAL
    let script = parse_opcodes(vec![0x54, 0x55, 0x93, 0x59, 0x87]).unwrap();
    assert_eq!(script.commands.len(), 5);
}

#[test]
fn test_parse_opcode_values() {
    let script = parse_opcodes(vec![0x54, 0x55, 0x93, 0x59, 0x87]).unwrap();
    assert!(matches!(script.commands[0], Cmd::OpCode(0x54)));
    assert!(matches!(script.commands[4], Cmd::OpCode(0x87)));
}

#[test]
fn test_parse_data_push() {
    // push 3 bytes of data: 0x03 <aa bb cc>
    let script = parse_opcodes(vec![0x03, 0xaa, 0xbb, 0xcc]).unwrap();
    assert_eq!(script.commands.len(), 1);
    assert!(matches!(&script.commands[0], Cmd::Data(d) if d == &[0xaa, 0xbb, 0xcc]));
}

#[test]
fn test_parse_empty_script() {
    let script = parse_opcodes(vec![]).unwrap();
    assert!(script.commands.is_empty());
}

// ============================================================
// Script::serialize round-trip
// ============================================================

#[test]
fn test_serialize_round_trip() {
    let raw = vec![0x54, 0x55, 0x93, 0x59, 0x87];
    let script = parse_opcodes(raw.clone()).unwrap();
    let serialized = script.serialize();
    // serialized = [length_varint, ...raw]
    assert_eq!(&serialized[1..], raw.as_slice());
}

#[test]
fn test_serialize_length_prefix() {
    let raw = vec![0x76, 0xa9, 0x88, 0xac]; // OP_DUP OP_HASH160 OP_EQUALVERIFY OP_CHECKSIG
    let script = parse_opcodes(raw.clone()).unwrap();
    let serialized = script.serialize();
    assert_eq!(serialized[0], raw.len() as u8);
}

// ============================================================
// Script::evaluate — arithmetic
// ============================================================

fn eval(opcodes: Vec<u8>) -> bool {
    let script = parse_opcodes(opcodes).unwrap();
    script.evaluate(&[0u8]).unwrap()
}

#[test]
fn test_op_add_valid() {
    // OP_4 OP_5 OP_ADD OP_9 OP_EQUAL
    assert!(eval(vec![0x54, 0x55, 0x93, 0x59, 0x87]));
}

#[test]
fn test_op_add_invalid() {
    // OP_4 OP_5 OP_ADD OP_8 OP_EQUAL  (4+5=9 ≠ 8)
    assert!(!eval(vec![0x54, 0x55, 0x93, 0x58, 0x87]));
}

#[test]
fn test_op_mul_valid() {
    // OP_3 OP_4 OP_MUL OP_12 OP_EQUAL  (3*4=12)
    assert!(eval(vec![0x53, 0x54, 0x95, 0x5c, 0x87]));
}

#[test]
fn test_op_mul_invalid() {
    // OP_3 OP_4 OP_MUL OP_9 OP_EQUAL  (3*4=12 ≠ 9)
    assert!(!eval(vec![0x53, 0x54, 0x95, 0x59, 0x87]));
}

#[test]
fn test_op_equal_true() {
    // OP_5 OP_5 OP_EQUAL
    assert!(eval(vec![0x55, 0x55, 0x87]));
}

#[test]
fn test_op_equal_false() {
    // OP_5 OP_4 OP_EQUAL
    assert!(!eval(vec![0x55, 0x54, 0x87]));
}

// ============================================================
// Script::evaluate — stack ops
// ============================================================

#[test]
fn test_op_dup() {
    // OP_1 OP_DUP OP_EQUAL  (dup then compare — should be true)
    assert!(eval(vec![0x51, 0x76, 0x87]));
}

#[test]
fn test_op_verify_passes() {
    // OP_1 OP_VERIFY — leaves empty stack but verify itself returns true
    // After verify the stack is empty → evaluate returns false (empty stack)
    // So we add OP_1 after to leave something on stack
    assert!(eval(vec![0x51, 0x69, 0x51]));
}

#[test]
fn test_op_verify_fails_on_zero() {
    // OP_0 OP_VERIFY — should fail
    assert!(!eval(vec![0x00, 0x69]));
}

#[test]
fn test_op_equal_verify_passes() {
    // OP_5 OP_5 OP_EQUALVERIFY OP_1
    assert!(eval(vec![0x55, 0x55, 0x88, 0x51]));
}

#[test]
fn test_op_equal_verify_fails() {
    // OP_5 OP_4 OP_EQUALVERIFY OP_1
    assert!(!eval(vec![0x55, 0x54, 0x88, 0x51]));
}

// ============================================================
// Script::evaluate — small number push opcodes
// ============================================================

#[test]
fn test_op_1_through_16() {
    // OP_1=0x51 through OP_16=0x60 push the literal integer value onto the stack.
    // The opcode byte encodes the value as (opcode - 80), so OP_4=0x54 pushes 4.
    for n in 1u8..=16 {
        let opcode = 80 + n; // OP_1=81 .. OP_16=96
        // push n, push n, OP_EQUAL
        let valid = eval(vec![opcode, opcode, 0x87]);
        assert!(valid, "OP_{n} self-equal failed");
    }
}

#[test]
fn test_op_1negate() {
    // OP_1NEGATE = 0x4f (79). Push -1, push -1, OP_EQUAL
    assert!(eval(vec![0x4f, 0x4f, 0x87]));
}

// ============================================================
// Script::evaluate — hash ops
// ============================================================

#[test]
fn test_op_hash256_deterministic() {
    // Push data, hash256, push same hash manually, equal
    // We'll just test that hash256 produces a 32-byte result
    let mut stack: Vec<Element> = vec![Element(vec![0xde, 0xad])];
    let mut alt: Vec<Element> = vec![];
    let mut cmds = std::collections::VecDeque::new();
    let ok = OpCodes::op_hash256(&mut stack, &mut alt, &mut cmds, &[]);
    assert!(ok);
    assert_eq!(stack[0].0.len(), 32);
}

#[test]
fn test_op_hash160_produces_20_bytes() {
    let mut stack: Vec<Element> = vec![Element(b"hello".to_vec())];
    let mut alt: Vec<Element> = vec![];
    let mut cmds = std::collections::VecDeque::new();
    let ok = OpCodes::op_hash160(&mut stack, &mut alt, &mut cmds, &[]);
    assert!(ok);
    assert_eq!(stack[0].0.len(), 20);
}

#[test]
fn test_op_hash160_empty_stack_fails() {
    let mut stack: Vec<Element> = vec![];
    let mut alt: Vec<Element> = vec![];
    let mut cmds = std::collections::VecDeque::new();
    let ok = OpCodes::op_hash160(&mut stack, &mut alt, &mut cmds, &[]);
    assert!(!ok);
}

// ============================================================
// Script + operator (concatenation)
// ============================================================

#[test]
fn test_script_add_concatenates() {
    let s1 = Script::new(vec![Cmd::OpCode(0x51)]);
    let s2 = Script::new(vec![Cmd::OpCode(0x51), Cmd::OpCode(0x87)]);
    let combined = s1 + s2;
    assert_eq!(combined.commands.len(), 3);
}
