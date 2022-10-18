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
}