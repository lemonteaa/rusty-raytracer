use na::Vector3;

use model::Sphere;
use model::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

//TODO
pub struct Intersection {
    pub distance: f64,
    pub normal: Vector3<f64>,
}

//Credit: https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.norm_squared() - (adj * adj);
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }
        let thc = (r2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
           return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}
