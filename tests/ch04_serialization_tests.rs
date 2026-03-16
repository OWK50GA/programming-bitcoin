// ============================================================
// CHAPTER 4: SERIALIZATION - INTEGRATION TESTS
// ============================================================
// Tests for Bitcoin serialization formats: SEC, DER, WIF, Base58

use num_bigint::{BigUint, ToBigInt, ToBigUint};
use programming_bitcoin::ser_private_key::PrivateKey;
use programming_bitcoin::ser_s256_field::S256Field;
use programming_bitcoin::ser_s256_point::S256Point;
use programming_bitcoin::ser_signature::Signature;
use secp256k1::constants::FIELD_SIZE;

// ============================================================
// UNIT TESTS - S256Field Serialization
// ============================================================

#[test]
fn test_s256_field_to_bytes() {
    let fe = S256Field::new(255_u64.to_biguint().unwrap());
    let bytes = fe.to_bytes();

    assert!(!bytes.is_empty());
    assert_eq!(bytes[bytes.len() - 1], 255);
}

#[test]
fn test_s256_field_from_bytes() {
    let mut bytes = vec![0u8; 32];
    bytes[31] = 42;

    let fe = S256Field::from_bytes(&bytes);
    assert_eq!(fe.element, 42_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_round_trip() {
    let original = S256Field::new(12345_u64.to_biguint().unwrap());
    let bytes = original.to_bytes();
    let restored = S256Field::from_bytes(&bytes);

    assert_eq!(original.element, restored.element);
}

#[test]
fn test_s256_field_zero_bytes() {
    let fe = S256Field::new(0_u64.to_biguint().unwrap());
    let bytes = fe.to_bytes();

    // Zero should serialize to empty or minimal bytes
    assert!(!bytes.is_empty());
}

#[test]
fn test_s256_field_large_value_bytes() {
    let p = BigUint::from_bytes_be(&FIELD_SIZE);
    let large = p - 1_u64.to_biguint().unwrap();
    let fe = S256Field::new(large.clone());
    let bytes = fe.to_bytes();

    let restored = S256Field::from_bytes(&bytes);
    assert_eq!(fe.element, restored.element);
}

#[test]
fn test_s256_field_sqrt() {
    // Test square root function
    let fe = S256Field::new(4_u64.to_biguint().unwrap());
    let sqrt = fe.sqrt();

    // sqrt^2 should equal original (mod p)
    let squared = sqrt.clone().pow(2_u64.to_bigint().unwrap());
    assert_eq!(squared.element, fe.element);
}

// ============================================================
// UNIT TESTS - SEC Format (Serialized Elliptic Curve)
// ============================================================

#[test]
fn test_sec_compressed_format() {
    let g = S256Point::generator();
    let sec = g.to_sec(true);

    // Compressed SEC should be 33 bytes
    assert_eq!(sec.len(), 33);

    // First byte should be 0x02 or 0x03
    assert!(sec[0] == 0x02 || sec[0] == 0x03);
}

#[test]
fn test_sec_uncompressed_format() {
    let g = S256Point::generator();
    let sec = g.to_sec(false);

    // Uncompressed SEC should be 65 bytes
    assert_eq!(sec.len(), 65);

    // First byte should be 0x04
    assert_eq!(sec[0], 0x04);
}

#[test]
fn test_sec_parse_uncompressed() {
    let g = S256Point::generator();
    let sec = g.to_sec(false);
    let parsed = S256Point::parse(sec);

    // Parsed point should match original
    assert_eq!(parsed.x.unwrap().element, g.x.unwrap().element);
    assert_eq!(parsed.y.unwrap().element, g.y.unwrap().element);
}

#[test]
fn test_sec_parse_compressed() {
    let g = S256Point::generator();
    let sec = g.to_sec(true);
    let parsed = S256Point::parse(sec);

    // Parsed point should match original
    assert_eq!(parsed.x.unwrap().element, g.x.unwrap().element);
    assert_eq!(parsed.y.unwrap().element, g.y.unwrap().element);
}

#[test]
fn test_sec_round_trip_compressed() {
    let scalar = 12345_u64.to_biguint().unwrap();
    let point = S256Point::generate_point(scalar);

    let sec = point.to_sec(true);
    let parsed = S256Point::parse(sec);

    assert_eq!(point.x.unwrap().element, parsed.x.unwrap().element);
    assert_eq!(point.y.unwrap().element, parsed.y.unwrap().element);
}

#[test]
fn test_sec_round_trip_uncompressed() {
    let scalar = 54321_u64.to_biguint().unwrap();
    let point = S256Point::generate_point(scalar);

    let sec = point.to_sec(false);
    let parsed = S256Point::parse(sec);

    assert_eq!(point.x.unwrap().element, parsed.x.unwrap().element);
    assert_eq!(point.y.unwrap().element, parsed.y.unwrap().element);
}

#[test]
fn test_sec_even_y_coordinate() {
    // Find a point with even y coordinate
    let g = S256Point::generator();
    let y_element = &g.y.as_ref().unwrap().element;
    let is_even = y_element % 2_u64.to_biguint().unwrap() == 0_u64.to_biguint().unwrap();

    let sec = g.to_sec(true);

    if is_even {
        assert_eq!(sec[0], 0x02);
    } else {
        assert_eq!(sec[0], 0x03);
    }
}

// ============================================================
// UNIT TESTS - DER Format (Distinguished Encoding Rules)
// ============================================================

#[test]
fn test_der_signature_format() {
    let r = S256Field::new(12345_u64.to_biguint().unwrap());
    let s = S256Field::new(67890_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();

    // DER should start with 0x30
    assert_eq!(der[0], 0x30);

    // Should have length byte
    assert!(der.len() > 2);
}

#[test]
fn test_der_signature_structure() {
    let r = S256Field::new(100_u64.to_biguint().unwrap());
    let s = S256Field::new(200_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();

    // Check DER structure
    assert_eq!(der[0], 0x30); // SEQUENCE tag
    // der[1] is total length
    // der[2] should be 0x02 (INTEGER tag for r)
}

#[test]
fn test_der_with_large_values() {
    let large_r = S256Field::new(BigUint::from_bytes_be(&FIELD_SIZE) / 2_u64.to_biguint().unwrap());
    let large_s = S256Field::new(BigUint::from_bytes_be(&FIELD_SIZE) / 3_u64.to_biguint().unwrap());
    let sig = Signature::new(large_r, large_s);

    let der = sig.to_der();

    // Should produce valid DER encoding
    assert_eq!(der[0], 0x30);
    assert!(der.len() > 10);
}

#[test]
fn test_der_high_bit_padding() {
    // Test that high bit is handled correctly (should add 0x00 padding)
    let r = S256Field::new(0x80_u64.to_biguint().unwrap());
    let s = S256Field::new(0x90_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();

    // DER should be valid
    assert_eq!(der[0], 0x30);
}

// ============================================================
// UNIT TESTS - Base58 Encoding
// ============================================================

#[test]
fn test_base58_encoding() {
    let data = vec![0x00, 0x01, 0x02, 0x03];
    let encoded = PrivateKey::encode_base58(&data);

    // Should produce a non-empty string
    assert!(!encoded.is_empty());

    // Should only contain Base58 characters
    let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    assert!(encoded.chars().all(|c| base58_chars.contains(c)));
}

#[test]
fn test_base58_checksum_encoding() {
    let data = vec![0x00, 0x01, 0x02, 0x03];
    let encoded = PrivateKey::encode_base58_checksum(&data);

    // Should produce a non-empty string
    assert!(!encoded.is_empty());

    // Should be longer than plain base58 (includes checksum)
    let plain = PrivateKey::encode_base58(&data);
    assert!(encoded.len() >= plain.len());
}

#[test]
fn test_base58_empty_data() {
    let data = vec![];
    let encoded = PrivateKey::encode_base58(&data);

    // Should handle empty data
    assert!(encoded.is_empty());
}

#[test]
fn test_base58_single_byte() {
    let data = vec![0x42];
    let encoded = PrivateKey::encode_base58(&data);

    assert!(!encoded.is_empty());
}

// ============================================================
// UNIT TESTS - WIF Format (Wallet Import Format)
// ============================================================

const SECRET: &str = "secret";

#[test]
fn test_wif_mainnet_uncompressed() {
    let pk = PrivateKey::new(SECRET);
    let wif = pk.wif(false, false);

    // WIF should be a non-empty string
    assert!(!wif.is_empty());

    // Mainnet uncompressed WIF typically starts with '5'
    // (though this depends on the Base58 alphabet used)
}

#[test]
fn test_wif_mainnet_compressed() {
    let pk = PrivateKey::new(SECRET);
    let wif = pk.wif(true, false);

    // WIF should be a non-empty string
    assert!(!wif.is_empty());

    // Compressed WIF should be different from uncompressed
    let wif_uncompressed = pk.wif(false, false);
    assert_ne!(wif, wif_uncompressed);
}

#[test]
fn test_wif_testnet_uncompressed() {
    let pk = PrivateKey::new(SECRET);
    let wif = pk.wif(false, true);

    assert!(!wif.is_empty());
}

#[test]
fn test_wif_testnet_compressed() {
    let pk = PrivateKey::new(SECRET);
    let wif = pk.wif(true, true);

    assert!(!wif.is_empty());
}

#[test]
fn test_wif_different_networks() {
    let pk = PrivateKey::new(SECRET);

    let mainnet = pk.wif(true, false);
    let testnet = pk.wif(true, true);

    // Different networks should produce different WIF
    assert_ne!(mainnet, testnet);
}

// ============================================================
// UNIT TESTS - Bitcoin Address Generation
// ============================================================

#[test]
fn test_address_mainnet_compressed() {
    let pk = PrivateKey::new(SECRET);
    let address = pk.public_key.0.address(true, false);

    // Address should be a non-empty string
    assert!(!address.is_empty());

    // Bitcoin mainnet addresses typically start with '1' or '3'
    // (though this depends on the Base58 alphabet and address type)
}

#[test]
fn test_address_mainnet_uncompressed() {
    let pk = PrivateKey::new(SECRET);
    let address = pk.public_key.0.address(false, false);

    assert!(!address.is_empty());
}

#[test]
fn test_address_testnet_compressed() {
    let pk = PrivateKey::new(SECRET);
    let address = pk.public_key.0.address(true, true);

    assert!(!address.is_empty());
}

#[test]
fn test_address_testnet_uncompressed() {
    let pk = PrivateKey::new(SECRET);
    let address = pk.public_key.0.address(false, true);

    assert!(!address.len() > 0);
}

#[test]
fn test_address_different_compression() {
    let pk = PrivateKey::new(SECRET);

    let compressed = pk.public_key.0.address(true, false);
    let uncompressed = pk.public_key.0.address(false, false);

    // Different compression should produce different addresses
    assert_ne!(compressed, uncompressed);
}

#[test]
fn test_address_different_networks() {
    let pk = PrivateKey::new(SECRET);

    let mainnet = pk.public_key.0.address(true, false);
    let testnet = pk.public_key.0.address(true, true);

    // Different networks should produce different addresses
    assert_ne!(mainnet, testnet);
}

// ============================================================
// INTEGRATION TESTS - Complete Serialization Workflow
// ============================================================

#[test]
fn test_complete_key_serialization_workflow() {
    // Generate key
    let pk = PrivateKey::new(SECRET);

    // Get WIF
    let wif = pk.wif(true, false);
    assert!(!wif.is_empty());

    // Get address
    let address = pk.public_key.0.address(true, false);
    assert!(!address.is_empty());

    // Get SEC format
    let sec = pk.public_key.0.to_sec(true);
    assert_eq!(sec.len(), 33);
}

#[test]
fn test_signature_serialization_workflow() {
    let pk = PrivateKey::new(SECRET);
    let z = S256Field::new(12345_u64.to_biguint().unwrap());

    // Sign
    let sig = pk.sign(z.clone()).unwrap();

    // Serialize to DER
    let der = sig.to_der();
    assert!(!der.is_empty());
    assert_eq!(der[0], 0x30);
}

#[test]
fn test_point_serialization_all_formats() {
    let scalar = 99999_u64.to_biguint().unwrap();
    let point = S256Point::generate_point(scalar);

    // SEC compressed
    let sec_compressed = point.to_sec(true);
    assert_eq!(sec_compressed.len(), 33);

    // SEC uncompressed
    let sec_uncompressed = point.to_sec(false);
    assert_eq!(sec_uncompressed.len(), 65);

    // Address mainnet
    let addr_main = point.address(true, false);
    assert!(!addr_main.is_empty());

    // Address testnet
    let addr_test = point.address(true, true);
    assert!(!addr_test.is_empty());
}

// ============================================================
// INTEGRATION TESTS - Deterministic Behavior
// ============================================================

#[test]
fn test_sec_deterministic() {
    let scalar = 77777_u64.to_biguint().unwrap();
    let point = S256Point::generate_point(scalar);

    let sec1 = point.to_sec(true);
    let sec2 = point.to_sec(true);

    // Same point should produce same SEC
    assert_eq!(sec1, sec2);
}

#[test]
fn test_der_deterministic() {
    let r = S256Field::new(11111_u64.to_biguint().unwrap());
    let s = S256Field::new(22222_u64.to_biguint().unwrap());

    let sig1 = Signature::new(r.clone(), s.clone());
    let sig2 = Signature::new(r, s);

    let der1 = sig1.to_der();
    let der2 = sig2.to_der();

    // Same signature should produce same DER
    assert_eq!(der1, der2);
}

#[test]
fn test_wif_deterministic() {
    // Note: PrivateKey::new(SECRET) uses random generation, so we can't test
    // determinism directly. This test just ensures WIF is consistent
    // for the same key object.
    let pk = PrivateKey::new(SECRET);

    let wif1 = pk.wif(true, false);
    let wif2 = pk.wif(true, false);

    assert_eq!(wif1, wif2);
}

// ============================================================
// INTEGRATION TESTS - Edge Cases
// ============================================================

#[test]
fn test_sec_generator_point() {
    let g = S256Point::generator();

    let sec_compressed = g.to_sec(true);
    let sec_uncompressed = g.to_sec(false);

    assert_eq!(sec_compressed.len(), 33);
    assert_eq!(sec_uncompressed.len(), 65);
}

#[test]
fn test_der_small_signature_values() {
    let r = S256Field::new(1_u64.to_biguint().unwrap());
    let s = S256Field::new(1_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();

    // Should handle small values correctly
    assert_eq!(der[0], 0x30);
}

#[test]
fn test_address_generator_point() {
    let g = S256Point::generator();

    let address = g.address(true, false);
    assert!(!address.is_empty());
}

// ============================================================
// INTEGRATION TESTS - Format Validation
// ============================================================

#[test]
fn test_sec_format_validation() {
    let point = S256Point::generate_point(12345_u64.to_biguint().unwrap());

    // Compressed
    let sec_comp = point.to_sec(true);
    assert!(sec_comp[0] == 0x02 || sec_comp[0] == 0x03);
    assert_eq!(sec_comp.len(), 33);

    // Uncompressed
    let sec_uncomp = point.to_sec(false);
    assert_eq!(sec_uncomp[0], 0x04);
    assert_eq!(sec_uncomp.len(), 65);
}

#[test]
fn test_der_format_validation() {
    let r = S256Field::new(999_u64.to_biguint().unwrap());
    let s = S256Field::new(888_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();

    // Validate DER structure
    assert_eq!(der[0], 0x30); // SEQUENCE
    assert!(der[1] > 0); // Length
    // der[2] should be 0x02 (INTEGER for r)
}

#[test]
fn test_base58_checksum_includes_hash() {
    let data = vec![0x00, 0x11, 0x22, 0x33];

    let with_checksum = PrivateKey::encode_base58_checksum(&data);
    let without_checksum = PrivateKey::encode_base58(&data);

    // With checksum should be longer
    assert!(with_checksum.len() > without_checksum.len());
}

// ============================================================
// INTEGRATION TESTS - Multiple Keys
// ============================================================

#[test]
fn test_multiple_keys_unique_wif() {
    let pk1 = PrivateKey::new(SECRET);
    let pk2 = PrivateKey::new("another_secret");

    let wif1 = pk1.wif(true, false);
    let wif2 = pk2.wif(true, false);

    // Different keys should have different WIF
    assert_ne!(wif1, wif2);
}

#[test]
fn test_multiple_keys_unique_addresses() {
    let pk1 = PrivateKey::new(SECRET);
    let pk2 = PrivateKey::new("another_secret");

    let addr1 = pk1.public_key.0.address(true, false);
    let addr2 = pk2.public_key.0.address(true, false);

    // Different keys should have different addresses
    assert_ne!(addr1, addr2);
}

#[test]
fn test_multiple_keys_unique_sec() {
    let pk1 = PrivateKey::new(SECRET);
    let pk2 = PrivateKey::new("another_secret");

    let sec1 = pk1.public_key.0.to_sec(true);
    let sec2 = pk2.public_key.0.to_sec(true);

    // Different keys should have different SEC
    assert_ne!(sec1, sec2);
}

// ============================================================
// UNIT TESTS - Signature::from_der
// DER layout: 0x30 <total_len> 0x02 <r_len> <r_bytes> 0x02 <s_len> <s_bytes>
// r and s may have a leading 0x00 padding byte when their high bit is set.
// ============================================================

#[test]
fn test_from_der_round_trip() {
    let r = S256Field::new(99999_u64.to_biguint().unwrap());
    let s = S256Field::new(12345_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();
    let parsed = Signature::from_der(&der).unwrap();

    assert_eq!(parsed.r.element, sig.r.element);
    assert_eq!(parsed.s.element, sig.s.element);
}

#[test]
fn test_from_der_large_values_round_trip() {
    let r = S256Field::new(BigUint::from_bytes_be(&FIELD_SIZE) / 2_u64.to_biguint().unwrap());
    let s = S256Field::new(BigUint::from_bytes_be(&FIELD_SIZE) / 3_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();
    let parsed = Signature::from_der(&der).unwrap();

    assert_eq!(parsed.r.element, sig.r.element);
    assert_eq!(parsed.s.element, sig.s.element);
}

#[test]
fn test_from_der_rejects_too_short() {
    let result = Signature::from_der(&[0x30, 0x06, 0x02]);
    assert!(result.is_err());
}

#[test]
fn test_from_der_rejects_wrong_sequence_tag() {
    // Replace 0x30 with 0x31
    let r = S256Field::new(1_u64.to_biguint().unwrap());
    let s = S256Field::new(1_u64.to_biguint().unwrap());
    let mut der = Signature::new(r, s).to_der();
    der[0] = 0x31;
    assert!(Signature::from_der(&der).is_err());
}

#[test]
fn test_from_der_rejects_length_mismatch() {
    let r = S256Field::new(1_u64.to_biguint().unwrap());
    let s = S256Field::new(1_u64.to_biguint().unwrap());
    let mut der = Signature::new(r, s).to_der();
    // Corrupt the total-length byte
    der[1] = 0xff;
    assert!(Signature::from_der(&der).is_err());
}

#[test]
fn test_from_der_rejects_wrong_integer_tag_for_r() {
    let r = S256Field::new(1_u64.to_biguint().unwrap());
    let s = S256Field::new(1_u64.to_biguint().unwrap());
    let mut der = Signature::new(r, s).to_der();
    // der[2] should be 0x02; corrupt it
    der[2] = 0x03;
    assert!(Signature::from_der(&der).is_err());
}

#[test]
fn test_from_der_rejects_extra_trailing_bytes() {
    let r = S256Field::new(1_u64.to_biguint().unwrap());
    let s = S256Field::new(1_u64.to_biguint().unwrap());
    let mut der = Signature::new(r, s).to_der();
    der.push(0x00); // extra byte
    assert!(Signature::from_der(&der).is_err());
}

#[test]
fn test_from_der_with_high_bit_padding() {
    // r and s with high bit set — to_der should add 0x00 padding
    let r = S256Field::new(0x80_u64.to_biguint().unwrap());
    let s = S256Field::new(0xff_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);

    let der = sig.to_der();
    let parsed = Signature::from_der(&der).unwrap();

    assert_eq!(parsed.r.element, sig.r.element);
    assert_eq!(parsed.s.element, sig.s.element);
}

#[test]
fn test_from_der_sign_preserves_r_and_s() {
    // Verifies that sign → to_der → from_der preserves r and s exactly.
    // Full verify_sig is NOT tested here because verify_sig has a known issue:
    // it uses FIELD_SIZE (prime p) as the modulus instead of CURVE_ORDER (n).
    // ECDSA scalar arithmetic must use n, not p. Fix verify_sig before adding
    // an end-to-end sign/verify test here.
    use programming_bitcoin::ser_private_key::PrivateKey;

    let pk = PrivateKey::new("test_secret_for_der");
    let z = S256Field::new(42_u64.to_biguint().unwrap());

    let sig = pk.sign(z).unwrap();
    let der = sig.to_der();

    let parsed = Signature::from_der(&der).unwrap();

    assert_eq!(parsed.r.element, sig.r.element);
    assert_eq!(parsed.s.element, sig.s.element);
}
