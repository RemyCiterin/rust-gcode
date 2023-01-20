use image::{Rgb, Pixel};
use rust_gcode::parse_config::*;
use rust_gcode::parse_image::parse_image;
use rust_gcode::bit_map::*;

pub fn main() {
    let args : Vec<String> = std::env::args().collect();
    let (_config_file, _hmap_file) = get_path(&args);

    let _config = Config::new(&_config_file).unwrap();

    println!("{}", _config.fly_z);


    let hmap1 = parse_image(
        "./test_png_image.png",
        Rgb::from_slice(&[255, 255, 255]).clone()
    ).unwrap();

    println!("Hello, world! {}", hmap1.get(0, 0));


    let depth = 0.002;

    // adapt the hmap with the shape of the tool for under-approximate the final shape
    let hmap2 = hmap1.generate_tool_hmap(0.2, 0.1, depth, ToolShape::Ball(0.0012));

    hmap2.save(-depth, 0.0, "test_hmap.png").expect("unable to save the height map");
    let bmap = BitMap::from_height_map(&hmap2, -depth * 0.5);
    bmap.save("test_bmap.png").expect("unable to save the bit map");
    //println!("{}", result);
}
