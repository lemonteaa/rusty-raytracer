use scene::camera::Camera;

pub mod camera;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
}
