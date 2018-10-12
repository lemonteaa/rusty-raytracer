use model::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
