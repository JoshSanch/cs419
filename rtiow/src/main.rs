mod imgrender;
mod shading;
mod utils;
mod renderables;
mod renderutil;

use nalgebra::Vector3;

use renderutil::Point3;
use renderutil::Ray;
use renderutil::HittableList;
use renderables::Sphere;
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
    let sphere2 = Box::new(Sphere::new(Point3::new(0., 0., -1.,), 0.5));

    world.add(sphere1);
    world.add(sphere2);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vector3::new(viewport_width, 0., 0.);
    let vertical = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., focal_length);

    // Render
    println!("P3\n{} {} \n255\n", image_width, image_height);

    let mut pixel_colors = vec!();
    for y in 0..image_height {
        println!("Scanlines remaining: {}", image_height - 1 - y);
        for x in 0..image_width {
            let u = x as f64 / (image_width-1) as f64;
            let v = y as f64 / (image_height-1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = calc_ray_color(&ray, &world);

            pixel_colors.push(pixel_color.x);
            pixel_colors.push(pixel_color.y);
            pixel_colors.push(pixel_color.z);
        }
    }

    render_image(image_height, image_width, &pixel_colors, "../img_output/test_img.png");
}
