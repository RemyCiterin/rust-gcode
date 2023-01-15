use image::{Rgb, Pixel};
use rust_gcode::parse_image::parse_image;
use rust_gcode::height_map::*;


pub fn main() {
    let hmap1 = parse_image(
        "./test_png_image.png",
        Rgb::from_slice(&[255, 255, 255]).clone()
    ).unwrap();

    println!("Hello, world!");

    let mut hmap2 = HeightMap::new(20, 20);
    for i in 0..hmap2.get_width() {for j in 0..hmap2.get_height() {
        hmap2.set(i, j, 1.4 * (i as f64) + 0.73 * (j as f64));
    }}

    let out = hmap1.par_get_max_plus_convolve(&hmap2);

    assert_eq!(out.get_height(), hmap1.get_height() + 1 - hmap2.get_height());
    assert_eq!(out.get_width(), hmap1.get_width() + 1 - hmap2.get_width());

    let mut result = 0.0;
    for i in 0..out.get_width() {
        for j in 0..out.get_height() {
            result += out.get(i, j);
        }
    }

    hmap1.save(-1.0, 0.0, "test_out.png").expect("unable to save the image");
    println!("{}", result);
}
