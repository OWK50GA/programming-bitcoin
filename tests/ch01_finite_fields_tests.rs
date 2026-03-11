// ============================================================
// CHAPTER 1: FINITE FIELDS - INTEGRATION TESTS
// ============================================================
// Tests for finite field arithmetic operations including
// addition, subtraction, multiplication, division, and exponentiation

use programming_bitcoin::field_element::{FieldElement, ToFieldElement};

// ============================================================
// UNIT TESTS - Basic Construction and Properties
// ============================================================

#[test]
fn test_field_element_creation() {
    let fe = FieldElement::new(7, 13);
    assert_eq!(fe.element, 7);
    assert_eq!(fe.order, 13);
}

#[test]
#[should_panic(expected = "Element must be less than order")]
fn test_field_element_invalid_creation() {
    FieldElement::new(13, 13);
}

#[test]
#[should_panic(expected = "Element must be less than order")]
fn test_field_element_element_exceeds_order() {
    FieldElement::new(20, 13);
}

#[test]
fn test_field_element_equality() {
    let fe1 = FieldElement::new(7, 13);
    let fe2 = FieldElement::new(7, 13);
    let fe3 = FieldElement::new(6, 13);

    assert_eq!(fe1, fe2);
    assert_ne!(fe1, fe3);
    assert!(fe1.equals(&fe2));
    assert!(!fe1.equals(&fe3));
}

#[test]
fn test_field_element_inequality() {
    let fe1 = FieldElement::new(7, 13);
    let fe2 = FieldElement::new(6, 13);
    let fe3 = FieldElement::new(7, 13);

    assert!(fe1.nequals(&fe2));
    assert!(!fe1.nequals(&fe3));
}

#[test]
fn test_field_element_comparison() {
    let fe1 = FieldElement::new(7, 13);
    let fe2 = FieldElement::new(6, 13);
    let fe3 = FieldElement::new(8, 13);

    assert!(fe1.geq(&fe2));
    assert!(!fe1.geq(&fe3));
    assert!(fe1.leq(&fe3));
    assert!(!fe1.leq(&fe2));
}

#[test]
fn test_field_element_repr() {
    let fe = FieldElement::new(7, 13);
    assert_eq!(fe.repr(), "FieldElement_7_13");
}

#[test]
fn test_field_element_display() {
    let fe = FieldElement::new(7, 13);
    assert_eq!(format!("{}", fe), "FieldElement_7_13");
}

// ============================================================
// UNIT TESTS - Arithmetic Operations
// ============================================================

#[test]
fn test_addition_no_wrap() {
    let fe1 = FieldElement::new(2, 7);
    let fe2 = FieldElement::new(3, 7);
    let result = fe1 + fe2;

    assert_eq!(result.element, 5);
    assert_eq!(result.order, 7);
}

#[test]
fn test_addition_with_wrap() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(4, 7);
    let result = fe1 + fe2;

    assert_eq!(result.element, 2); // (5 + 4) mod 7 = 2
    assert_eq!(result.order, 7);
}

#[test]
fn test_addition_identity() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(0, 7);
    let result = fe1 + fe2;

    assert_eq!(result.element, 5);
}

#[test]
#[should_panic]
fn test_addition_different_orders() {
    let fe1 = FieldElement::new(2, 7);
    let fe2 = FieldElement::new(3, 11);
    let _ = fe1 + fe2;
}

#[test]
fn test_subtraction_no_wrap() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(2, 7);
    let result = fe1 - fe2;

    assert_eq!(result.element, 3);
    assert_eq!(result.order, 7);
}

#[test]
fn test_subtraction_with_wrap() {
    let fe1 = FieldElement::new(2, 7);
    let fe2 = FieldElement::new(5, 7);
    let result = fe1 - fe2;

    assert_eq!(result.element, 4); // (2 - 5) mod 7 = -3 mod 7 = 4
    assert_eq!(result.order, 7);
}

#[test]
fn test_subtraction_identity() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(0, 7);
    let result = fe1 - fe2;

    assert_eq!(result.element, 5);
}

#[test]
fn test_subtraction_self() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(5, 7);
    let result = fe1 - fe2;

    assert_eq!(result.element, 0);
}

#[test]
#[should_panic]
fn test_subtraction_different_orders() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(2, 11);
    let _ = fe1 - fe2;
}

#[test]
fn test_multiplication_no_wrap() {
    let fe1 = FieldElement::new(2, 7);
    let fe2 = FieldElement::new(3, 7);
    let result = fe1 * fe2;

    assert_eq!(result.element, 6);
    assert_eq!(result.order, 7);
}

#[test]
fn test_multiplication_with_wrap() {
    let fe1 = FieldElement::new(3, 7);
    let fe2 = FieldElement::new(4, 7);
    let result = fe1 * fe2;

    assert_eq!(result.element, 5); // (3 * 4) mod 7 = 12 mod 7 = 5
    assert_eq!(result.order, 7);
}

