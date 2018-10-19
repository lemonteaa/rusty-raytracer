extern crate nalgebra as na;

extern crate image;

extern crate alga;

extern crate rand;

use image::GenericImage;
use image::Pixel;
use image::{DynamicImage, Rgba};

use std::fs::File;

use na::{Point3, Vector3, Matrix4};

use scene::Scene;
use util::{Color, ColorSetting};

pub mod scene;
pub mod model;
pub mod util;
pub mod core;

fn basic_render(scene: Scene, sample_cnt: usize, color_setting: ColorSetting) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let mut samples: Vec<Option<Color>> = Vec::with_capacity(sample_cnt);
            for _ in 0..sample_cnt {
                let ray = core::gen_ray_antialiased_sample(x, y, &scene);
                samples.push(core::trace(&ray, &scene.scene_graph, &scene.lightins));
            }
            let color = core::antialias_average_color(samples.into_iter(),
                            scene.background.get_pixel(x, y));
            image.put_pixel(x, y, color.to_rgb(&color_setting));
        }
    }
    image
}

fn dummy_render() -> DynamicImage {
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
    let img = dummy_render();
    let mut out = File::create("out.png").unwrap();
    img.write_to(&mut out, image::PNG).expect("Saving image failed");
    println!("Done. Exiting...");
}
