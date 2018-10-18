use model::SceneObject;
use scene::camera::Camera;
use scene::light::Lighting;

pub mod camera;
pub mod light;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub lightins: Vec<Lighting>,
    pub scene_graph: SceneObject
}
