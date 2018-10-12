use na::{Point3, Vector3, Matrix4, Perspective3, Unit};

use alga::linear::Transformation;

pub struct Camera {
    pub aspect_ratio: f64,
    pub fovy: f64,
    pub pos: Point3<f64>,
    pub lookat: Vector3<f64>,
    pub vup: Vector3<f64>,
}

impl Camera {
    pub fn get_inv_view(&self) -> Matrix4<f64> {
        //The fcn maps view dir to +ve z axis, but we want -ve z axis hence the sign
        Matrix4::new_observer_frame(&self.pos, &(self.pos - self.lookat), &self.vup)
    }

    pub fn get_projection(&self) -> Perspective3<f64> {
        Perspective3::new(self.aspect_ratio, self.fovy, 1.0, 1000.0)
    }

    pub fn get_raw_ray_direction(&self, x_ndc: f64, y_ndc: f64) -> Vector3<f64> {
        let projection = self.get_projection();
        let inv_view = self.get_inv_view();
        let p = projection.unproject_point(&Point3::new(x_ndc, y_ndc, -1.0));
        let d: Vector3<f64> = inv_view.transform_point(&p) - self.pos;
        //Unit::new_normalize(d)
        d
    }
}

#[test]
fn test_nalgebra() {
    let o = Point3::new(2.4, 3.1, -7.0);
    let v = Vector3::new(1.0, 0.0, -1.0);
    let m = Matrix4::new_observer_frame(&o, //Camera to World coord
        &(o + v),
        &(Vector3::new(0.0, 1.0, 0.0) + v));
    println!("Test observer frame: {}", m);
}

fn test_create_camera() -> Camera {
    let x: Camera = Camera {
        aspect_ratio: 16.0/9.0,
        fovy: 3.14/2.0,
        pos: Point3::new(7.3, -4.2, 8.6),
        lookat: Vector3::new(1.0, 0.2, -1.0),
        vup: Vector3::new(0.0, 1.0, 0.0),
    };
    x
}

#[test]
fn test_camera_gen_ray() {
    let c = test_create_camera();
    //Debug
    //let p = c.get_projection().unproject_point(&Point3::new(-1.0, -1.0, 1.0));
    //println!("debug, after unproject: {}", p);
    println!("Test cam ray: {}", c.get_raw_ray_direction(-1.0, -1.0))
}
