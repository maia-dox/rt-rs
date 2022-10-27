
mod math;
use math::{Float3, ColorRGB, Point3, Ray, Hit, HitRecord};
mod sphere;
use sphere::{Sphere, World};
// ray-tracer following the Ray Tracing in a Weekend book in Rust
use std::io::{stderr, Write};
mod camera;
use camera::{Camera};

use rand::Rng;

fn ray_color(r: &Ray, world: &World, depth: u64) -> ColorRGB {

    if depth <= 0 {
        return ColorRGB::new(0.1, 0.1, 0.1)
    }

    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        let target = rec.p + rec.normal + Float3::random_in_unit_sphere();
        let r = Ray::new(rec.p, target - rec.p);
       
        0.5 * ray_color(&r, world, depth-1)

    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * ColorRGB::new(1.0, 1.0, 1.0) + t * ColorRGB::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length().powi(2);
    let half_b = oc.dot(r.direction());
    let c = oc.length().powi(2) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) /  a
    }
}


fn main() {
    
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1024;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    let camera = Camera::new();


    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));


    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Float3::new(viewport_width, 0.0, 0.0);
    let vertical = Float3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 
                                - Float3::new(0.0, 0.0, focal_length);


    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {

        eprint!("\rScanlines completed: {:3}", IMAGE_HEIGHT - j );
        stderr().flush().unwrap();


        for i in 0..IMAGE_WIDTH {
           let mut pixel_color = ColorRGB::new(0.0, 0.0, 0.0);
           for _ in 0..SAMPLES_PER_PIXEL {
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();

            let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
            let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

            let r = camera.get_ray(u, v);
            pixel_color += ray_color(&r, &world, MAX_DEPTH);
           }

           println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("\nDone.");

    
}
