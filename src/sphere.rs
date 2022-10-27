use super::math::{Float3, Point3, Hit, HitRecord, Ray};

pub struct Sphere {
    center: Point3, 
    radius: f64
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen, 
            radius: r
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64) ->Option<HitRecord> {
        let oc = r.origin() - self.center;
        
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;    // no hit
        }

        // find root in acceptable range that is nearest

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return None;    // no hit
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: Float3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);


        Some(rec)
    }

}

// pub struct World {
//     spheres: Vec<Sphere>
// }

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max:f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }

}