use nalgebra::Vector3;

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

pub struct Plane {
    orig: Point3,
    normal: Vector3<f64>
}

impl Plane {
    pub fn new(origin: Point3, norm: Vector3<f64>) -> Plane {
        Plane {
            orig: origin,
            normal: norm
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let discriminant = ray.dir.dot(&self.normal);
        if discriminant == 0. {
            return false;
        }

        let hit_time = (self.orig - ray.orig).dot(&self.normal);
        if hit_time < t_min || hit_time > t_max {
            return false;
        }

        record.hit_time = hit_time; 
        record.hitpt = ray.find_pos_at(record.hit_time);
        record.set_face_normal(&ray, &self.normal);

        return true;
    }
}

pub struct Triangle {
    point_one: Point3,
    point_two: Point3,
    point_three: Point3,
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        // Adapted from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let edge1 = self.point_two - self.point_one;
        let edge2 = self.point_three - self.point_one;
        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);

        if a == 0. {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.orig - self.point_one;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);

        if v < t_min || u + v > 1.0 {
            return false;
        } else {
            record.hit_time = f * edge2.dot(&q);
            record.hitpt = ray.find_pos_at(record.hit_time);
            record.set_face_normal(&ray, &self.calc_surface_normal());

            return true;
        }
    }
}

impl Triangle {
    pub fn new(pt1: Point3, pt2: Point3, pt3: Point3) -> Triangle {
        Triangle {
            point_one: pt1,
            point_two: pt2,
            point_three: pt3
        }
    }

    pub fn calc_surface_normal(&self) -> Vector3<f64> {
        // Adapted from https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal
        let u = self.point_two - self.point_one;
        let v = self.point_three - self.point_one;

        let nx = u.y * v.z - u.z * v.y;
        let ny = u.z * v.x - u.x * v.z;
        let nz = u.x * v.y - u.y * v.x;

        Vector3::new(nx, ny, nz)
    }
}