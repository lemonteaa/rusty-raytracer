use na::{Point3, clamp};

pub struct Color {
    pub intensities : Point3<f64>
}

pub struct ColorSetting {
    pub gamma_a : f64,
    pub gamma_power : f64,
    pub max_intensity : f64
}

pub fn gamma_encode(x: f64, a: f64, p: f64) -> f64 {
    a * x.powf(p)
}

impl Color {
    pub fn to_rgb(&self, setting : &ColorSetting) {
        let r = clamp(gamma_encode(self.intensities.x,
                    setting.gamma_a, setting.gamma_power),
                    0.0, setting.max_intensity) / setting.max_intensity;
        let g = clamp(gamma_encode(self.intensities.y,
                    setting.gamma_a, setting.gamma_power),
                    0.0, setting.max_intensity) / setting.max_intensity;
        let b = clamp(gamma_encode(self.intensities.z,
                    setting.gamma_a, setting.gamma_power),
                    0.0, setting.max_intensity) / setting.max_intensity;
    }
}

#[test]
fn test_gamma() {
    assert_eq!(8.0, gamma_encode(4.0, 0.25, 2.5));
}
