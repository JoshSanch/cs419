use nalgebra::Vector3;
use super::rays::Point3;
use super::rays::Ray;

/**
 * Adapted from Section 11.38 of Ray Tracing in One Weekend.
 */
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horiz_vec: Vector3<f64>,
    vert_vec: Vector3<f64>
}

pub enum CameraMode {
    Orthographic,
    Perspective
}

impl Camera {
    /// Constructor for Camera struct.
    /// 
    /// lookfrom: Origin point for the camera.
    /// lookat: Desired point for the camera to look at.
    /// vup: Vector representing the up direction in worldspace.
    /// vfov: Vertical field-of-view in degrees.
    /// aspect_ratio: the aspect ratio of the desired image output.
    /// world_dims: view plane height in world coordinates
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vector3<f64>, 
                vfov: f64, aspect_ratio: f64, world_height: f64) -> Camera {
        let theta = vfov.to_radians();
        let viewport_height = world_height * (theta / 2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w: Point3 = (lookfrom - lookat) / (lookfrom - lookat).norm();
        let u = vup.cross(&w) / (vup.cross(&w)).norm();
        let v = w.cross(&u);
        
        Camera {
            origin: lookfrom,
            horiz_vec: viewport_width * u,
            vert_vec: viewport_height * v,
            lower_left_corner: lookfrom - (viewport_width * u / 2.) - (viewport_height * v) - w
        }
    }

    /// 
    /// Calculate the ray associated with a given x & y coordinate in world space.
    /// 
    pub fn calc_ray(&self, x: i32, y: i32, mode: CameraMode) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + (x as f64) * self.horiz_vec + (y as f64) * self.vert_vec - self.origin))
    }
}