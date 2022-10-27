use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use std::fmt;
use std::fmt::Display;

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

impl Add for Float3 {
    type Output = Float3;

    fn add(self, other: Float3) -> Float3 {
        Float3 {
            data: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        }
    }
}

impl AddAssign for Float3 {
    fn add_assign(&mut self, other: Float3) -> () {
        *self = Float3 {
            data:[ self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        };
    }
}

impl Sub for Float3 {
    type Output = Float3;
    
    fn sub(self, other: Float3) -> Float3 {
        Float3 {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        }
    }
}

impl SubAssign for Float3 {
    fn sub_assign(&mut self, other: Float3) -> () {
        *self = Float3 {
            data: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        };
    }
}

impl Mul<f64> for Float3 {
    type Output = Float3;

    fn mul(self, other: f64) -> Float3 {
        Float3 {
            data: [self[0] * other, self[1] * other, self[2] * other]
        }
    }
}

impl MulAssign<f64> for Float3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Float3 {
            data: [self[0] * other, self[1] * other, self[2] * other]
        };
    }
}

impl Mul<Float3> for f64 {
    type Output = Float3;

    fn mul(self, other:Float3) -> Float3 {
        Float3 {
            data: [self * other[0], self * other[1], self * other[2]]
        }
    }
}

impl Div<f64> for Float3 {
    type Output = Float3;

    fn div(self, other:f64) -> Float3 {
        Float3 {
            data: [self[0] / other, self[1] / other, self[2] / other]
        }
    }
}

impl DivAssign<f64> for Float3 {
    fn div_assign(&mut self, other:f64) -> () {
        *self = Float3 {
            data: [self[0] / other, self[1] / other, self[2] / other]
        };
    }
}

impl Float3 {

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Float3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Float3) -> Float3 {
        Float3 {
            data: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0]
            ]
        }
    }    

    pub fn normalized(self) -> Float3 {
        self / self.length()
    }

    pub fn format_color(self) -> String {
        format!("{} {} {}", (255.999 * self[0]) as u64,
                            (255.999 * self[1]) as u64,
                            (255.999 * self[2]) as u64)
    }
}

impl Display for Float3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}


// ------------------------------------------------------------------------

pub struct Ray {
    orig: Point3, 
    dir: Float3
}

impl Ray {
    pub fn new(origin: Point3, direction: Float3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Float3 {
        self.dir
    }

    pub fn at(&self, t:f64) -> Point3 {
        self.orig + t * self.dir
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Float3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Float3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else { 
            (-1.0) * outward_normal
        };
    }

    // fn hit(&self, r:&Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

