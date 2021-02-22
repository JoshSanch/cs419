use nalgebra::Vector3;

pub type Color = Vector3<i32>;

pub struct Material {
    base_color: Color
}

impl Material {
    pub fn new(r: i32, g: i32, b: i32) -> Material {
        Material {
            base_color: Color::new(r, g, b)
        }
    }
}