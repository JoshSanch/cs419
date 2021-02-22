mod rays;
mod imgrender;
mod color;
mod camera;
mod renderables;

use rays::Point3;
use color::Material;
use nalgebra::Vector3;

fn main() {
    // Min assignment spec for MP1 is 500x500 image resolution
    let image_height = 500;
    let image_width = 500;

    /* Initialize objects for rendering in the scene. */
    let sphere = renderables::Sphere::new(
        Point3::new(0., 3., 3.), 
        5.,
        Material::new(0, 0, 255)
    );
    let plane = renderables::Plane::new(
        Point3::new(0., 0.5, 0.), 
        Vector3::new(1., 0., 0.), 
        Material::new(255, 0, 0)
    );
    let triangle = renderables::Triangle::new(
        Point3::new(1., 2., 2.),
        Point3::new(1., 3., 1.),
        Point3::new(2., 2., 2.),
        Material::new(0, 255, 0)
    );
}