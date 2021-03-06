use crate::renderutil::Point3;
use crate::renderutil::Ray;
use crate::renderutil::Hittable;
use crate::renderutil::HitRecord;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(cent: Point3, rad: f64) -> Sphere{
        Sphere {
            center: cent,
            radius: rad
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let sphere_center = &ray.orig - self.center;
        let a = &ray.dir.magnitude().powi(2);
        let half_b = sphere_center.dot(&ray.dir);
        let c = sphere_center.magnitude().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        
        if discriminant < 0. {
            return false;
        } 

        let disc_sqrt = discriminant.sqrt();
        let mut root = (-half_b - disc_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + disc_sqrt) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        // Modify hit record passed to the function.
        record.hit_time = root;
        record.hitpt = ray.find_pos_at(root);
        let outward_normal = (record.hitpt - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        
        return true;
    }
}