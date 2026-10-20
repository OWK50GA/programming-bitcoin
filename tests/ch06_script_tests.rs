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

#[test]
fn test_script_boolean_encodings() {
    for bytes in [vec![], vec![0], vec![0, 0], vec![0x80], vec![0, 0x80]] {
        let script = Script::new(vec![Cmd::Data(bytes.clone())]);
        assert!(!script.evaluate(&[]).unwrap(), "false encoding: {bytes:?}");
        let verify = script + Script::new(vec![Cmd::OpCode(0x69), Cmd::OpCode(0x51)]);
        assert!(!verify.evaluate(&[]).unwrap());
    }
    for bytes in [vec![1], vec![0x80, 0], vec![0, 1], vec![0xff; 16]] {
        let script = Script::new(vec![Cmd::Data(bytes.clone())]);
        assert!(script.evaluate(&[]).unwrap(), "true encoding: {bytes:?}");
        let verify = script + Script::new(vec![Cmd::OpCode(0x69), Cmd::OpCode(0x51)]);
        assert!(verify.evaluate(&[]).unwrap());
    }
}

#[test]
fn test_parse_rejects_truncated_scripts() {
    let cases = [
        vec![],
        vec![0xfd],
        vec![0xfd, 1],
        vec![0xfe, 0, 0, 0],
        vec![0xff, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 0x51],
        vec![1, 2],
        vec![2, 2, 0xaa],
        vec![1, 76],
        vec![2, 76, 1],
        vec![1, 77],
        vec![2, 77, 1],
        vec![3, 77, 1, 0],
        vec![0xff, 255, 255, 255, 255, 255, 255, 255, 255],
        // Bytes beyond the declared body must not satisfy a data push.
        vec![1, 1, 0xaa],
        vec![2, 76, 1, 0xaa],
        vec![3, 77, 1, 0, 0xaa],
    ];
    for input in cases {
        assert!(Script::parse(&input).is_err(), "input: {input:?}");
    }
}

#[test]
fn test_parse_pushdata_boundaries() {
    for length in [0usize, 1, 75, 76, 255, 256, 520] {
        let script = Script::new(vec![Cmd::Data(vec![0xaa; length])]);
        let parsed = Script::parse(&script.serialize()).unwrap();
        assert_eq!(parsed.serialize(), script.serialize());
    }
    for prefix in [
        vec![0xfd, 1, 0],
        vec![0xfe, 1, 0, 0, 0],
        vec![0xff, 1, 0, 0, 0, 0, 0, 0, 0],
    ] {
        let mut input = prefix;
        input.extend_from_slice(&[0x51, 0x00]);
        let script = Script::parse(&input).unwrap();
        assert_eq!(script.commands.len(), 1);
        assert!(script.evaluate(&[]).unwrap());
    }
}

#[test]
fn test_checksig_standard_stack_order() {
    use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
    let context = Secp256k1::new();
    let secret = SecretKey::from_byte_array([1; 32]).unwrap();
    let public = PublicKey::from_secret_key(&context, &secret);
    let digest = [2; 32];
    let mut signature = context
        .sign_ecdsa(Message::from_digest(digest), &secret)
        .serialize_der()
        .to_vec();
    signature.push(1); // SIGHASH_ALL
    let script = Script::new(vec![
        Cmd::Data(signature),
        Cmd::Data(public.serialize().to_vec()),
        Cmd::OpCode(0xac),
    ]);
    assert!(script.evaluate(&digest).unwrap());
    assert!(!script.evaluate(&[3; 32]).unwrap());
    let empty_signature = Script::new(vec![
        Cmd::Data(vec![]),
        Cmd::Data(public.serialize().to_vec()),
        Cmd::OpCode(0xac),
    ]);
    assert!(!empty_signature.evaluate(&digest).unwrap());
}
