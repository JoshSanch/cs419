use nalgebra::Vector3;
use crate::Ray;
use crate::utils::Unit;

pub type Color = Vector3<u8>;

pub fn calc_ray_color(r: &Ray) -> Color {
    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    let float_vec = (1.0 - t) * Vector3::new(255., 255., 255.) + t * Vector3::new(128., 200., 255.);
    
    Color::new(float_vec.x as u8, float_vec.y as u8, float_vec.z as u8)
}