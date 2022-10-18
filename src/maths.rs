use std::f32::EPSILON;

pub struct Coordinate {
    value: f32
}

impl Coordinate {

    pub fn new(value: f32) -> Coordinate {
        Self {
            value
        }
    }

    pub fn zero() -> Coordinate {
        Self {
            value: 0.0
        }
    }

    pub fn eq(&self, other: &Coordinate) -> bool {
        (self.value - other.value) <= f32::EPSILON
    }

    pub fn add(&self, other: &Coordinate) -> f32 {
        self.value + other.value
    }

    pub fn sub(&self, other: &Coordinate) -> f32 {
        self.value - other.value
    }
}

pub struct Tuple {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
    w: f32
}

impl Tuple {

    pub fn new(x: f32, y:f32, z:f32, w: f32) -> Tuple {
        Self {
            x: Coordinate::new(x),
            y: Coordinate::new(y),
            z: Coordinate::new(z),
            w
        }
    }

    pub fn zero() -> Tuple {
        Self {
            x: Coordinate::new(0.0),
            y: Coordinate::new(0.0),
            z: Coordinate::new(0.0),
            w: 1.0
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maths::Tuple;

    #[test]
    fn test_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0 );
        assert_eq!(a.x.value, 4.3);
        assert_eq!(a.y.value, -4.2);
        assert_eq!(a.z.value, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0 );
        assert_eq!(a.x.value, 4.3);
        assert_eq!(a.y.value, -4.2);
        assert_eq!(a.z.value, 3.1);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }
}