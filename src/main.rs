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
    let image_width: u32 = 900;

    /* Initialize objects for rendering in the scene. */
    let sphere = Sphere::new(
        Point3::new(0., 2., 5.), 
        0.5,
        Material::new(0, 0, 200)
    );
    let plane = Plane::new(
        Point3::new(0., 0., 0.), 
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
    let world_height_max = 8.;
    let camera = Camera::new(
        Point3::new(0., 0.5, 0.),
        Point3::new(0., 0.5, 1.),
        Vector3::new(0., -1., 0.),
        90.,
        image_height as f64 / image_width as f64,
        world_height_max,
    );

    /* Render basic perspective image */
    render(
        &camera,
        &scene,
        image_height,
        image_width,
        CameraMode::Perspective,
        "/home/sway/Documents/CS419_Renders/basic_persp.png"
    );

    /* Set up alternate perspective */
    let alt_angle_cam = Camera::new(
        Point3::new(0.6, 0.5, 0.),
        Point3::new(0., 0.5, 1.),
        Vector3::new(0., -1., 0.),
        90.,
        image_height as f64 / image_width as f64,
        world_height_max,
    );

    render(
        &alt_angle_cam,
        &scene,
        image_height,
        image_width,
        CameraMode::Perspective,
        "/home/sway/Documents/CS419_Renders/alt_persp.png"
    );

    /* Render original camera with ortho persp */
    render(
        &alt_angle_cam,
        &scene,
        image_height,
        image_width,
        CameraMode::Orthographic,
        "/home/sway/Documents/CS419_Renders/basic_ortho.png"
    );
}

fn render(cam: &Camera, scene: &Scene, img_height: u32, img_width: u32, mode: CameraMode, path: &str) {
    /* Get pixel color values for first perspective image */
    let mut pixel_colors: Vec<u8> = vec!();
    for pix_y in 0..img_height {
        for pix_x in 0..img_width {
            let u = pix_x as f64 / (img_width - 1) as f64;
            let v = pix_y as f64 / (img_height - 1) as f64;
            let ray = cam.calc_ray(u, v, &mode);
            let closest_obj = scene.find_nearest_obj(&ray);

            let pixel_color: Color;
            if closest_obj.is_none() {
                pixel_color = Scene::calc_bg_color(img_height, img_width, pix_y);
            } else {
                pixel_color = closest_obj.unwrap().get_material().base_color;
            }

            pixel_colors.push(pixel_color.x);  // r
            pixel_colors.push(pixel_color.y);  // g
            pixel_colors.push(pixel_color.z);  // b
        }
    }

    imgrender::render_image(img_height, img_width, &pixel_colors, path);
}