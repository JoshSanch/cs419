mod imgrender;
mod shading;

use shading::Color;

fn main() {

    // Image 

    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {} \n255\n", image_width, image_height );

    for y in 0..image_height {
        println!("Scanlines remaining: {}", image_height);
        for x in 0..image_width {
            let pixel_color: Color = Color::new(
                x as f64 / (image_width-1) as f64,
                y as f64 / (image_height-1) as f64, 
                0.25
            );
            
        }
    }

}
