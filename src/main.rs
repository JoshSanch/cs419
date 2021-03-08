mod imgrender;
mod shading;
mod utils;
mod renderables;
mod renderutil;

use nalgebra::Vector3;

use renderutil::Point3;
use renderutil::Ray;
use renderutil::HittableList;
use renderutil::Camera;
use renderutil::CameraMode;

use renderables::Sphere;
use renderables::Plane;
use renderables::Triangle;

use shading::calc_ray_color;
use imgrender::render_image;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World
    let mut world = HittableList::new();
    let sphere1 = Box::new(Sphere::new(Point3::new(0., 0., -1.,), 0.5));
    let plane = Box::new(Plane::new(Point3::new(0., -2.0, 0.), Vector3::new(0., -1., 0.)));
    let triangle = Box::new(Triangle::new(
        Point3::new(0., 0., -5.),
        Point3::new(0., -4., -4.),
        Point3::new(2., 0., -4.)
    ));

    world.add(sphere1);
    world.add(plane);
    world.add(triangle);

    // Camera
    let cam = Camera::new(Point3::new(-2., 2., 1.), Point3::new(0., 0., -1.), Vector3::new(0., 1., 0.), 90., aspect_ratio);

    // Render
    println!("P3\n{} {} \n255\n", image_width, image_height);

    let mut pixel_colors = vec!();
    for y in 0..image_height {
        println!("Scanlines remaining: {}", image_height - 1 - y);
        for x in 0..image_width {
            let u = x as f64 / (image_width-1) as f64;
            let v = y as f64 / (image_height-1) as f64;
            let ray = cam.calc_ray(u, v, &CameraMode::Orthographic);
            let pixel_color = calc_ray_color(&ray, &world);

            pixel_colors.push(pixel_color.x);
            pixel_colors.push(pixel_color.y);
            pixel_colors.push(pixel_color.z);
        }
    }

    render_image(image_height, image_width, &pixel_colors, "img_output/test_img.png");
}
