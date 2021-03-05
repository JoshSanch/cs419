use super::color::Material;
use super::rays::Point3;
use super::rays::Ray;

use nalgebra::Vector3;
use float_eq::float_eq;
use core::fmt::Debug;

/**
 * Interface that contains essential rendering methods for any object that appears in a scene.
 */
pub trait SceneObject: Debug {
    /// Determines the time that the given ray intersects the object. If there is no intersection, then 
    /// return None.
    fn calc_intersect_time(&self, ray: &Ray) -> Option<Vec<f64>>;

    /// Generic getter for accessing the unique material from a scene.
    fn get_material(&self) -> &Material;   
}

/**
 * Sphere object for rendering spheres.
 * 
 */
pub struct Sphere {
    origin: Point3,
    radius: f64,
    mat: Material
}

impl Sphere {
    pub fn new(orig: Point3, rad: f64, material: Material) -> Sphere{
       Sphere {
           origin: orig,
           radius: rad,
           mat: material
       } 
    }
}

impl SceneObject for Sphere {
    fn calc_intersect_time(&self, ray: &Ray) -> Option<Vec<f64>> {
        let f = ray.orig - self.origin;
        let a = ray.dir.dot(&ray.dir);
        let b = 2. * (f.dot(&ray.dir));
        let c = f.dot(&f) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4. * a * c;
        if discriminant < 0. {
            None
        } else if float_eq!(discriminant, 0., abs <= 0.0000000001) {
            // Single intersection point
            let t = (-1. * b) / (2. * a);
            Some(vec!(t))
        } else {
            // Two intersection points
            let t_0 = (-1. * b - discriminant.sqrt()) / (2. * a);
            let t_1 = (-1. * b + discriminant.sqrt()) / (2. * a);
            Some(vec!(t_0, t_1))
        }
    }

    fn get_material(&self) -> &Material {
        &self.mat
    }
}

impl Debug for Sphere {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sphere: {} {}", self.origin, self.radius)
    }
}

/**
 * Plane object for rendering planes.
 * Planes are defined by a point on the plane and a vector representing the surface normal of the plane.
 */
pub struct Plane {
    origin: Point3,
    normal_vec: Vector3<f64>,
    mat: Material
}

impl Plane {
    pub fn new(orig: Point3, normal: Vector3<f64>, material: Material) -> Plane {
        Plane {
            origin: orig,
            normal_vec: normal,
            mat: material
        }
    }
}

impl SceneObject for Plane {
    fn calc_intersect_time(&self, ray: &Ray) -> Option<Vec<f64>> {
        // Check if the plane and pixel ray are parallel, which is a sign that they do not intersect.
        let discriminant = ray.dir.dot(&self.normal_vec);
        if float_eq!(discriminant, 0., abs <= 0.00000001) {
            None 
        } else {
            let num = (self.origin - ray.orig).dot(&self.normal_vec);
            let denom = discriminant;
            return Some(vec!(num / denom))
        }
    }

    fn get_material(&self) -> &Material {
        &self.mat
    }
}

impl Debug for Plane {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Plane: {} {}", self.origin, self.normal_vec)
    }
}

/**
 * Triangle object for rendering triangles.
 * Triangles are defined by three points.
 */
pub struct Triangle {
    point_one: Point3,
    point_two: Point3,
    point_three: Point3,
    mat: Material
}

impl Triangle {
    pub fn new(p1: Point3, p2: Point3, p3: Point3, material: Material) -> Triangle {
        Triangle {
            point_one: p1,
            point_two: p2,
            point_three: p3,
            mat: material
        }
    }
}

impl SceneObject for Triangle {
    fn calc_intersect_time(&self, ray: &Ray) -> Option<Vec<f64>> {
        // Adapted from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let epsilon = 0.0000001;
        let edge1 = self.point_two - self.point_one;
        let edge2 = self.point_three - self.point_one;
        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);

        if float_eq!(a, 0., abs <= epsilon) {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.orig - self.point_one;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        } else {
            return Some(vec!(f * edge2.dot(&q)));
        }
    }

    fn get_material(&self) -> &Material {
        &self.mat
    }
}

impl Debug for Triangle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Triangle: {} {} {}", self.point_one, self.point_two, self.point_three)
    }
}