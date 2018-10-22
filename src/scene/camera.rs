use model::Ray;

use na::{Point3, Vector3, Matrix4, Perspective3};

use alga::linear::Transformation;

pub struct Camera {
    pub aspect_ratio: f64,
    pub fovy: f64,
    pub pos: Point3<f64>,
    pub lookat: Vector3<f64>,
    pub vup: Vector3<f64>,

    //Accept overhead for copying matrix; cache locality seems to imply
    //the desired optimization won't work anyway
    pub inv_view: Option<Matrix4<f64>>,
    pub projection: Option<Perspective3<f64>>
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            fovy: 3.14/2.0,
            pos: Point3::new(0.0, 0.0, 1.0),
            lookat: Vector3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            inv_view: None,
            projection: None
        }
    }
}

impl Camera {
    /*
     * Prefer explicit initialization over implicit meomization
     * Because the default memory model for a cache *under concurrency*
     * does not pass Rust's borrow checker
     * (Rust ensure both memory safety and safety under concurrency by default)
     */
    pub fn initialize(&mut self) {
        //The fcn maps view dir to +ve z axis, but we want -ve z axis hence the sign
        self.inv_view = Some(Matrix4::new_observer_frame(&self.pos, &(self.pos - self.lookat), &self.vup));
        self.projection = Some(Perspective3::new(self.aspect_ratio, self.fovy, 1.0, 1000.0));
    }

    pub fn get_inv_view(&self) -> Option<Matrix4<f64>> {
        self.inv_view
    }

    pub fn get_projection(&self) -> Option<Perspective3<f64>> {
        self.projection
    }

    fn get_raw_ray_direction(&self, x_ndc: f64, y_ndc: f64) -> Vector3<f64> {
        let projection = self.get_projection().unwrap();
        let inv_view = self.get_inv_view().unwrap();
        let p = projection.unproject_point(&Point3::new(x_ndc, y_ndc, -1.0));
        let d: Vector3<f64> = inv_view.transform_point(&p) - self.pos;
        //Unit::new_normalize(d)
        d
    }

    pub fn get_ray(&self, x_ndc: f64, y_ndc: f64) -> Ray {
        let mut r = Ray {
            origin: self.pos,
            direction: self.get_raw_ray_direction(x_ndc, y_ndc)
        };
        r.normalize_dir();
        r
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
        .. Default::default()
    };
    x
}

#[test]
fn test_camera_gen_ray() {
    let mut c = test_create_camera();
    c.initialize();
    //Debug
    //let p = c.get_projection().unproject_point(&Point3::new(-1.0, -1.0, 1.0));
    //println!("debug, after unproject: {}", p);
    println!("Test cam ray: {}", c.get_raw_ray_direction(-1.0, -1.0))
}
