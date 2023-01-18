use crate::height_map::*;
use image::{RgbImage, DynamicImage, Pixel, Luma, ImageFormat::Png};

pub struct BitMap {
    height: usize,
    width : usize,
    buffer: Vec<bool>
}

impl BitMap {

    pub fn new(width: usize, height : usize) -> Self {
        let mut buffer = Vec::with_capacity(width * height);
        for _ in 0..width*height {buffer.push(false);}

        BitMap {width, height, buffer}
    }

    pub fn from_height_map(hmap:&HeightMap, max_val:f64) -> Self {
        let mut out = Self::new(hmap.get_width(), hmap.get_height());

        for i in 0..out.width {
            for j in 0..out.height {
                out.set(i, j, hmap.get(i, j) <= max_val);
            }
        }

        out
    }

    pub fn unsafe_set(&mut self, x:usize, y:usize, val:bool) -> &mut Self {
        self.buffer[x * self.height + y] = val;
        self
    }

    pub fn set(&mut self, x:usize, y:usize, val:bool) -> &mut Self {
        assert!(x < self.width && y < self.height);
        self.unsafe_set(x, y, val)
    }

    pub fn unsafe_get(&self, x:usize, y:usize) -> bool {
        self.buffer[x * self.height + y]
    }

    pub fn get(&self, x:usize, y:usize) -> bool {
        assert!(x < self.width && y < self.height);
        self.unsafe_get(x, y)
    }

    pub fn get_default(&self, x:usize, y:usize) -> bool {
        if x < self.width && y < self.height {self.get(x, y)}
        else {false}
    }

    pub fn save(&self, path:&str) -> Result<(), String> {
        let mut rgb: RgbImage = RgbImage::new(self.width as u32, self.height as u32);

        for i in 0..self.width {
            for j in 0..self.height {
                let pixel = Luma::<u8>::from_slice(&[if self.get(i, j) {0} else {255}]).to_rgb();
                rgb.put_pixel(i as u32, j as u32, pixel);
            }
        }

        if let Err(_) = DynamicImage::ImageRgb8(rgb).save_with_format(path, Png){
            Err("unable to save the image".to_string())
        } else {Ok(())}

    }

}

#[derive(Clone, Copy)]
pub enum Move{
    XYmove(f64, f64), // move to the position `x, y, same_z_as_current`
    Zmove(f64), // move to the position `same_x_as_current, same_y_as_current, z`
    FXYmove(f64, f64) // move fast to the position `x, y, same_z_as_current`
}

#[derive(Clone)]
pub struct Path {
    pub x_init : f64,
    pub y_init : f64,
    pub z_init : f64,
    pub path:Vec<Move>
}

pub trait PathAlgo {
    fn from_bit_map(&self, bit_map:&BitMap, x_init:f64, y_init:f64, z_init:f64) -> Path;
}
