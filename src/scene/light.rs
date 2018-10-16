use na::{Vector3, Point3, Unit};

use ::util::Color;

pub struct Lighting {
    pub light_pos : LightingType,
    pub color : Color
}

pub enum LightingType {
    Ambient,
    Directional { dir : Vector3<f64> },
    Point { loc : Point3<f64> }
}

impl LightingType {
    pub fn get_incident_vec(&self, pos : Point3<f64>) -> Vector3<f64> {
        match *self {
            LightingType::Ambient {} => Vector3::new(0.0, 0.0, 0.0),
            LightingType::Directional { dir } => dir,
            LightingType::Point { loc } => (pos - loc)
        }
    }
}
