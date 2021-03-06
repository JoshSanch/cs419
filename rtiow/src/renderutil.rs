use nalgebra::Vector3;

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


/// Ray intersection data for renderable objects
pub struct HitRecord {
    pub hitpt: Point3,
    pub hit_time: f64,
    normal: Vector3<f64>,
    front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&self, ray: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = ray.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {*outward_normal} else {*outward_normal * -1.};
    }
}

/// Define trait for objects that can have ray intersections.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}