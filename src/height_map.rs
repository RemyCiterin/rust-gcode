use image::{Rgba, Rgb, Pixel, DynamicImage, RgbImage, Luma, ImageFormat::Png};

use rayon::prelude::*;

pub struct HeightMap {
    buffer : Vec<f64>,
    width : usize,
    height: usize
}

pub enum ToolShape {
    Flat(f64), // `Flat(f)` is a disk of rayon `f`
    Ball(f64), // `Ball(r)` is a ball of rayon `r`
    V(f64, f64)// `V(r, a)` is a `V` of rayon `r` and angle `a` in radian
}

impl ToolShape {
    pub fn get_rayon(&self) -> f64 {
        match self {
            ToolShape::Flat(r) => r,
            ToolShape::Ball(r) => r,
            ToolShape::V(r, _) => r
        }.clone()
    }

    pub fn get_size(&self) -> f64 {
        match self {
            ToolShape::Flat(_) => 1.0,
            ToolShape::Ball(r) => r.clone(),
            ToolShape::V(r, t) => r.clone() * f64::tan(t.clone())
        }
    }
}

impl HeightMap {
    pub fn new(width : usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(width * height);

        for _ in 0..width*height {
            buffer.push(0.0);
        }

        HeightMap {
            buffer,
            width,
            height
        }
    }

    // return an new height map such that the tool under-approximate the input fmap
    pub fn generate_tool_hmap(mut self, width:f64, height: f64, depth:f64, tool:ToolShape) -> Self {
        for i in 0..self.width {for j in 0..self.height {
            let x = self.get(i, j);
            self.set(i, j, x * depth);
        }}

        let out_width  = (self.width  as f64) * tool.get_rayon() / width ;
        let out_height = (self.height as f64) * tool.get_rayon() / height;

        let mut hmap = Self::new(
            2 * out_width as usize + 1,
            2 * out_height as usize + 1
        );


        for i in 0..hmap.width {
            for j in 0..hmap.height {
                let distance_to_center = f64::sqrt(
                    f64::powi((out_width  - i as f64) * width  / (self.width  as f64), 2) +
                    f64::powi((out_height - j as f64) * height / (self.height as f64), 2),
                );

                if distance_to_center > tool.get_rayon() {
                    hmap.set(i, j, -1e9);
                } else {
                    hmap.set(i, j, match tool {
                        ToolShape::Flat(_) => 0.0,
                        ToolShape::Ball(r) => {
                            let angle = f64::acos(distance_to_center / r);
                            assert!(f64::sin(angle) >= 0.0);
                            r * f64::sin(angle) - r
                        },
                        ToolShape::V(_, theta) => {
                            assert!(f64::tan(theta / 2.0) >= 0.0);
                            -distance_to_center / f64::tan(theta / 2.0)
                        }
                    });
                }
            }
        }

        hmap.save(-2.0 * tool.get_size(), 0.0, "./tool_shape.png").unwrap();

        self.par_get_max_plus_convolve(&hmap)
    }

    pub fn new_with_buffer(width:usize, height: usize, buffer:Vec<f64>) -> Self {
        assert_eq!(width * height, buffer.len());

        HeightMap {buffer, width, height}
    }

    pub fn get(&self, x:usize, y:usize) -> f64 {
        assert!(x < self.width && y < self.height);
        self.buffer[x*self.height+y]
    }

    pub fn set(&mut self, x:usize, y:usize, f:f64) -> &mut Self {
        assert!(x < self.width && y < self.height);
        self.buffer[x*self.height+y] = f;
        self
    }

    pub fn unsafe_get(&self, x:usize, y:usize) -> f64 {
        self.buffer[x*self.height+y]
    }

    pub fn unsafe_set(&mut self, x:usize, y:usize, f:f64) -> &mut Self {
        self.buffer[x*self.height+y] = f;
        self
    }

    pub fn set_from_rgba8(&mut self, x:usize, y:usize, pixel:Rgba<u8>, background:Rgb<u8>) -> &mut Self {
        let luma_a = pixel.to_luma_alpha();

        let alpha = luma_a.channels()[1] as usize as f64 / 255.0;
        let color = luma_a.channels()[0] as usize as f64 / 255.0;
        let background = background.to_luma().channels()[0] as usize as f64 / 255.0;

        let color = color * alpha + background * (1.0 - alpha);

        self.set(x, y, color-1.0)
    }

    pub fn get_height(&self) -> usize {self.height}
    pub fn get_width(&self) -> usize {self.width}


