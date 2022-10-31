use std::sync::Arc;
use rand::Rng;
use rayon::prelude::*;

mod math;
use math::{Float3, ColorRGB, Point3, Ray, Hit, HitRecord};
mod sphere;
use sphere::{Sphere, World};
// ray-tracer following the Ray Tracing in a Weekend book in Rust
use std::io::{stderr, Write};
mod camera;
use camera::{Camera};
pub mod material;
use material::{Lambertian, Metal, Dielectric};


fn ray_color(r: &Ray, world: &World, depth: u64) -> ColorRGB {

    if depth <= 0 {
        return ColorRGB::new(0.0, 0.0, 0.0)
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
      if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
        attenuation * ray_color(&scattered, world, depth - 1)
      } else {
        ColorRGB::new(0.0, 0.0, 0.0)
      }
    } else {
        let unit_dir = r.direction().normalized();
        let t = 0.5 * (unit_dir.y() + 1.0);
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

fn random_sphere_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(ColorRGB::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9)
            );

            if choose_mat < 0.8 {
                // diffuse mat
                let albedo = ColorRGB::random(0.0..1.0) * ColorRGB::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // metal mat
                let albedo = ColorRGB::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // glass mat
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(ColorRGB::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(ColorRGB::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-0.4, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}


fn main() {
    
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const MAX_DEPTH: u64 = 50;

    let mut world = random_sphere_scene();
    
    let aperature = 0.1;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Float3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        lookfrom, 
        lookat, 
        vup, 
        20.0, 
        ASPECT_RATIO,
        aperature,
        dist_to_focus
    );

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {:3}", j + 1);

        let scanline: Vec<ColorRGB> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = ColorRGB::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();
        
        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

      eprintln!("Complete.");
}