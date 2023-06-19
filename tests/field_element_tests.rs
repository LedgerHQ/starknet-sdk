use starknet_sdk::types::FieldElement;

#[test]
fn test_addition() {
    let fe1 = FieldElement::ONE;
    let fe2 = FieldElement::ONE;
    let fe3 = &fe1 + &fe2;
    
    assert_eq!(fe3, FieldElement::TWO);
}

#[test]
fn test_multiplication() {
    let fe1 = FieldElement::ONE;
    let fe2 = FieldElement::TWO;
    let fe3 = &fe1 * &fe2;

    assert_eq!(fe3, FieldElement::TWO);
}

#[test]
fn test_subtraction() {
    let fe1 = FieldElement::TWO;
    let fe2 = FieldElement::ONE;
    let fe3 = &fe1 - &fe2;

    assert_eq!(fe3, FieldElement::ONE);
}

#[test]
fn test_modulo() {
    let fe1 = FieldElement::TWO;
    let fe2 = FieldElement::ONE;
    let fe3 = &fe1 % &fe2;

    assert_eq!(fe3, FieldElement::ZERO);
}

#[test]
fn test_chain_operations() {
    let fe1 = FieldElement::TWO;
    let fe2 = FieldElement::ONE;
    let fe3 = &fe1 + &fe2; // result should be 3
    let fe4 = &fe3 * &fe1; // result should be 6
    let fe5 = &fe4 - &fe3; // result should be 3
    let fe6 = &fe5 % &fe2; // result should be 0

    assert_eq!(fe6, FieldElement::ZERO);
}
