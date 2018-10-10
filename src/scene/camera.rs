use na::{Point3, Vector3, Matrix4, Perspective3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub fovy: f64,
    pub pos: Point3<f64>,
    pub lookat: Vector3<f64>,
    pub vup: Vector3<f64>,
}

fn camInvProjection(cam: &Camera) -> Matrix4<f64> {
    Matrix4::new_observer_frame(&cam.pos, &(cam.pos + cam.lookat), &cam.vup)
}

fn camView(cam: &Camera) -> Perspective3<f64> {
    Perspective3::new(cam.aspect_ratio, cam.fovy, 1.0, 1000.0)
}

#[test]
fn test_nalgebra() {
    let o = Point3::new(2.4, 3.1, -7.0);
    let v = Vector3::new(1.0, 0.0, -1.0);
    let m = Matrix4::new_observer_frame(&o, //Camera to World coord
        &(o + v),
        &(Vector3::new(0.0, 1.0, 0.0) + v));
    println!("Test: {}", m);
}
