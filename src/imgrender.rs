use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::Encoder;

pub fn render_image(img_height: u32, img_width: u32, pixel_values: &[u8], file_path: &str) {
    let rgba_byte_cnt: u32 = 3;

    // Make sure that each pixel has an RGBA value for valid rendering
    if pixel_values.len() as u32 % rgba_byte_cnt != 0 {
        panic!("Missing or extra RGB values in pixel slice. Unable to generate valid PNG file.");
    }

    if pixel_values.len() as u32 != (img_height * img_width) * rgba_byte_cnt {
        panic!("Incorrect amount of RGB values for specified image height and width.");
    }

    // Boilerplate setup from https://docs.rs/png/0.16.8/png/index.html#encoder
    let path = Path::new(&file_path);
    let file = File::create(path).unwrap();
    let ref mut writer = BufWriter::new(file);

    let mut encoder = Encoder::new(writer, img_width, img_height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(pixel_values).unwrap();              
}