#[test]
fn test_multiplication_by_zero() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(0, 7);
    let result = fe1 * fe2;

    assert_eq!(result.element, 0);
}

#[test]
fn test_multiplication_by_one() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(1, 7);
    let result = fe1 * fe2;

    assert_eq!(result.element, 5);
}

#[test]
#[should_panic]
fn test_multiplication_different_orders() {
    let fe1 = FieldElement::new(3, 7);
    let fe2 = FieldElement::new(4, 11);
    let _ = fe1 * fe2;
}

// ============================================================
// UNIT TESTS - Modular Inverse and Division
// ============================================================

#[test]
fn test_inverse() {
    let fe = FieldElement::new(3, 7);
    let inv = fe.inv().unwrap();

    assert_eq!(inv.element, 5); // 3 * 5 = 15 ≡ 1 mod 7

    // Verify: fe * inv = 1
    let product = fe * inv;
    assert_eq!(product.element, 1);
}

#[test]
fn test_inverse_of_one() {
    let fe = FieldElement::new(1, 7);
    let inv = fe.inv().unwrap();

    assert_eq!(inv.element, 1);
}

#[test]
fn test_division() {
    let fe1 = FieldElement::new(2, 7);
    let fe2 = FieldElement::new(3, 7);
    let result = fe1 / fe2;

    // 2 / 3 = 2 * 3^-1 = 2 * 5 = 10 ≡ 3 mod 7
    assert_eq!(result.element, 3);
    assert_eq!(result.order, 7);
}

#[test]
fn test_division_by_one() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = FieldElement::new(1, 7);
    let result = fe1 / fe2;

    assert_eq!(result.element, 5);
}

#[test]
#[should_panic]
fn test_division_different_orders() {
    let fe1 = FieldElement::new(2, 7);
    let fe2 = FieldElement::new(3, 11);
    let _ = fe1 / fe2;
}

// ============================================================
// UNIT TESTS - Exponentiation
// ============================================================

#[test]
fn test_pow_positive() {
    let fe = FieldElement::new(3, 7);
    let result = fe.pow(2);

    assert_eq!(result.element, 2); // 3^2 = 9 ≡ 2 mod 7
}

#[test]
fn test_pow_zero() {
    let fe = FieldElement::new(3, 7);
    let result = fe.pow(0);

    assert_eq!(result.element, 1); // Any number^0 = 1
}

#[test]
fn test_pow_one() {
    let fe = FieldElement::new(3, 7);
    let result = fe.pow(1);

    assert_eq!(result.element, 3);
}

#[test]
fn test_pow_negative() {
    let fe = FieldElement::new(3, 7);
    let result = fe.pow(-1);

    assert_eq!(result.element, 5); // 3^-1 ≡ 5 mod 7
}

#[test]
fn test_pow_large_positive() {
    let fe = FieldElement::new(2, 7);
    let result = fe.pow(10);

    // 2^10 = 1024 ≡ 2 mod 7
    assert_eq!(result.element, 2);
}

#[test]
fn test_pow_large_negative() {
    let fe = FieldElement::new(3, 7);
    let result = fe.pow(-5);

    // 3^-5 = (3^-1)^5 = 5^5 mod 7
    let inv = fe.inv().unwrap();
    let expected = inv.pow(5);
    assert_eq!(result.element, expected.element);
}

// ============================================================
// UNIT TESTS - Reduce Function
// ============================================================

#[test]
fn test_reduce_positive_in_range() {
    assert_eq!(FieldElement::reduce(5, 7), 5);
    assert_eq!(FieldElement::reduce(0, 7), 0);
    assert_eq!(FieldElement::reduce(6, 7), 6);
}

#[test]
fn test_reduce_positive_overflow() {
    assert_eq!(FieldElement::reduce(10, 7), 3);
    assert_eq!(FieldElement::reduce(14, 7), 0);
    assert_eq!(FieldElement::reduce(15, 7), 1);
}

#[test]
fn test_reduce_negative() {
    assert_eq!(FieldElement::reduce(-1, 7), 6);
    assert_eq!(FieldElement::reduce(-3, 7), 4);
    assert_eq!(FieldElement::reduce(-7, 7), 0);
}

#[test]
fn test_reduce_negative_multiple() {
    assert_eq!(FieldElement::reduce(-10, 7), 4);
    assert_eq!(FieldElement::reduce(-14, 7), 0);
    assert_eq!(FieldElement::reduce(-15, 7), 6);
}

// ============================================================
// UNIT TESTS - ToFieldElement Trait
// ============================================================

#[test]
fn test_to_felt_u8() {
    let fe = 5_u8.to_felt(7);
    assert_eq!(fe.element, 5);
    assert_eq!(fe.order, 7);
}

#[test]
fn test_to_felt_u16() {
    let fe = 5_u16.to_felt(7);
    assert_eq!(fe.element, 5);
    assert_eq!(fe.order, 7);
}

#[test]
fn test_to_felt_u32() {
    let fe = 5_u32.to_felt(7);
    assert_eq!(fe.element, 5);
    assert_eq!(fe.order, 7);
}

