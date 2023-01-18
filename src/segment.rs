use std::ops::{Add, Mul, Sub, Neg};
#[allow(unused_imports)]
use rand::*;

use crate::bit_map::BitMap;
#[allow(unused_imports)]
use crate::height_map::*;

/// structure for vector of size 2
/// implement the trait `Add`, `Sub`, `Neg`, `Mul<f64>`
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec2{x:f64, y:f64}

impl Vec2 {
    /// create a new Vec2 from two f64
    pub fn new(x:f64, y:f64) -> Self {
        Vec2{x, y}
    }

    /// return the value of the vector along the x-axis
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// return the value of the vector along the y-axis
    pub fn get_y(&self) -> f64 {
        self.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other:Self) -> Self {
        Vec2{x:self.x + other.x, y:self.y+other.y}
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other:Self) -> Self {
        Vec2::new(self.x-other.x, self.y-other.y)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other:f64) -> Self {
        Vec2::new(self.x * other, self.y * other)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, other:Vec2) -> Vec2 {
        other * self
    }
}

impl Mul for Vec2 {
    type Output = f64;
    fn mul(self, other:Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// this structure represent a half line using a source and a non-zero direction 2D vector
#[derive(Clone, Copy)]
pub struct HalfLine {
    src:Vec2,
    dir:Vec2
}

/// represent a segment using a source and a target (the source and target need to be different)
#[derive(Clone, Copy)]
pub struct Segment {
    src:Vec2,
    tgt:Vec2
}

impl Segment {
    /// create a segment using two point of it
    pub fn new(source:Vec2, target:Vec2) -> Self {
        Segment{src:source, tgt:target}
    }

    /// return the source vector of a segment
    pub fn source(&self) -> Vec2 {self.src}

    /// return the target vector of a segment
    pub fn target(&self) -> Vec2 {self.tgt}

    pub fn to_half_line(&self) -> HalfLine {
        HalfLine::new(self.src, self.tgt-self.src)
    }
}


impl HalfLine {

    /// create a new half line using a source and a direction,
    /// the direction must be non-zero
    pub fn new(src:Vec2, dir:Vec2) -> Self {
        HalfLine {src, dir}
    }

    /// `self.intersection(line, eps)`
    /// compute the intersection of a half line `self` and a line `line`
    ///
    /// self = {self.src + self.dir * X | X : f64}
    ///
    /// line = {line.src + line.dir * X | X : f64}
    ///
    /// then
    ///
    /// X * self.dir - Y * line.dir = line.src - self.src
    ///
    /// is equivalent to
    ///
    /// | self.dir.x  -line.dir.x | |X| = line.src - self.src
    ///
    /// | self.dir.y  -line.dir.y | |Y|
    ///
    /// this fonction return the values of X, Y that verify this equation if
    /// their exists
    pub fn plan_intersection(&self, line:HalfLine, eps:f64) -> Option<Vec2> {
        let equal_eps = |f1:f64, f2:f64| -> bool {
            f1 + eps >= f2 && f2 + eps >= f1
        };

        let b = line.src - self.src;
        if equal_eps(b.x, 0.0) && equal_eps(b.y, 0.0) {
            return Some(self.src);
        }

        let det = - self.dir.x * line.dir.y + line.dir.x * self.dir.y;

        if equal_eps(det, 0.0) {return None;}

        let x = (-line.dir.y * b.x + line.dir.x * b.y) / det;
        let y = (-self.dir.y * b.x + self.dir.x * b.y) / det;

        debug_assert!(equal_eps((self.src + self.dir * x).x, (line.src + line.dir * y).x));
        debug_assert!(equal_eps((self.src + self.dir * x).y, (line.src + line.dir * y).y));

        Some(Vec2::new(x, y))
    }

    pub fn source(&self) -> Vec2 {self.src}
    pub fn direction(&self) -> Vec2 {self.dir}
}

pub enum Case2<T>{
    C0,
    C1(T),
    C2(T, T)
}


pub fn get_segments_from_pixel<F>(f:F, x:isize, y:isize) -> Case2<Segment>
    where
        F: Fn(isize, isize) -> bool
{
    let p00 = f(x, y);
    let p10 = f(x+1, y);
    let p01 = f(x, y+1);
    let p11 = f(x+1, y+1);

    let x = x as f64;
    let y = y as f64;
    let v00 = Vec2::new(x, y);
    let v10 = Vec2::new(x+1.0, y);
    let v01 = Vec2::new(x, y+1.0);
    let v11 = Vec2::new(x+1.0, y+1.0);

    let s11 = Segment::new(
        (v10 + v11) * 0.5,
        (v01 + v11) * 0.5
    );

    let s00 = Segment::new(
        (v00 + v10) * 0.5,
        (v00 + v01) * 0.5
    );

    let s10 = Segment::new(
        (v11 + v10) * 0.5,
        (v00 + v10) * 0.5
    );

    let s01 = Segment::new(
        (v11 + v01) * 0.5,
        (v00 + v01) * 0.5
    );

    // TF
    // FT
    if p00 && p11 && !p10 && !p01 {
        return Case2::C2(
            s00, s11
        );
    }

    // FT
    // TF
    if !p00 && !p11 && p01 && p10 {
        return Case2::C2(
            s01, s11
        );
    }

    if p10 != p00 && p01 != p00 && p11 != p00 {return Case2::C1(s00);}

    if p10 != p00 && p10 != p01 && p10 != p11 {return Case2::C1(s10);}

    if p01 != p00 && p01 != p10 && p01 != p11 {return Case2::C1(s01);}

    if p11 != p00 && p11 != p01 && p11 != p10 {return Case2::C1(s11);}

    if p00 == p10 && p00 != p01 && p00 != p11 {
        return Case2::C1(
            Segment::new(
                (v00 + v01) * 0.5,
                (v10 + v11) * 0.5
            )
        );
    }

    if p00 == p01 && p00 != p10 && p00 != p11 {
        return Case2::C1(
            Segment::new(
                (v00 + v10) * 0.5,
                (v01 + v11) * 0.5
            )
        );
    }

    if p00 == p11 && p01 == p00 && p10 == p00 {return Case2::C0;}

    unreachable!()

}

impl BitMap {
    pub fn from_pixel_to_segments(&self, x:isize, y:isize) -> Case2<Segment> {
        get_segments_from_pixel(|i:isize, j:isize | if i >= 0 && j >= 0 {self.get_default(i as usize, j as usize)} else {false}, x, y)
    }
}

impl HeightMap {
    /// take a height map as input and return the set of segments
    /// (the boundary between pixels smaller and larger than z)
    /// include in this height map between the pixel
    /// `(i, y)` and `(i+1, y+1)`
    pub fn from_pixel_to_segments(&self, x:isize, y:isize, z:f64) -> Case2<Segment> {
        get_segments_from_pixel(|i:isize , j:isize | if i >= 0 && j >= 0 {self.get_default(i as usize, j as usize) <= z} else {false}, x, y)
    }

    pub fn get_f64(&self, x:f64, y:f64) -> f64 {
        let x = if x >= -1e-9 {x.round() as usize} else {return 0.0;};
        let y = if y >= -1e-9 {y.round() as usize} else {return 0.0;};

        self.get_default(x, y)
    }
}

/// an iterator for iterate on the intersections between a half line and
/// an height map
pub struct IntersectionIterator {
    line: HalfLine,
    hmap: HeightMap
}

impl IntersectionIterator {
    pub fn new(hmap: HeightMap, line:HalfLine) -> Self {
        Self{line, hmap}
    }

    pub fn next(&mut self) -> Option<(Vec2, bool)> {
        unimplemented!()
    }

    pub fn get_hmap<'a>(&'a self) -> &'a HeightMap {&self.hmap}
    pub fn get_half_line(&self) -> HalfLine {self.line}
}

#[cfg(test)]
mod tests {
    use crate::segment::*;

    #[test]
    fn test_is_intersection() {
        let line1 = HalfLine::new(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 0.5));
        let line2 = HalfLine::new(Vec2::new(1.0, 0.0), Vec2::new(-1.0, 1.0));

        let result = line1.plan_intersection(line2, 1e-6);


        assert!(result.is_some());
        assert!(Vec2::new(1.0, 0.0) == line1.source() + line1.direction() * result.unwrap().get_x());

        let mut rng : rngs::ThreadRng = rand::thread_rng();

        for _ in 0..10000 {
            let line1 = HalfLine::new(
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
            );

            let line2 = HalfLine::new(
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
            );

            let _ = line1.plan_intersection(line2, 1e-6);
        }
    }
}
