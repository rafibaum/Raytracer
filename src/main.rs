use crate::ray::Ray;
use crate::vec::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod ray;
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

    for j in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = (IMAGE_HEIGHT - j - 1) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&ray);

            let r = (255.999 * color.x) as u8;
            let g = (255.999 * color.y) as u8;
            let b = (255.999 * color.z) as u8;
            writer.write_all(&[r, g, b]).unwrap();
        }
    }

    writer.finish().unwrap();
    println!("Done!");
}

fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_dir = ray.direction.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