    // inplace naive algorithm for 2D convolution in the semi-ring (R, max, +)
    pub fn set_max_plus_convolve(&mut self, f:&Self, g:&Self) -> &mut Self {
        assert!(self.height >= f.height + 1 - g.height);
        assert!(self.width >= f.width + 1 - g.width);

        for i in 0..self.width {
            for j in 0..self.height {
                self.set(i, j, -1e9);
            }
        }

        for i in 0..self.width {
            for x in 0..g.width {
                for y in 0..g.height {
                    for j in 0..self.height {
                        let val = g.unsafe_get(g.width-1-x, g.height-1-y) + f.unsafe_get(i+x, j+y);
                        if val > self.unsafe_get(i, j) {self.unsafe_set(i, j, val);}
                    }
                }
            }
        }

        /*for i in 0..self.width {
            for j in 0..self.height {
                if let Some(max) = f.get_max_plus_convolve_at(g, i, j) {
                    self.unsafe_set(i, j, max);
                }
            }
        }*/

        self
    }


    fn get_max_plus_convolve_at(&self, other:&Self, i:usize, j:usize) -> Option<f64> {
        let mut maxopt = None;

        for x in 0..other.width {
            for y in 0..other.height {
                let val = other.unsafe_get(other.width-1-x, other.height-1-y) + self.unsafe_get(i+x, j+y);
                if let Some(max) = maxopt {if val > max {maxopt = Some(val);}}
                else {maxopt = Some(val);}
            }
        }

        maxopt
    }

    // immutable 2D convolution in the semi-ring (R, max, +)
    pub fn get_max_plus_convolve(&self, other:&Self) -> Self {
        assert!(self.height + 1 >= other.height);
        assert!(self.width + 1 >= other.width);

        let mut buffer = Self::new(self.width - other.width + 1, self.height - other.height + 1);
        buffer.set_max_plus_convolve(self, other);

        buffer
    }

    pub fn par_get_max_plus_convolve(&self, other:&Self) -> Self {
        assert!(self.height + 1 >= other.height);
        assert!(self.width + 1 >= other.width);

        let mut buffer = Self::new(self.width - other.width + 1, self.height - other.height + 1);
        buffer.par_set_max_plus_convolve(self, other);

        buffer
    }

    pub fn par_set_max_plus_convolve(&mut self, f:&Self, g:&Self) -> &mut Self {
        assert!(self.height >= f.height + 1 - g.height);
        assert!(self.width >= f.width + 1 - g.width);

        self.buffer.par_iter_mut().enumerate().for_each(|(addr, p)| {
            if let Some(max) = f.get_max_plus_convolve_at(g, addr / self.height, addr % self.height) {
                *p = max;
            }
        });

        self
    }

    pub fn save(&self, min:f64, max:f64, path:&str) -> Result<(), String> {
        let mut rgb: RgbImage = RgbImage::new(self.width as u32, self.height as u32);

        for i in 0..self.width {
            for j in 0..self.height {
                let mut color = (self.get(i, j) - min) / (max - min);
                if color > 1.0 {color = 1.0;}
                if color < 0.0 {color = 0.0;}

                let pixel = Luma::<u8>::from_slice(&[(255.0 * color) as usize as u8]).to_rgb();
                rgb.put_pixel(i as u32, j as u32, pixel);
            }
        }

        if let Err(_) = DynamicImage::ImageRgb8(rgb).save_with_format(path, Png){
            Err("unable to save the image".to_string())
        } else {Ok(())}

    }

}

#[cfg(test)]
mod tests {
    use crate::height_map::*;

    #[test]
    fn test_size() {
        let hmap1 = HeightMap::new(1920, 1080);
        let hmap2 = HeightMap::new(5, 5);

        let out = hmap1.get_max_plus_convolve(&hmap2);

        assert_eq!(out.get_height(), hmap1.get_height() + 1 - hmap2.get_height());
        assert_eq!(out.get_width(), hmap1.get_width() + 1 - hmap2.get_width());
    }


    #[test]
    fn test_output_value() {
        // warning the coordinate `(i, j)` is equivalent to the
        // index `i * height + j`, not `i + j * width`
        let hmap1 = HeightMap::new_with_buffer(5, 3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 2.0,
            3.0, 4.0, 5.0,
            6.0, 3.0, 4.0,
            5.0, 6.0, 7.0
        ]);

        let hmap2 = HeightMap::new_with_buffer(2, 2, vec![
            0.0, 1.0,
            1.0, 0.0
        ]);

        let out1 = hmap1.get_max_plus_convolve(&hmap2);
        let out2 = hmap1.par_get_max_plus_convolve(&hmap2);

        let v_out = HeightMap::new_with_buffer(4, 2, vec![
            5.0, 6.0,
            6.0, 5.0,
            7.0, 6.0,
            6.0, 7.0
        ]);

        for i in 0..out1.get_width() {
            for j in 0..out1.get_height() {
                let x1 = out1.get(i, j);
                let x2 = v_out.get(i, j);
                assert!(x1 < x2 + 1e-9 && x2 < x1 + 1e-9);
            }
        }

        for i in 0..out2.get_width() {
            for j in 0..out2.get_height() {
                let x1 = out2.get(i, j);
                let x2 = v_out.get(i, j);
                assert!(x1 < x2 + 1e-9 && x2 < x1 + 1e-9);
            }
        }
    }
}
