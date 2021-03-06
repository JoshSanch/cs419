use nalgebra::Vector3;
use crate::Ray;
use crate::utils::Unit;
use crate::Point3;
use crate::renderables::Sphere;
use crate::renderutil::HitRecord;
use crate::renderutil::Hittable;

pub type Color = Vector3<u8>;

pub fn calc_ray_color(r: &Ray, world: &Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0., f64::INFINITY, &mut rec) {
        let float_vec = 0.5 * (rec.normal + Vector3::new(255., 255., 255.));
        return Color::new(float_vec.x as u8, float_vec.y as u8, float_vec.z as u8);
    }

    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    let float_vec = (1.0 - t) * Vector3::new(255., 255., 255.) + t * Vector3::new(128., 200., 255.);
    
    Color::new(float_vec.x as u8, float_vec.y as u8, float_vec.z as u8)
}