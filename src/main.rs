use rust_gcode::parse_image::parse_image;

pub fn main() {
    let img_to_vector = parse_image("./test_png_image.png").unwrap();

    println!("{}", img_to_vector.len());

    println!("Hello, world!");
}
