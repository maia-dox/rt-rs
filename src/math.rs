use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Range};
use std::sync::Arc;
use std::fmt;
use std::fmt::Display;
use rand::prelude::*;

use super::material::Scatter;

// specifically the math for our vector impl (Float3), Hits and Rays

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

impl Mul<Float3> for f64 {
    type Output = Float3;

    fn mul(self, other:Float3) -> Float3 {
        Float3 {
            data: [self * other[0], self * other[1], self * other[2]]
        }
    }
}

impl Mul<Float3> for Float3 {
    type Output = Float3;

    fn mul(self, other: Float3) -> Float3 {
        Float3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]]
        }
    }
}

impl MulAssign<Float3> for Float3 {
    fn mul_assign(&mut self, other: Float3) -> () {
        *self = Float3 {
            data: [self[0] * other[0], self[1] * other[1], self[2] * other[2]]
        };
    }
}

impl MulAssign<f64> for Float3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Float3 {
            data: [self[0] * other, self[1] * other, self[2] * other]
        };
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

    pub fn random(r: Range<f64>) -> Float3 {
        let mut rng = rand::thread_rng();

        Float3 {
            data: [rng.gen_range(r.clone()), rng.gen_range(r.clone()), rng.gen_range(r.clone())]
        }
    }

    pub fn random_in_unit_sphere() -> Float3 {
        loop {
            let v = Float3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal:Float3) -> Float3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Float3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Float3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    pub fn almost_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, n: Float3) -> Float3 {
        self - 2.0 * self.dot(n) * n
    }


    // snell's law
    pub fn refract(self, n: Float3, etai_over_etat: f64) -> Float3 {
        let cos_theta = ((-1.0) * self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * n;
        
        r_out_perp + r_out_parallel
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {

        let ir = (256.0 * (self[0] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
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
    pub mat: Arc<dyn Scatter>,
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
}

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}