use model::SceneObject;
use na::{Vector3, Point3, Unit};

use model::Sphere;
use model::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

//TODO
pub struct Intersection {
    pub distance: f64,
    pub intersect_pt: Point3<f64>,
    pub normal: Vector3<f64>,
}

//Generic intersection for a scene graph
impl Intersectable for SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            SceneObject::Group { child_nodes } => {
                child_nodes.iter()
                    .map(|x| x.intersect(ray) )
                    .filter_map(|x| x) //Filter out None and extract value from Some(x)
                    .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
            },
            SceneObject::Node { model } => (*model).intersect(ray)
        }
    }
}

//Credit: https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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

        let intersect_pt = ray.eval(distance);
        Some(Intersection {
            distance: distance,
            intersect_pt: intersect_pt,
            normal: intersect_pt - self.center
        })
    }
}
