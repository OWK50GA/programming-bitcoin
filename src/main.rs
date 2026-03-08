use programming_bitcoin::ex01::FieldElement;

fn main() {
    // Create a new FieldElement
    let fe1 = FieldElement::new(3, 7);
    let fe2 = FieldElement::new(6, 7);

    // Use the repr method
    println!("Field element 1: {}", fe1.repr());
    println!("Field element 2: {}", fe2.repr());

    // Check equality
    let fe3 = FieldElement::new(3, 7);
    println!("fe1 equals fe3: {}", fe1.equals(&fe3));
    println!("fe1 equals fe2: {}", fe1.equals(&fe2));
    println!("fe1 equals fe2: {}", fe1 == fe3);
    println!("fe1 times fe2: {:?}", fe1.mul(&fe2));
    println!("fe1 raised to power 6: {:?}", fe1.exp(6));
    println!("fe1 divided by fe2: {:?}", fe1.div(&fe2));
    println!("fe1 raised to power 6: {:?}", fe1.exp(-5));
}
