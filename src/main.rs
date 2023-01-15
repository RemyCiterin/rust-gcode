use image::{Rgb, Pixel};
use rust_gcode::parse_image::parse_image;
use rust_gcode::height_map::*;


pub fn main() {
    let hmap1 = parse_image(
        "./test_png_image.png",
        Rgb::from_slice(&[255, 255, 255]).clone()
    ).unwrap();

    println!("Hello, world! {}", hmap1.get(0, 0));


    let depth = 0.002;

    // adapt the hmap with the shape of the tool for under-approximate the final shape
    let hmap2 = hmap1.generate_tool_hmap(0.2, 0.1, depth, ToolShape::Flat(0.0012));

    hmap2.save(-2.0 * depth, 1.0 * depth, "test_out.png").expect("unable to save the image");
    //println!("{}", result);
}
