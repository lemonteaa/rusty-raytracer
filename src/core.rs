use na::{Vector3, dot};
use scene::light::{Lighting, LightingType};
use model::{SceneObject, Ray};
use model::intersect::{Intersection, Intersectable};
use util::Color;

pub fn trace(ray : &Ray, scene : &SceneObject, lightings : &Vec<Lighting>) -> Option<Color> {
    match scene.intersect(ray) {
        None => None,
        Some(int) => Some(lighting_model(&int, scene, lightings))
    }
}

pub fn lighting_model(intersection : &Intersection,
    scene : &SceneObject, lightings : &Vec<Lighting>) -> Color {
    let mut c = Color { intensities: Vector3::zeros() };
    for lighting in lightings {
        let ref l = lighting.light_pos;
        let lc = lighting.color;
        let inc_v = l.get_incident_vec(intersection.intersect_pt);
        let p = dot(&inc_v, &intersection.normal);
        match l {
            LightingType::Ambient =>
                { c = c + lc * intersection.material.ambient_reflectance },
            LightingType::Directional {dir} =>
                { c = c + lc * intersection.material.diffuse_reflectance * p },
            LightingType::Point {loc} =>
                { c = c + lc * intersection.material.diffuse_reflectance * p }
        }
    }
    c
}
