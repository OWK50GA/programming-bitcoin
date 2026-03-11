// ============================================================
// CHAPTER 3: BITCOIN CRYPTOGRAPHY (ECC) - INTEGRATION TESTS
// ============================================================
// Tests for secp256k1 curve operations, signatures, and private keys

use programming_bitcoin::s256_field::{S256Field, ToS256Field};
use programming_bitcoin::s256_point::S256Point;
use programming_bitcoin::signature::Signature;
use programming_bitcoin::private_key::PrivateKey;
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use secp256k1::constants::{FIELD_SIZE, CURVE_ORDER, GENERATOR_X, GENERATOR_Y};

// ============================================================
// UNIT TESTS - S256Field Construction
// ============================================================

#[test]
fn test_s256_field_creation() {
    let fe = S256Field::new(7_u64.to_biguint().unwrap());
    assert_eq!(fe.element, 7_u64.to_biguint().unwrap());
    assert_eq!(fe.order, BigUint::from_bytes_be(&FIELD_SIZE));
}

#[test]
fn test_s256_field_creation_with_reduction() {
    // Element larger than field size should be reduced
    let large = BigUint::from_bytes_be(&FIELD_SIZE) + 10_u64.to_biguint().unwrap();
    let fe = S256Field::new(large);
    assert_eq!(fe.element, 10_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_from_bytes() {
    let bytes = [0u8; 32];
    let fe = S256Field::from_bytes(&bytes);
    assert_eq!(fe.element, 0_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_to_bytes() {
    let fe = S256Field::new(255_u64.to_biguint().unwrap());
    let bytes = fe.to_bytes();
    assert!(bytes.len() > 0);
    assert_eq!(bytes[bytes.len() - 1], 255);
}

#[test]
fn test_s256_field_repr() {
    let fe = S256Field::new(42_u64.to_biguint().unwrap());
    let repr = fe.repr();
    assert!(repr.contains("S256Field"));
    assert!(repr.contains("42"));
}

// ============================================================
// UNIT TESTS - S256Field Arithmetic
// ============================================================

#[test]
fn test_s256_field_addition() {
    let fe1 = S256Field::new(10_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(20_u64.to_biguint().unwrap());
    let result = fe1 + fe2;
    
    assert_eq!(result.element, 30_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_addition_with_wrap() {
    let p = BigUint::from_bytes_be(&FIELD_SIZE);
    let fe1 = S256Field::new(p.clone() - 5_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(10_u64.to_biguint().unwrap());
    let result = fe1 + fe2;
    
    assert_eq!(result.element, 5_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_subtraction() {
    let fe1 = S256Field::new(20_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(10_u64.to_biguint().unwrap());
    let result = fe1 - fe2;
    
    assert_eq!(result.element, 10_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_subtraction_with_wrap() {
    let fe1 = S256Field::new(5_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(10_u64.to_biguint().unwrap());
    let result = fe1 - fe2;
    
    let p = BigUint::from_bytes_be(&FIELD_SIZE);
    assert_eq!(result.element, p - 5_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_multiplication() {
    let fe1 = S256Field::new(10_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(20_u64.to_biguint().unwrap());
    let result = fe1 * fe2;
    
    assert_eq!(result.element, 200_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_division() {
    let fe1 = S256Field::new(20_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(10_u64.to_biguint().unwrap());
    let result = fe1 / fe2;
    
    assert_eq!(result.element, 2_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_inverse() {
    let fe = S256Field::new(3_u64.to_biguint().unwrap());
    let inv = fe.inv().unwrap();
    
    // fe * inv should equal 1
    let product = fe * inv;
    assert_eq!(product.element, 1_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_pow_positive() {
    let fe = S256Field::new(2_u64.to_biguint().unwrap());
    let result = fe.pow(3_u64.to_bigint().unwrap());
    
    assert_eq!(result.element, 8_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_pow_zero() {
    let fe = S256Field::new(5_u64.to_biguint().unwrap());
    let result = fe.pow(0_u64.to_bigint().unwrap());
    
    assert_eq!(result.element, 1_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_pow_negative() {
    let fe = S256Field::new(3_u64.to_biguint().unwrap());
    let result = fe.pow((-1_i64).to_bigint().unwrap());
    
    // Should equal the inverse
    let inv = fe.inv().unwrap();
    assert_eq!(result.element, inv.element);
}

// ============================================================
// UNIT TESTS - S256Field Comparison
// ============================================================

#[test]
fn test_s256_field_equality() {
    let fe1 = S256Field::new(42_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(42_u64.to_biguint().unwrap());
    
    assert!(fe1 == fe2);
    assert!(fe1.equals(&fe2));
}

#[test]
fn test_s256_field_inequality() {
    let fe1 = S256Field::new(42_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(43_u64.to_biguint().unwrap());
    
    assert!(fe1 != fe2);
    assert!(fe1.nequals(&fe2));
}

#[test]
fn test_s256_field_greater_equal() {
    let fe1 = S256Field::new(43_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(42_u64.to_biguint().unwrap());
    
    assert!(fe1.geq(&fe2));
    assert!(!fe2.geq(&fe1));
}

#[test]
fn test_s256_field_less_equal() {
    let fe1 = S256Field::new(42_u64.to_biguint().unwrap());
    let fe2 = S256Field::new(43_u64.to_biguint().unwrap());
    
    assert!(fe1.leq(&fe2));
    assert!(!fe2.leq(&fe1));
}

// ============================================================
// UNIT TESTS - ToS256Field Trait
// ============================================================

#[test]
fn test_to_s256_field_u8() {
    let fe = 42_u8.to_felts256();
    assert_eq!(fe.element, 42_u64.to_biguint().unwrap());
}

#[test]
fn test_to_s256_field_u16() {
    let fe = 1000_u16.to_felts256();
    assert_eq!(fe.element, 1000_u64.to_biguint().unwrap());
}

#[test]
fn test_to_s256_field_u32() {
    let fe = 100000_u32.to_felts256();
    assert_eq!(fe.element, 100000_u64.to_biguint().unwrap());
}

#[test]
fn test_to_s256_field_u64() {
    let fe = 1000000_u64.to_felts256();
    assert_eq!(fe.element, 1000000_u64.to_biguint().unwrap());
}

// ============================================================
// UNIT TESTS - S256Point Construction
// ============================================================

#[test]
fn test_s256_point_infinity() {
    let result = S256Point::new(None, None);
    assert!(result.is_ok());
    
    let point = result.unwrap();
    assert!(point.x.is_none());
    assert!(point.y.is_none());
}

#[test]
fn test_s256_point_invalid_one_coordinate() {
    let x = S256Field::new(5_u64.to_biguint().unwrap());
    let result = S256Point::new(Some(x), None);
    
    assert!(result.is_err());
}

#[test]
fn test_s256_point_generator() {
    let g = S256Point::generator();
    
    // Generator should have valid x and y coordinates
    assert!(g.x.is_some());
    assert!(g.y.is_some());
    
    // Verify generator coordinates match constants
    let gx = BigUint::from_bytes_be(&GENERATOR_X);
    let gy = BigUint::from_bytes_be(&GENERATOR_Y);
    
    assert_eq!(g.x.unwrap().element, gx);
    assert_eq!(g.y.unwrap().element, gy);
}

#[test]
fn test_s256_point_is_valid() {
    let g = S256Point::generator();
    let result = S256Point::is_valid_point(g);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

// ============================================================
// UNIT TESTS - S256Point Arithmetic
// ============================================================

#[test]
fn test_s256_point_add_infinity() {
    let g = S256Point::generator();
    let inf = S256Point::new(None, None).unwrap();
    
    let result = g.clone() + inf;
    assert!(result.is_ok());
    
    let sum = result.unwrap();
    assert_eq!(sum.x.unwrap().element, g.x.unwrap().element);
}

#[test]
fn test_s256_point_scalar_multiplication_by_zero() {
    let g = S256Point::generator();
    let result = g.scalar_mult(0_u64.to_biguint().unwrap());
    
    // G * 0 = O (point at infinity)
    assert!(result.x.is_none());
    assert!(result.y.is_none());
}

#[test]
fn test_s256_point_scalar_multiplication_by_one() {
    let g = S256Point::generator();
    let result = g.scalar_mult(1_u64.to_biguint().unwrap());
    
    // G * 1 = G
    assert_eq!(result.x.unwrap().element, g.x.unwrap().element);
    assert_eq!(result.y.unwrap().element, g.y.unwrap().element);
}

#[test]
fn test_s256_point_scalar_multiplication_by_curve_order() {
    let g = S256Point::generator();
    let n = BigUint::from_bytes_be(&CURVE_ORDER);
    let result = g.scalar_mult(n);
    
    // G * n = O (point at infinity)
    assert!(result.x.is_none());
    assert!(result.y.is_none());
}

#[test]
fn test_s256_point_generate_point() {
    let scalar = 12345_u64.to_biguint().unwrap();
    let point = S256Point::generate_point(scalar);
    
    // Should produce a valid point
    assert!(point.x.is_some());
    assert!(point.y.is_some());
}

// ============================================================
// UNIT TESTS - Signature
// ============================================================

#[test]
fn test_signature_creation() {
    let r = S256Field::new(10_u64.to_biguint().unwrap());
    let s = S256Field::new(20_u64.to_biguint().unwrap());
    
    let sig = Signature::new(r.clone(), s.clone());
    
    assert_eq!(sig.r.element, r.element);
    assert_eq!(sig.s.element, s.element);
}

#[test]
fn test_signature_display() {
    let r = S256Field::new(10_u64.to_biguint().unwrap());
    let s = S256Field::new(20_u64.to_biguint().unwrap());
    let sig = Signature::new(r, s);
    
    let display = format!("{}", sig);
    assert!(display.contains("Signature"));
}

#[test]
fn test_signature_clone() {
    let r = S256Field::new(10_u64.to_biguint().unwrap());
    let s = S256Field::new(20_u64.to_biguint().unwrap());
    let sig1 = Signature::new(r, s);
    let sig2 = sig1.clone();
    
    assert_eq!(sig1.r.element, sig2.r.element);
    assert_eq!(sig1.s.element, sig2.s.element);
}

// ============================================================
// UNIT TESTS - PrivateKey
// ============================================================

#[test]
fn test_private_key_generation() {
    let pk = PrivateKey::new();
    
    // Should have a secret and a point
    assert!(pk.secret_bytes.element > 0_u64.to_biguint().unwrap());
    assert!(pk.point.x.is_some());
    assert!(pk.point.y.is_some());
}

#[test]
fn test_private_key_hex() {
    let pk = PrivateKey::new();
    let hex = pk.hex();
    
    // Should be a valid hex string
    assert!(hex.len() > 0);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_private_key_deterministic_k() {
    let pk = PrivateKey::new();
    let z = S256Field::new(12345_u64.to_biguint().unwrap());
    
    let k1 = pk.deterministic_k(z.clone());
    let k2 = pk.deterministic_k(z);
    
    // Same input should produce same k
    assert_eq!(k1.element, k2.element);
}

#[test]
fn test_private_key_sign() {
    let pk = PrivateKey::new();
    let z = S256Field::new(12345_u64.to_biguint().unwrap());
    
    let result = pk.sign(z);
    assert!(result.is_ok());
    
    let sig = result.unwrap();
    assert!(sig.r.element > 0_u64.to_biguint().unwrap());
    assert!(sig.s.element > 0_u64.to_biguint().unwrap());
}

// ============================================================
// INTEGRATION TESTS - Signature Verification
// ============================================================

#[test]
fn test_sign_and_verify() {
    let pk = PrivateKey::new();
    let z = S256Field::new(12345_u64.to_biguint().unwrap());
    
    let sig = pk.clone().sign(z.clone()).unwrap();
    println!("{:?}", sig);
    let sig2 = pk.clone().sign(z.clone()).unwrap();
    println!("{:?}", sig2);
    
    let public_key = pk.point;
    let result = public_key.verify_sig(z, sig);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_verify_with_wrong_message() {
    let pk = PrivateKey::new();
    let z1 = S256Field::new(12345_u64.to_biguint().unwrap());
    let z2 = S256Field::new(54321_u64.to_biguint().unwrap());
    
    let sig = pk.clone().sign(z1).unwrap();
    let public_key = pk.point;
    
    // Verifying with different message should fail
    let result = public_key.verify_sig(z2, sig);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

// ============================================================
// INTEGRATION TESTS - Field Properties
// ============================================================

#[test]
fn test_s256_field_distributive_property() {
    let a = S256Field::new(2_u64.to_biguint().unwrap());
    let b = S256Field::new(3_u64.to_biguint().unwrap());
    let c = S256Field::new(4_u64.to_biguint().unwrap());
    
    // a * (b + c) = a * b + a * c
    let left = a.clone() * (b.clone() + c.clone());
    let right = a.clone() * b + a * c;
    
    assert_eq!(left.element, right.element);
}

#[test]
fn test_s256_field_associative_addition() {
    let a = S256Field::new(2_u64.to_biguint().unwrap());
    let b = S256Field::new(3_u64.to_biguint().unwrap());
    let c = S256Field::new(4_u64.to_biguint().unwrap());
    
    // (a + b) + c = a + (b + c)
    let left = (a.clone() + b.clone()) + c.clone();
    let right = a + (b + c);
    
    assert_eq!(left.element, right.element);
}

#[test]
fn test_s256_field_associative_multiplication() {
    let a = S256Field::new(2_u64.to_biguint().unwrap());
    let b = S256Field::new(3_u64.to_biguint().unwrap());
    let c = S256Field::new(4_u64.to_biguint().unwrap());
    
    // (a * b) * c = a * (b * c)
    let left = (a.clone() * b.clone()) * c.clone();
    let right = a * (b * c);
    
    assert_eq!(left.element, right.element);
}

#[test]
fn test_s256_field_commutative_addition() {
    let a = S256Field::new(7_u64.to_biguint().unwrap());
    let b = S256Field::new(13_u64.to_biguint().unwrap());
    
    // a + b = b + a
    let left = a.clone() + b.clone();
    let right = b + a;
    
    assert_eq!(left.element, right.element);
}

#[test]
fn test_s256_field_commutative_multiplication() {
    let a = S256Field::new(7_u64.to_biguint().unwrap());
    let b = S256Field::new(13_u64.to_biguint().unwrap());
    
    // a * b = b * a
    let left = a.clone() * b.clone();
    let right = b * a;
    
    assert_eq!(left.element, right.element);
}

// ============================================================
// INTEGRATION TESTS - Point Properties
// ============================================================

#[test]
fn test_point_addition_commutative() {
    let g = S256Point::generator();
    let p1 = g.scalar_mult(5_u64.to_biguint().unwrap());
    let p2 = g.scalar_mult(7_u64.to_biguint().unwrap());
    
    let sum1 = (p1.clone() + p2.clone()).unwrap();
    let sum2 = (p2 + p1).unwrap();
    
    assert_eq!(sum1.x.unwrap().element, sum2.x.unwrap().element);
    assert_eq!(sum1.y.unwrap().element, sum2.y.unwrap().element);
}

#[test]
fn test_scalar_multiplication_distributive() {
    let g = S256Point::generator();
    
    // (a + b) * G = a * G + b * G
    let a = 5_u64.to_biguint().unwrap();
    let b = 7_u64.to_biguint().unwrap();
    
    let left = g.scalar_mult(a.clone() + b.clone());
    let right = (g.scalar_mult(a) + g.scalar_mult(b)).unwrap();
    
    assert_eq!(left.x.unwrap().element, right.x.unwrap().element);
    assert_eq!(left.y.unwrap().element, right.y.unwrap().element);
}

// ============================================================
// INTEGRATION TESTS - Clone and Copy
// ============================================================

#[test]
fn test_s256_field_clone() {
    let fe1 = S256Field::new(42_u64.to_biguint().unwrap());
    let fe2 = fe1.clone();
    
    assert_eq!(fe1.element, fe2.element);
    assert_eq!(fe1.order, fe2.order);
}

#[test]
fn test_s256_point_clone() {
    let g = S256Point::generator();
    let g2 = g.clone();
    
    assert_eq!(g.x.unwrap().element, g2.x.unwrap().element);
    assert_eq!(g.y.unwrap().element, g2.y.unwrap().element);
}

// ============================================================
// INTEGRATION TESTS - Edge Cases
// ============================================================

#[test]
fn test_s256_field_zero() {
    let zero = S256Field::new(0_u64.to_biguint().unwrap());
    let fe = S256Field::new(42_u64.to_biguint().unwrap());
    
    // Adding zero
    let sum = fe.clone() + zero.clone();
    assert_eq!(sum.element, fe.element);
    
    // Multiplying by zero
    let product = fe * zero;
    assert_eq!(product.element, 0_u64.to_biguint().unwrap());
}

#[test]
fn test_s256_field_one() {
    let one = S256Field::new(1_u64.to_biguint().unwrap());
    let fe = S256Field::new(42_u64.to_biguint().unwrap());
    
    // Multiplying by one
    let product = fe.clone() * one.clone();
    assert_eq!(product.element, fe.element);
    
    // Dividing by one
    let quotient = fe / one;
    assert_eq!(quotient.element, 42_u64.to_biguint().unwrap());
}

#[test]
fn test_multiple_private_keys_unique() {
    let pk1 = PrivateKey::new();
    let pk2 = PrivateKey::new();
    
    // Different private keys should have different secrets
    assert_ne!(pk1.secret_bytes.element, pk2.secret_bytes.element);
}
