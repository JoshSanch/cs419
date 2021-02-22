use nalgebra::Vector3;

pub type Point3 = Vector3<f64>;

pub struct Ray {
    orig: Point3,
    dir: Vector3<f64>,
}

impl Ray { 
    pub fn new(origin: &Point3, direction: &Vector3<f64>) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction
        }
    }

    pub fn find_pos_at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