#[test]
fn test_to_felt_u64() {
    let fe = 5_u64.to_felt(7);
    assert_eq!(fe.element, 5);
    assert_eq!(fe.order, 7);
}

#[test]
#[should_panic]
fn test_to_felt_u8_exceeds_order() {
    let _ = 10_u8.to_felt(7);
}

// ============================================================
// INTEGRATION TESTS - Complex Operations
// ============================================================

#[test]
fn test_field_arithmetic_combination() {
    // Test: (a + b) * c - d
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(3, 7);
    let c = FieldElement::new(4, 7);
    let d = FieldElement::new(1, 7);

    let result = (a + b) * c - d;
    // (2 + 3) * 4 - 1 = 5 * 4 - 1 = 20 - 1 = 19 ≡ 5 mod 7
    assert_eq!(result.element, 5);
}

#[test]
fn test_fermat_little_theorem() {
    // For prime p and a not divisible by p: a^(p-1) ≡ 1 mod p
    let fe = FieldElement::new(3, 7);
    let result = fe.pow(6); // 7 - 1 = 6

    assert_eq!(result.element, 1);
}

#[test]
fn test_inverse_via_fermat() {
    // a^-1 ≡ a^(p-2) mod p
    let fe = FieldElement::new(3, 7);
    let inv1 = fe.inv().unwrap();
    let inv2 = fe.pow(5); // 7 - 2 = 5

    assert_eq!(inv1.element, inv2.element);
}

#[test]
fn test_distributive_property() {
    // a * (b + c) = a * b + a * c
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(3, 7);
    let c = FieldElement::new(4, 7);

    let left = a * (b + c);
    let right = a * b + a * c;

    assert_eq!(left.element, right.element);
}

#[test]
fn test_associative_property_addition() {
    // (a + b) + c = a + (b + c)
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(3, 7);
    let c = FieldElement::new(4, 7);

    let left = (a + b) + c;
    let right = a + (b + c);

    assert_eq!(left.element, right.element);
}

#[test]
fn test_associative_property_multiplication() {
    // (a * b) * c = a * (b * c)
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(3, 7);
    let c = FieldElement::new(4, 7);

    let left = (a * b) * c;
    let right = a * (b * c);

    assert_eq!(left.element, right.element);
}

#[test]
fn test_commutative_property_addition() {
    // a + b = b + a
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(5, 7);

    let left = a + b;
    let right = b + a;

    assert_eq!(left.element, right.element);
}

#[test]
fn test_commutative_property_multiplication() {
    // a * b = b * a
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(5, 7);

    let left = a * b;
    let right = b * a;

    assert_eq!(left.element, right.element);
}

#[test]
fn test_division_multiplication_inverse() {
    // a / b = a * b^-1
    let a = FieldElement::new(5, 7);
    let b = FieldElement::new(3, 7);

    let div_result = a / b;
    let mult_result = a * b.inv().unwrap();

    assert_eq!(div_result.element, mult_result.element);
}

#[test]
fn test_power_multiplication() {
    // a^(m+n) = a^m * a^n
    let a = FieldElement::new(3, 7);
    let m = 2;
    let n = 3;

    let left = a.pow(m + n);
    let right = a.pow(m) * a.pow(n);

    assert_eq!(left.element, right.element);
}

#[test]
fn test_power_of_product() {
    // (a * b)^n = a^n * b^n
    let a = FieldElement::new(2, 7);
    let b = FieldElement::new(3, 7);
    let n = 3;

    let left = (a * b).pow(n);
    let right = a.pow(n) * b.pow(n);

    assert_eq!(left.element, right.element);
}

// ============================================================
// INTEGRATION TESTS - Larger Prime Fields
// ============================================================

#[test]
fn test_larger_prime_field_operations() {
    let p = 31;
    let a = FieldElement::new(17, p);
    let b = FieldElement::new(21, p);

    let sum = a + b;
    assert_eq!(sum.element, 7); // (17 + 21) mod 31 = 7

    let product = a * b;
    assert_eq!(product.element, 16); // (17 * 21) mod 31 = 357 mod 31 = 16
}

#[test]
fn test_prime_field_97() {
    let p = 97;
    let a = FieldElement::new(95, p);
    let b = FieldElement::new(45, p);
    let c = FieldElement::new(31, p);

    // (95 + 45) * 31 mod 97
    let result = (a + b) * c;
    // (95 + 45) = 140 ≡ 43 mod 97
    // 43 * 31 = 1333 ≡ 77 mod 97
    assert_eq!(result.element, 72);
}

#[test]
fn test_copy_trait() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = fe1; // Copy, not move
    let fe3 = fe1; // Can still use fe1

    assert_eq!(fe1.element, fe2.element);
    assert_eq!(fe1.element, fe3.element);
}

#[test]
fn test_clone_trait() {
    let fe1 = FieldElement::new(5, 7);
    let fe2 = fe1;

    assert_eq!(fe1.element, fe2.element);
    assert_eq!(fe1.order, fe2.order);
}
