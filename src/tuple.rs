use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;

#[derive(Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Tuple {

    pub fn new(x: f32, y:f32, z:f32, w: f32) -> Tuple {
        Self {
            x,
            y,
            z,
            w
        }
    }

    pub fn new_point(x: f32, y:f32, z:f32) -> Tuple {
        Self {
            x,
            y,
            z,
            w: 1.0
        }
    }

    pub fn new_vector(x: f32, y:f32, z:f32) -> Tuple {
        Self {
            x,
            y,
            z,
            w: 0.0
        }
    }

    pub fn origin() -> Tuple {
        Tuple::new_point(0.0,0.0,0.0)
    }

    pub fn zero_vector() -> Tuple {
        Tuple::new_vector(0.0,0.0,0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.w == 1.0 && rhs.w == 1.0 {
            panic!("Cannot add two points!");
        }
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.w == 0.0 && rhs.w == 1.0 {
            panic!("Cannot subtract a point from a vector!");
        }
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.z - other.z).abs() < f32::EPSILON
            && (self.w - other.w).abs() < f32::EPSILON
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn test_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0 );
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0 );
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    pub fn test_new_point() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    pub fn test_new_vector() {
        let point = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(point.w, 0.0);
    }

    #[test]
    fn test_equals() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0 );
        let b = Tuple::new(4.3, -4.2, 3.1, 0.0 );
        let c = Tuple::new(4.3, -4.2, 3.1, 1.0 );
        assert_eq!(a, b);
        assert_eq!(b, a);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = Tuple::new(3.0,-2.0, 5.0,1.0);
        let b = Tuple::new(-2.0,3.0, 1.0,0.0);
        let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);
        assert_eq!(a + b, expected);
    }

    #[test]
    #[should_panic]
    fn test_add_points() {
        let a = Tuple::new_point(3.0,-2.0, 5.0);
        let b = Tuple::new_point(-2.0,3.0, 1.0);
        let result = a + b;
    }

    #[test]
    fn test_subtract_two_points() {
        let p1 = Tuple::new_point(3.0,2.0,1.0);
        let p2 = Tuple::new_point(5.0,6.0,7.0);
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_subtract_two_vectors() {
        let p1 = Tuple::new_vector(3.0,2.0,1.0);
        let p2 = Tuple::new_vector(5.0,6.0,7.0);
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let p1 = Tuple::new_point(3.0,2.0,1.0);
        let p2 = Tuple::new_vector(5.0,6.0,7.0);
        let expected = Tuple::new_point(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    #[should_panic]
    fn test_point_from_vector() {
        let a = Tuple::new_vector(3.0,-2.0, 5.0);
        let b = Tuple::new_point(-2.0,3.0, 1.0);
        let result = a - b;
    }

    #[test]
    fn test_negate() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(-a, expected);
    }
}