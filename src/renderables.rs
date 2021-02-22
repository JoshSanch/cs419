use super::color::Material;
use super::rays::Point3;
use super::rays::Ray;

use nalgebra::Vector3;
use float_eq::float_eq;

/**
 * Trait to support determining if a ray intersects an object, and if so, at what time.
 */
pub trait Collision {
    /// Determines the time that the given ray intersects the object. If there is no intersection, then 
    /// return None.
    fn calc_intersect_time(&self, ray: &Ray) -> Option<f64>;
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

impl Collision for Plane {
    fn calc_intersect_time(&self, ray: &Ray) -> Option<f64> {
        // Check if the plane and pixel ray are parallel, which is a sign that they do not intersect.
        let determinant = ray.dir.dot(&self.normal_vec);
        if float_eq!(determinant, 0., abs <= 0.0000000001) {
            None 
        } else {
            let top = &(self.origin - ray.orig).dot(&self.normal_vec);
            let bottom = determinant;
            return Some(top / bottom)
        }
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