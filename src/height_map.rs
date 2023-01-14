use image::{Rgba, Rgb};


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
        assert!(self.height + g.height >= f.height);
        assert!(self.width + g.width >= f.width);

        for i in 0..self.width {
            for j in 0..self.height {
                let mut maxopt : Option<f64> = None;

                for x in 0..g.width {
                    for y in 0..g.height {
                        let val = g.unsafe_get(g.width-1-x, g.height-1-y) + f.unsafe_get(i+x, j+y);
                        if let Some(max) = maxopt {
                            if val > max {maxopt = Some(val);}
                        } else {maxopt = Some(val);}
                    }
                }

                if let Some(max) = maxopt {
                    self.set(i, j, max);
                }
            }
        }

        self
    }

    // immutable 2D convolution in the semi-ring (R, max, +)
    pub fn get_max_plus_convolve(&self, other:&Self) -> Self {
        assert!(self.height >= other.height);
        assert!(self.width  >= other.width );

        let mut buffer = Self::new(self.width - other.width, self.height - other.height);
        buffer.set_max_plus_convolve(self, other);

        buffer
    }

}
