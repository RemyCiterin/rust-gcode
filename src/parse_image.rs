use image::{io::Reader as ImageReader, GenericImageView, Rgb};
use crate::height_map::*;

// take a path to an image and return it as Rgb array
pub fn parse_image(path : &str, background:Rgb<u8>) -> Result<HeightMap, String> {
    let img = ImageReader::open("./test_png_image.png");//.unwrap().decode().unwrap();

    match img {
        Ok(img) => {
            match img.decode() {
                Ok(img) => {

                    let (width, height) = img.dimensions();

                    println!("{} {}", width, height);

                    let mut hmap = HeightMap::new(width as usize, height as usize);

                    for i in 0..width {
                        for j in 0..height {
                            hmap.set_from_rgba8(
                                i as usize,
                                j as usize,
                                img.get_pixel(i, j).clone(),
                                background
                            );
                        }
                    }

                    Ok(hmap)
                },
                Err(_) => Err("unable to decode the image".to_string())
            }
        },
        Err(_) => Err(format!("unable to open the file `{}`", path))

    }
}
