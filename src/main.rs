
mod math;
use math::{Float3, ColorRGB, Ray, Point3};

// ray-tracer following the Ray Tracing in a Weekend book in Rust
use std::io::{stderr, Write};



fn ray_color(r: &Ray) -> ColorRGB {

   let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - Float3::new(0.0, 0.0, -1.0)).normalized(); // normalized surface normal
        return 0.5 * ColorRGB::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * ColorRGB::new(1.0, 1.0, 1.0) + t * ColorRGB::new(0.5, 0.7, 1.0)

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
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

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
    for j in (0..IMAGE_HEIGHT).rev() {

        eprint!("\rScanlines completed: {:3}", IMAGE_HEIGHT - j );
        stderr().flush().unwrap();


        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, 
                            lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\nDone.");

    
}
