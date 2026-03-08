// Finite Fields

#[derive(Debug)]
pub struct FieldElement {
    order: u8,
    element: u8,
}

impl FieldElement {
    pub fn new(element: u8, order: u8) -> FieldElement {
        assert!(element < order, "Element must be less than order");
        FieldElement { order, element }
    }

    pub fn repr(&self) -> String {
        format!("FieldElement_{}_{}", self.element, self.order)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.element == other.element && self.order == other.order
    }

    pub fn nequals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    pub fn reduce(element: i128, order: u8) -> u8 {
        let mut result: u8 = 0;
        if element < order as i128 && element >= 0 {
            result = element as u8;
        } else if element >= order as i128 {
            result = (element % order as i128) as u8;
        } else if element < 0 {
            result = ((order as i128 + element) % order as i128) as u8;     
        }

        result
    }

    pub fn reduce_neg_exp(exp: i32, order: u8) -> u32 {
        if exp > 0 { return exp as u32 };

        let mut result = 0;
        if exp < 0 {
            let positive_counterpart = exp * -1;

            if positive_counterpart < order as i32 {
                result = order as i32 - (positive_counterpart + 1);
            } else if positive_counterpart == order as i32 {
                result = (2 * (order as i32 - 1)) - order as i32;
            } else {
                let quotient = positive_counterpart / order as i32;

                result = ((quotient + 1) * (order as i32 - 1)) - positive_counterpart;
            }
        }

        result as u32
    }

    pub fn add(&self, other: &Self) -> Self {
        assert!(self.order == other.order, "Cannot add different order felts");

        let mut result = self.element + other.element;
        result = FieldElement::reduce(result as i128, self.order);

        FieldElement { order: self.order, element: result }
    }

    pub fn sub(&self, other: &Self) -> Self {
        assert!(self.order == other.order, "Cannot subtract different order felts");

        let result: i128 = self.element as i128 - other.element as i128;
        let new_result = FieldElement::reduce(result as i128, self.order);

        FieldElement { order: self.order, element: new_result }
    }

    pub fn mul(&self, other: &Self) -> Self {
        assert!(self.order == other.order, "Cannot add different order felts");

        let mut result = self.element * other.element;
        result = FieldElement::reduce(result as i128, self.order);

        FieldElement { order: self.order, element: result }
    }

    pub fn exp(&self, power: i32) -> Self {
        let resolved_power = if power < 0 { FieldElement::reduce_neg_exp(power, self.order) } else { power as u32 };
        let element = self.element as u128;
        let result = element.pow(resolved_power);
        let reduced_result = FieldElement::reduce(result as i128, self.order);
        
        FieldElement { order: self.order, element: reduced_result }
    }

    pub fn div(&self, other: &Self) -> Self {
        assert!(self.order == other.order, "Cannot add different order felts");

        let mult_inv_other: i128 = (other.element as i128).pow(self.order as u32 - 2);
        let result: i128 = (self.element as i128) * mult_inv_other;
        let reduced_result = FieldElement::reduce(result, self.order);

        FieldElement { order: self.order, element: reduced_result }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}