use crate::vec::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod ray;
mod vec;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let file_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(file_writer, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap().into_stream_writer();

    for j in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = (IMAGE_HEIGHT - j - 1) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            writer.write_all(&[ir, ig, ib]).unwrap();
        }
    }

    writer.finish().unwrap();
    println!("Done!");
}
