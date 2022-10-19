use std::f32::EPSILON;

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

    pub fn zero() -> Tuple {
        Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.z - other.z).abs() < f32::EPSILON
            && (self.w - other.w).abs() < f32::EPSILON
    }

    pub fn add(&self, other: &Tuple) -> Tuple {
        if self.w == 1.0 && other.w == 0.0 {
            panic!("Cannot add two points!");
        }
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }

    pub fn sub(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maths::Tuple;

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
    fn test_equals() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0 );
        let b = Tuple::new(4.3, -4.2, 3.1, 0.0 );
        let c = Tuple::new(4.3, -4.2, 3.1, 1.0 );
        assert!(a.eq(&b));
        assert!(b.eq(&a));
        assert!(!b.eq(&c));
    }
}