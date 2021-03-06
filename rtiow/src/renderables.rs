use nalgebra::Vector3;
use crate::shading::Color;
use crate::utils::Unit;

/// Alias for a 3D vector of doubles. Used to signify that the vector represents a point.
pub type Point3 = Vector3<f64>;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3<f64>) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn find_pos_at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
