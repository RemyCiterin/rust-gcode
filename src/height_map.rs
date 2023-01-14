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
        self.buffer[x+y*self.width]
    }

    pub fn set(&mut self, x:usize, y:usize, f:f64) -> &mut Self {
        self.buffer[x+y*self.width] = f;
        self
    }

    pub fn set_from_rgba8(&mut self, x:usize, y:usize, pixel:Rgba<u8>, background:Rgb<u8>) -> &mut Self {
        self.set(x, y, panic!("need to compute it"))
    }

    pub fn get_height(&self) -> usize {self.height}
    pub fn get_width(&self) -> usize {self.width}

}
