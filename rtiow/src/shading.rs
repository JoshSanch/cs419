use nalgebra::Vector3;
use crate::Ray;
use crate::utils::Unit;
use crate::Point3;

pub type Color = Vector3<u8>;

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let sphere_center = r.orig - center;
    let a = r.dir.dot(&r.dir);
    let b = 2.0 * sphere_center.dot(&r.dir);
    let c = sphere_center.dot(&sphere_center) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    return discriminant > 0.;
}

pub fn calc_ray_color(r: &Ray) -> Color {
    let sphere_orig = Point3::new(0.,0.,-1.);
    if hit_sphere(&sphere_orig, 0.5, r) {
        return Color::new(255, 0, 0);
    }

    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    let float_vec = (1.0 - t) * Vector3::new(255., 255., 255.) + t * Vector3::new(128., 200., 255.);
    
    Color::new(float_vec.x as u8, float_vec.y as u8, float_vec.z as u8)
}