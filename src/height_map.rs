use image::{Rgba, Rgb};
use rayon::prelude::*;

pub struct HeightMap {
    buffer : Vec<f64>,
    width : usize,
    height: usize
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
        self.set(x, y, panic!("need to compute it"))
    }

    pub fn get_height(&self) -> usize {self.height}
    pub fn get_width(&self) -> usize {self.width}


    // inplace naive algorithm for 2D convolution in the semi-ring (R, max, +)
    pub fn set_max_plus_convolve(&mut self, f:&Self, g:&Self) -> &mut Self {
        assert!(self.height >= f.height + 1 - g.height);
        assert!(self.width >= f.width + 1 - g.width);

        for i in 0..self.width {
            for j in 0..self.height {
                if let Some(max) = f.get_max_plus_convolve_at(g, i, j) {
                    self.unsafe_set(i, j, max);
                }
            }
        }

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

}

#[cfg(test)]
mod tests {
    use crate::height_map::*;

    #[test]
    fn test_size() {
        let hmap1 = HeightMap::new(155, 155);
        let hmap2 = HeightMap::new(3, 3);

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

        let out = hmap1.get_max_plus_convolve(&hmap2);

        let v_out = HeightMap::new_with_buffer(4, 2, vec![
            5.0, 6.0,
            6.0, 5.0,
            7.0, 6.0,
            6.0, 7.0
        ]);

        for i in 0..out.get_width() {
            for j in 0..out.get_height() {
                let x1 = out.get(i, j);
                let x2 = v_out.get(i, j);
                assert!(x1 < x2 + 1e-9 && x2 < x1 + 1e-9);
            }
        }
    }
}
