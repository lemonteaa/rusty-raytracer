extern crate nalgebra as na;

extern crate image;

extern crate alga;

use image::GenericImage;
use image::Pixel;
use image::{DynamicImage, Rgba};

use std::fs::File;

use na::{Point3, Vector3, Matrix4};

pub mod scene;

use scene::camera::Camera;

fn render() -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(800, 600);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..800 {
        for y in 0..600 {
            image.put_pixel(x, y, black)
        }
    }
    image
}

fn main() {
    println!("Learning Raytracer");
    let img = render();
    let mut out = File::create("out.png").unwrap();
    img.write_to(&mut out, image::PNG).expect("Saving image failed");
    println!("Done. Exiting...");
}
