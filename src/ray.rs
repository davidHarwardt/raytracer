use cgmath::{Point3, Vector3};
use crate::types::Float;

pub struct Ray {
    pub origin: Point3<Float>,
    pub direction: Vector3<Float>,
}

impl Ray {
    pub fn at(&self, v: Float) -> Point3<Float> {
        self.origin + self.direction * v
    }
}

