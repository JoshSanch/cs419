mod rays;
mod imgrender;
mod color;
mod camera;
mod renderables;
mod scene;

use nalgebra::Vector3;

use rays::Point3;
use color::Material;
use camera::Camera;
use camera::CameraMode;
use scene::Scene;
use renderables::SceneObject;
use renderables::Sphere;
use renderables::Plane;
use renderables::Triangle;
use color::Color;

fn main() {
    // Min assignment spec for MP1 is 500x500 image resolution
    let image_height: u32 = 500;
    let image_width: u32 = 500;

    /* Initialize objects for rendering in the scene. */
    let sphere = Sphere::new(
        Point3::new(0., 3., 3.), 
        0.5,
        Material::new(0, 0, 255)
    );
    let plane = Plane::new(
        Point3::new(0., 0.5, 0.), 
        Vector3::new(0., 1., 0.), 
        Material::new(255, 0, 0)
    );
    let triangle = Triangle::new(
        Point3::new(1., 2., 2.),
        Point3::new(1., 3., 1.),
        Point3::new(2., 2., 2.),
        Material::new(0, 255, 0)
    );

    let objs: Vec<Box<dyn SceneObject>> = vec!(Box::new(sphere), Box::new(plane), Box::new(triangle));
    let scene = Scene::new(objs);

    /* Set up initial camera angle to begin ray-casting onto scene objects */
    let world_height_max = 20.;
    let camera = Camera::new(
        Point3::new(0., 1., 0.),
        Point3::new(0., 1., 200.),
        Vector3::new(0., 1., 0.),
        120.,
        image_height as f64 / image_width as f64,
        world_height_max,
    );

    /* Get pixel color values for first perspective image */
    let mut pixel_colors: Vec<u8> = vec!();
    for pix_y in 0..image_height {
        for pix_x in 0..image_width {
            let ray = camera.calc_ray(pix_x, pix_y, CameraMode::Perspective);
            let closest_obj = scene.find_nearest_obj(&ray);

            let pixel_color: Color;
            if closest_obj.is_none() {
                pixel_color = Scene::calc_bg_color(image_height, image_width, pix_y);
            } else {
                pixel_color = closest_obj.unwrap().get_material().base_color;
            }

            println!("{:?}", pixel_color);
            pixel_colors.push(pixel_color.x);  // r
            pixel_colors.push(pixel_color.y);  // g
            pixel_colors.push(pixel_color.z);  // b
        }
    }

    //println!("{:?}", pixel_colors);

    imgrender::render_image(image_height, image_width, &pixel_colors, "/home/sway/Documents/CS419_Renders/basic_persp.png");
}