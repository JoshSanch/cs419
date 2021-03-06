use nalgebra::Vector3;
use crate::Ray;
use crate::utils::Unit;
use crate::Point3;
use crate::renderables::Sphere;

pub type Color = Vector3<u8>;

pub fn calc_ray_color(r: &Ray) -> Color {
    let sphere_orig = Point3::new(0.,0.,-1.);
    let hit_time = hit_sphere(&sphere_orig, 0.5, r);

    if hit_time > 0. {
        let normal = (r.find_pos_at(hit_time) - Vector3::new(0.,0.,-1.)).unit();
        let float_vec = 0.5 * 255. * Vector3::new(normal.x + 1., normal.y + 1., normal.z + 1.);
        println!("{:?}", float_vec);
        
        return Color::new(float_vec.x as u8, float_vec.y as u8, float_vec.z as u8)
    }

    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    let float_vec = (1.0 - t) * Vector3::new(255., 255., 255.) + t * Vector3::new(128., 200., 255.);
    
    Color::new(float_vec.x as u8, float_vec.y as u8, float_vec.z as u8)
}