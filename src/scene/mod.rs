use model::SceneObject;
use scene::camera::Camera;

pub mod camera;
pub mod light;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub scene_graph: SceneObject
}
