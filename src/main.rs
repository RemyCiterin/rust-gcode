use rust_gcode::parse_image::parse_image;
use rust_gcode::height_map::*;


pub fn main() {
    let img_to_vector = parse_image("./test_png_image.png").unwrap();

    println!("{}", img_to_vector.len());

    println!("Hello, world!");
    let mut hmap1 = HeightMap::new(1055, 1055);
    for i in 0..hmap1.get_width() {for j in 0..hmap1.get_height() {
        hmap1.set(i, j, 1.45 * (i as f64) + 0.763 * (j as f64));
    }}


    let mut hmap2 = HeightMap::new(10, 10);
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
    println!("{}", result);
}
