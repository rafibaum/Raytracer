use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod hittable;
mod ray;
mod sphere;
mod vec;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let file_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(file_writer, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap().into_stream_writer();

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut world = Vec::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for j in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = (IMAGE_HEIGHT - j - 1) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&ray, &world);

            let r = (255.999 * color.x) as u8;
            let g = (255.999 * color.y) as u8;
            let b = (255.999 * color.z) as u8;
            writer.write_all(&[r, g, b]).unwrap();
        }
    }

    writer.finish().unwrap();
    println!("Done!");
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        return (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = (unit_direction.y + 1.0) * 0.5;
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
