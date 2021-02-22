use super::color::Material;
use super::rays::Point3;

use nalgebra::Vector3;

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