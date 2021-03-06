use nalgebra::Vector3;
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


/// Ray intersection data for renderable objects
#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub hitpt: Point3,
    pub hit_time: f64,
    pub normal: Vector3<f64>,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = ray.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {*outward_normal} else {*outward_normal * -1.};
    }
}

/// Define trait for objects that can have ray intersections.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

/// Create store for hittable objects
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
           if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
               hit_anything = true;
               closest_so_far = temp_rec.hit_time;
               *record = temp_rec;
           }
        }

        return hit_anything;
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec!()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

pub enum CameraMode {
    Orthographic,
    Perspective
}

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horiz: Vector3<f64>,
    pub vert: Vector3<f64>,
    pub forward: Vector3<f64>    
}

impl Camera {
    pub fn new(cam_origin: Point3, look_at: Point3, vup: Vector3<f64>, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov.to_radians();
        let viewport_height = 2. * (theta / 2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (cam_origin - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = cam_origin;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin: origin,
            horiz: horizontal,
            vert: vertical,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - w,
            forward: -1. * w
        }
    }

    pub fn calc_ray(&self, u: f64, v: f64, mode: &CameraMode) -> Ray {
        match mode {
            CameraMode::Perspective => {
                Ray::new(self.origin, self.lower_left_corner + u * self.horiz + v * self.vert - self.origin)
            },
            CameraMode::Orthographic => {
                Ray::new(self.lower_left_corner + u * self.horiz + v * self.vert, self.forward)
            },
        }
    }
}
