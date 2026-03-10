use crate::{ch01::ex01::FieldElement, ex01::ToFieldElement};
use std::{
    io::{Error, ErrorKind},
    ops::Add,
};
// use crate::ch02::ex02::Point;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>, // Option because of the point at infinity
}

impl Add for Point {
    type Output = Result<Self, Error>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Points are not on the same curve",
            ));
        }

        if self.x.is_none() {
            return Ok(rhs);
        }
        if rhs.x.is_none() {
            return Ok(self);
        }

        // let mut slope = 0_u64.to_felt(self.a.order);
        let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
        let (x2, y2) = (rhs.x.unwrap(), rhs.y.unwrap());

        // If x1 == x2 and y1 != y2 => P + (-P) = O
        let slope = if x1 == x2 {
            // If y1 != y2 (i.e. y1 == -y2 mod p) -> P + (-P) = O
            if y1 != y2 {
                return Ok(Point::infinity(self.a, self.b));
            }

            // Now y1 == y2 -> could be doubling. If y1 == 0 => tangent vertical => O
            let zero = 0_u64.to_felt(self.a.order);
            if y1 == zero {
                return Ok(Point::infinity(self.a, self.b));
            }

            // Doubling with non-zero y: slope = (3*x1^2 + a) / (2*y1)
            let x_squared = x1.pow(2);
            let numerator = x_squared * 3_u64.to_felt(self.a.order) + self.a;
            let denominator = y1 * 2_u64.to_felt(self.a.order);

            // denominator should not be zero here, but double-check to avoid panic
            if denominator.element == 0 {
                return Ok(Point::infinity(self.a, self.b));
            }
            numerator / denominator
        } else {
            // General addition case: slope = (y2 - y1) / (x2 - x1)
            let change_y = y2 - y1;
            let change_x = x2 - x1;

            // if change_x == 0 we are in x1 == x2 branch above, so here we expect non-zero
            if change_x.element == 0 {
                return Ok(Point::infinity(self.a, self.b));
            }
            change_y / change_x
        };

        let x3 = slope.pow(2) - self.x.unwrap() - rhs.x.unwrap();

        // if self.eq(rhs) {
        //     x3 = slope.pow(2) - (2_u64.to_felt(self.a.order) * self.x.unwrap());
        // }
        let y3 = slope * (self.x.unwrap() - x3) - self.y.unwrap();

        Ok(Point {
            a: self.a,
            b: self.b,
            x: Some(x3),
            y: Some(y3),
        })
    }
}

impl Point {
    pub fn new(
        a: FieldElement,
        b: FieldElement,
        x: Option<FieldElement>,
        y: Option<FieldElement>,
    ) -> Result<Point, Error> {
        if x.is_none() && y.is_none() {
            return Ok(Point {
                a,
                b,
                x: None,
                y: None,
            });
        }

        if x.is_none() || y.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "x and y must be both some or both none",
            ));
        }

        let x_unwrapped = x.unwrap();
        let y_unwrapped = y.unwrap();

        let y_squared = y_unwrapped.pow(2);
        let x_cubed = x_unwrapped.pow(3);
        let a_x = a * x_unwrapped;
        let right_side = x_cubed + a_x + b;

        if y_squared != right_side {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Point ({:?}, {:?}) does not satisfy y^2 = x^3 + {:?}*x + {:?}",
                    x, y, a, b
                ),
            ));
        }

        Ok(Point {
            a,
            b,
            x: Some(x_unwrapped),
            y: Some(y_unwrapped),
        })
    }

    pub fn infinity(a: FieldElement, b: FieldElement) -> Point {
        Point {
            a,
            b,
            x: None,
            y: None,
        }
    }

    pub fn eq(&self, other: Self) -> bool {
        if self.a == other.a && self.b == other.b {
            return false;
        }

        match (self.x, self.y, other.x, other.y) {
            (None, None, None, None) => true, // both infinity
            (Some(sx), Some(sy), Some(ox), Some(oy)) => sx == ox && sy == oy,
            _ => false, // one is infinity, the other isn't
        }
    }

    pub fn neq(&self, other: Self) -> bool {
        !self.eq(other)
    }

    pub fn is_valid_point(point: Self) -> Result<bool, Error> {
        if point.x.is_none() && point.y.is_none() {
            return Ok(true);
        }
        let y = point.y.unwrap();
        let x = point.x.unwrap();

        let y_squared = y.pow(2);
        let x_cubed = x.pow(3);
        let a_x = point.a * x;

        let right_side = x_cubed + a_x + point.b;

        Ok(y_squared == right_side)
    }

    pub fn scalar_mult(&self, scalar: u64) -> Self {
        let mut coef = scalar;
        let mut current = self.clone();
        let mut result = Self::infinity(self.a, self.b);

        while coef > 0 {
            if (coef & 1) == 1 {
                result = (result + current).unwrap();
            }
            current = (current + current).unwrap();
            coef >>= 1;
        }

        result
    }
}

pub fn test_point() {
    let order = 223;
    let a = FieldElement::new(0, order);
    let b = FieldElement::new(7, order);
    let x1 = FieldElement::new(192, order);
    let y1 = FieldElement::new(105, order);
    let x2 = FieldElement::new(17, order);
    let y2 = FieldElement::new(56, order);
    let x3 = FieldElement::new(15, order);
    let y3 = FieldElement::new(86, order);

    let point1 = Point::new(a, b, Some(x1), Some(y1)).unwrap();
    let point2 = Point::new(a, b, Some(x2), Some(y2)).unwrap();
    println!("{:?}", point1);
    println!("{:?}", point2);

    let point3 = point1 + point2;
    println!("{:?}", point3);

    let point4 = Point::new(a, b, Some(x3), Some(y3)).unwrap();
    println!("{:?}", point4);

    let point4_scaled = point4.scalar_mult(7);
    println!("{:?}", point4_scaled);
}
