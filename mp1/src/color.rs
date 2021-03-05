use nalgebra::Vector3;

pub type Color = Vector3<u8>;

pub struct Material {
   pub base_color: Color
}

impl Material {
    pub fn new(r: u8, g: u8, b: u8) -> Material {
        Material {
            base_color: Color::new(r, g, b)
        }
    }
}