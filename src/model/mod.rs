use na::{Point3, Vector3};

pub mod intersect;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn eval(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }

    pub fn normalize_dir(&mut self) -> &Ray {
        self.direction.normalize_mut();
        self
    }
}

/* Some concrete models */
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64
}
