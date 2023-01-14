use image::{io::Reader as ImageReader, GenericImageView, Rgba};


// take a path to an image and return it as Rgb array
pub fn parse_image(path : &str) -> Result<Vec<Rgba<u8>>, String> {
    let img = ImageReader::open("./test_png_image.png");//.unwrap().decode().unwrap();

    match img {
        Ok(img) => {
            match img.decode() {
                Ok(img) => {

                    let (width, height) = img.dimensions();

                    println!("{} {}", width, height);

                    let mut img_to_vector : Vec<Rgba<u8>> = vec![];

                    for i in 0..width {
                        for j in 0..height {
                            img_to_vector.push(img.get_pixel(i, j).clone());
                        }
                    }

                    Ok(img_to_vector)
                },
                Err(_) => Err("unable to decode the image".to_string())
            }
        },
        Err(_) => Err(format!("unable to open the file `{}`", path))

    }
}
