// ============================================================
// CHAPTER 5: TRANSACTIONS - INTEGRATION TESTS
// ============================================================

use programming_bitcoin::{
    decode_varint, encode_varint,
    transaction::Transaction,
    tx_input::{TxId, TxIn},
    tx_output::TxOut,
};

// A real mainnet tx (4 inputs, 2 outputs) used throughout.
// Parsed values (version, locktime, amounts) are verified against known-good data
// so any regression in parse/serialize will be caught immediately.
const REAL_TX_HEX: &str = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

fn parse_real_tx() -> Transaction {
    let bytes = hex::decode(REAL_TX_HEX).unwrap();
    Transaction::parse(&bytes)
}

// ============================================================
// varint helpers
// ============================================================

#[test]
fn test_decode_varint_single_byte() {
    let data = vec![0x05];
    let (val, next) = decode_varint(&data, 0);
    assert_eq!(val, 5);
    assert_eq!(next, 1);
}

#[test]
fn test_decode_varint_fd_prefix() {
    // 0xfd signals a 2-byte (u16 LE) value follows; 0x0001 = 256
    let data = vec![0xfd, 0x00, 0x01];
    let (val, next) = decode_varint(&data, 0);
    assert_eq!(val, 256);
    assert_eq!(next, 3);
}

#[test]
fn test_encode_varint_single_byte() {
    assert_eq!(encode_varint(0xfc), vec![0xfc]);
}

#[test]
fn test_encode_varint_fd_prefix() {
    let encoded = encode_varint(256);
    assert_eq!(encoded[0], 0xfd);
    assert_eq!(encoded.len(), 3);
}

#[test]
fn test_varint_round_trip() {
    // Covers all four varint encoding widths: 1-byte, 0xfd+2, 0xfe+4, 0xff+8
    for n in [0u64, 1, 100, 252, 253, 300, 65535, 65536] {
        let encoded = encode_varint(n);
        let (decoded, _) = decode_varint(&encoded, 0);
        assert_eq!(decoded, n, "round-trip failed for {n}");
    }
}

// ============================================================
// Transaction::parse
// ============================================================

#[test]
fn test_parse_version() {
    let tx = parse_real_tx();
    assert_eq!(tx.version, 1);
}

#[test]
fn test_parse_input_count() {
    let tx = parse_real_tx();
    assert_eq!(tx.inputs.len(), 4);
}

#[test]
fn test_parse_output_count() {
    let tx = parse_real_tx();
    assert_eq!(tx.outputs.len(), 2);
}

#[test]
fn test_parse_locktime() {
    let tx = parse_real_tx();
    assert_eq!(tx.locktime, 410438);
}

#[test]
fn test_parse_first_input_output_index() {
    let tx = parse_real_tx();
    assert_eq!(tx.inputs[0].output_index, 1);
}

#[test]
fn test_parse_first_input_sequence() {
    let tx = parse_real_tx();
    // feffffff = 4294967294
    assert_eq!(tx.inputs[0].sequence, 0xfffffffe);
}

#[test]
fn test_parse_output_amounts() {
    let tx = parse_real_tx();
    assert_eq!(tx.outputs[0].amount, 1000273);
    assert_eq!(tx.outputs[1].amount, 40000000);
}

#[test]
fn test_parse_output_script_pubkeys_non_empty() {
    let tx = parse_real_tx();
    assert!(!tx.outputs[0].script_pubkey.is_empty());
    assert!(!tx.outputs[1].script_pubkey.is_empty());
}

#[test]
fn test_parse_input_script_sig_non_empty() {
    let tx = parse_real_tx();
    // All 4 inputs have a scriptSig
    for input in &tx.inputs {
        assert!(!input.script_sig.is_empty());
    }
}

// ============================================================
// Transaction::serialize (round-trip)
// ============================================================

#[test]
fn test_serialize_round_trip() {
    let tx = parse_real_tx();
    let serialized = tx.serialize();
    assert_eq!(serialized, REAL_TX_HEX);
}

#[test]
fn test_serialize_starts_with_version() {
    let tx = parse_real_tx();
    let hex = tx.serialize();
    // version 1 little-endian = "01000000"
    assert!(hex.starts_with("01000000"));
}

// ============================================================
// TxIn
// ============================================================

#[test]
fn test_txin_repr() {
    let id = [0u8; 32];
    let txin = TxIn::new(id, 0, String::new(), 0xffffffff);
    let repr = txin.repr();
    assert!(repr.contains(':'));
}

#[test]
fn test_txin_serialize_deserialize() {
    let tx = parse_real_tx();
    let original = &tx.inputs[0];
    let bytes = original.serialize();

    // TxIn::parse returns (TxIn, displacement) where displacement == bytes consumed.
    // After a full round-trip the displacement must equal the total serialized length.
    let (parsed, displacement) = TxIn::parse(&bytes, 0);
    assert_eq!(displacement, bytes.len());
    assert_eq!(parsed.output_index, original.output_index);
    assert_eq!(parsed.sequence, original.sequence);
    assert_eq!(parsed.script_sig, original.script_sig);
}

#[test]
fn test_txid_from_hash() {
    let bytes = [1u8; 32];
    let id = TxId::from_hash(bytes);
    assert_eq!(id.0, bytes);
}

#[test]
fn test_txid_from_raw_transaction() {
    let raw = vec![0xde, 0xad, 0xbe, 0xef];
    let id = TxId::from_raw_transaction(raw);
    assert!(id.is_ok());
    assert_eq!(id.unwrap().0.len(), 32);
}

// ============================================================
// TxOut
// ============================================================

#[test]
fn test_txout_new() {
    let out = TxOut::new(50000, "76a914ab".to_string());
    assert_eq!(out.amount, 50000);
    assert_eq!(out.script_pubkey, "76a914ab");
}

#[test]
fn test_txout_serialize_round_trip() {
    let tx = parse_real_tx();
    let original = &tx.outputs[0];
    let bytes = original.serialize();

    let (parsed, _) = TxOut::parse(&bytes, 0);
    assert_eq!(parsed.amount, original.amount);
    assert_eq!(parsed.script_pubkey, original.script_pubkey);
}

#[test]
fn test_txout_repr() {
    let out = TxOut::new(1000, "deadbeef".to_string());
    let repr = out.repr();
    assert!(repr.contains("1000"));
}
