use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub a: u64,
    pub b: u64,
    pub x: Option<u64>,
    pub y: Option<u64>, // Option because of the point at infinity
}

impl Point {
    pub fn new(a: u64, b: u64, x: u64, y: u64) -> Result<Point, Error> {
        // Check the elliptic curve equation: y^2 = x^3 + a*x + b
        let y_squared = y
            .checked_mul(y)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "y^2 would overflow"))?;

        let x_cubed = x
            .checked_mul(x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^2 would overflow"))?
            .checked_mul(x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^3 would overflow"))?;

        let a_x = a
            .checked_mul(x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "a*x would overflow"))?;

        let right_side = x_cubed
            .checked_add(a_x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^3 + a*x would overflow"))?
            .checked_add(b)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^3 + a*x + b would overflow"))?;

        if y_squared != right_side {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Point ({}, {}) does not satisfy y^2 = x^3 + {}*x + {}",
                    x, y, a, b
                ),
            ));
        }

        Ok(Point {
            a,
            b,
            x: Some(x),
            y: Some(y),
        })
    }

    pub fn eq(&self, other: Self) -> bool {
        self.a == other.a
            && self.b == other.b
            && self.x.unwrap() == other.x.unwrap()
            && self.y.unwrap() == other.y.unwrap()
    }

    pub fn neq(&self, other: Self) -> bool {
        !self.eq(other)
    }

    pub fn is_valid_point(point: Self) -> Result<bool, Error> {
        let y = point.y.unwrap();
        let x = point.x.unwrap();
        let y_squared = y
            .checked_mul(y)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "y^2 would overflow"))?;

        let x_cubed = x
            .checked_mul(x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^2 would overflow"))?
            .checked_mul(x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^3 would overflow"))?;

        let a_x = point
            .a
            .checked_mul(x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "a*x would overflow"))?;

        let right_side = x_cubed
            .checked_add(a_x)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^3 + a*x would overflow"))?
            .checked_add(point.b)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "x^3 + a*x + b would overflow"))?;

        Ok(y_squared == right_side)
    }

    pub fn add(&self, other: Self) -> Result<Point, Error> {
        if self.a != other.a || self.b != other.b {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Points are not on the same curve",
            ));
        }

        if self.x.is_none() {
            return Ok(other);
        }
        if other.x.is_none() {
            return Ok(Point {
                a: self.a,
                b: self.b,
                x: self.x,
                y: self.y,
            });
        }

        if self.x.unwrap() == other.x.unwrap() {
            return Ok(Point {
                a: self.a,
                b: self.b,
                x: None,
                y: None,
            });
        }

        if self.eq(other) && self.y.unwrap() == 0 {
            return Ok(Point {
                a: self.a,
                b: self.b,
                x: None,
                y: None,
            });
        }

        let slope = if self.eq(other) {
            let x_squared = self.x.unwrap().checked_mul(self.x.unwrap()).unwrap();
            let numerator = x_squared.checked_mul(3).unwrap() + self.a;
            let denominator = self.y.unwrap().checked_mul(2).unwrap();

            numerator / denominator
        } else {
            let change_y = other.y.unwrap() - self.y.unwrap();
            let change_x = other.x.unwrap() - self.x.unwrap();

            change_y / change_x
        };

        let mut x3 = slope.checked_pow(2).unwrap() - self.x.unwrap() - other.x.unwrap();

        if self.eq(other) {
            x3 = slope.checked_pow(2).unwrap() - (2 * self.x.unwrap());
        }
        let y3 = slope * (self.x.unwrap() - x3) - self.y.unwrap();

        Ok(Point {
            a: self.a,
            b: self.b,
            x: Some(x3),
            y: Some(y3),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_point() {
        // For curve y^2 = x^3 + 2*x + 3, point (1, 2) should work:
        // 2^2 = 4, 1^3 + 2*1 + 3 = 1 + 2 + 3 = 6, 4 != 6, so let's use a valid point
        // Let's use (0, 3): 3^2 = 9, 0^3 + 2*0 + 3 = 3, 9 != 3
        // (1, 0): 0^2 = 0, 1^3 + 2*1 + 3 = 1 + 2 + 3 = 6, 0 != 6
        // For y^2 = x^3 + x + 1, point (0, 1): 1^2 = 1, 0 + 0 + 1 = 1 ✓
        let point = Point::new(1, 1, 0, 1).unwrap();
        assert_eq!(point.x.unwrap(), 0);
        assert_eq!(point.y.unwrap(), 1);
    }

    #[test]
    fn test_invalid_point() {
        // Point (1, 2) on curve y^2 = x^3 + x + 1:
        // 2^2 = 4, 1^3 + 1*1 + 1 = 1 + 1 + 1 = 3, 4 != 3
        let result = Point::new(1, 1, 1, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_point_true() {
        let p = Point {
            a: 1,
            b: 1,
            x: Some(0),
            y: Some(1),
        };
        assert_eq!(Point::is_valid_point(p).unwrap(), true);
    }

    #[test]
    fn test_is_valid_point_false() {
        let p = Point {
            a: 1,
            b: 1,
            x: Some(1),
            y: Some(2),
        };
        assert_eq!(Point::is_valid_point(p).unwrap(), false);
    }
}
