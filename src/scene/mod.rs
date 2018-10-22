use model::SceneObject;
use scene::camera::Camera;
use scene::light::Lighting;
use util::{Color, ColorSetting};

use std::path::Path;
use image::{DynamicImage, open};

use na::Vector3;

pub mod camera;
pub mod light;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub lightins: Vec<Lighting>,
    pub scene_graph: SceneObject,
    pub background: Background,
    pub color_setting: ColorSetting
}

pub enum Background {
    PlainColor { color : Color },
    Image { img : DynamicImage }
}

impl Background {
    pub fn from_image(img_path: &str) -> Background {
        Background::Image { img : open(&Path::new(img_path)).unwrap() }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        match *self {
            Background::PlainColor { color } => color,
            Background::Image { ref img } => Color { intensities: Vector3::zeros() }
        }
    }
}
