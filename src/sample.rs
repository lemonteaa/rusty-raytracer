use model::{Material, SceneObject, Sphere};
use scene::camera::Camera;
use scene::light::{Lighting, LightingType};
use scene::{Scene, Background};
use util::{Color, ColorSetting};

use std::rc::Rc;

use image::{Rgba, Pixel};
use na::{Vector3, Point3};

pub enum SampleSceneType {
    Ball,
    CornellBox
}

pub trait SampleScene {
    fn gen_scene(&self) -> Scene;
}

pub struct BallScene {
    //TODO
}

impl SampleScene for BallScene {
    fn gen_scene(&self) -> Scene {
        let mut cam: Camera = Camera {
            aspect_ratio: 4.0/3.0,
            fovy: 3.14/2.0,
            //pos: Point3::new(7.3, -4.2, 8.6),
            //lookat: Vector3::new(1.0, 0.2, -1.0),
            //vup: Vector3::new(0.0, 1.0, 0.0),
            .. Default::default()
        };
        cam.initialize();
        let color_setting = ColorSetting {
            gamma_a: 1.0,
            gamma_power: 0.25,
            max_intensity: 1.0
        };
        let mat_a = Rc::new(Material {
            ambient_reflectance: 0.1,
            diffuse_reflectance: Point3::new(1.0, 1.0, 1.0)
        });
        Scene {
            width: 800,
            height: 600,
            camera: cam,
            lightins: vec![
                Lighting {
                    light_pos: LightingType::Ambient,
                    color: Color::from_rgba(Rgba::from_channels(255,255,255,255), &color_setting)
                },
                Lighting {
                    light_pos: LightingType::Directional {
                        dir: Vector3::new(0.0, -1.0, -1.0).normalize()
                    },
                    color: Color::from_rgba(Rgba::from_channels(255,255,255,255), &color_setting)
                }
            ],
            scene_graph: SceneObject::Group {
                child_nodes: vec![
                    SceneObject::Node { model: Box::new(Sphere {
                        material: Rc::clone(&mat_a),
                        center: Point3::new(-2.0, 0.0, -4.0),
                        radius: 1.0
                    }) },
                    SceneObject::Node { model: Box::new(Sphere {
                        material: Rc::clone(&mat_a),
                        center: Point3::new(2.0, 0.0, -4.0),
                        radius: 1.0
                    }) }
                ]
            },
            background: Background::PlainColor {
                color: Color::from_rgba(Rgba::from_channels(165, 200, 255, 255),
                                        &color_setting)
            },
            color_setting: color_setting
        }
    }
}
