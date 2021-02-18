use nalgebra::Vector3;

type Point3 = Vector3<f64>;

struct Ray {
    orig: Point3,
    dir: Vector3<f64>,
}

impl Ray { 
    fn new(origin: &Point3, direction: &Vector3<f64>) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction
        }
    }

    fn find_pos_at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
