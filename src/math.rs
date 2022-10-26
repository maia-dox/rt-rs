use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy)]
pub struct Float3 {
    data: [f64; 3]
}

pub type Point3 = Float3;
pub type ColorRGB = Float3;

impl Float3 {
    pub fn new(f0: f64, f1: f64, f2: f64) -> Float3 {
        Float3 {
            data: [f0, f1, f2]
        }
    }
}

impl Index<usize> for Float3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.data[index]
    }
}

impl IndexMut<usize> for Float3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.data[index]
    }
}

