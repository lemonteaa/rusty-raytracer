use image::{Rgba, Pixel};
use na::{Vector3, Point3, clamp};

use std::ops::{Add, Mul};

//Unfortunately not possible atm: https://discourse.nphysics.org/t/creation-of-constant-vectors/113/2
//pub const BLACK : Color = Color { intensities: Vector3::zeros() };

//Quick fix for operator, https://stackoverflow.com/questions/37313335/cannot-move-out-of-borrowed-content-with-operator-overloading
//Ideally https://github.com/rust-lang/rust/issues/21188
#[derive(Copy, Clone)]
pub struct Color {
    pub intensities : Vector3<f64>
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
    pub fn to_rgb(&self, setting : &ColorSetting) -> Rgba<u8> {
        let r = clamp(gamma_encode(self.intensities.x,
                    setting.gamma_a, setting.gamma_power),
                    0.0, setting.max_intensity) / setting.max_intensity;
        let g = clamp(gamma_encode(self.intensities.y,
                    setting.gamma_a, setting.gamma_power),
                    0.0, setting.max_intensity) / setting.max_intensity;
        let b = clamp(gamma_encode(self.intensities.z,
                    setting.gamma_a, setting.gamma_power),
                    0.0, setting.max_intensity) / setting.max_intensity;
        Rgba::from_channels((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, 255)
    }

    pub fn from_rgba(rgba: Rgba<u8>, setting: &ColorSetting) -> Color {
        let r = (rgba.data[0] as f64) / 255.0;
        let g = (rgba.data[1] as f64) / 255.0;
        let b = (rgba.data[2] as f64) / 255.0;
        let am = 1.0 / setting.gamma_a;
        let power = 1.0 / setting.gamma_power;
        Color { intensities:
            Vector3::new(gamma_encode(r * setting.max_intensity, am, power),
                         gamma_encode(g * setting.max_intensity, am, power),
                         gamma_encode(b * setting.max_intensity, am, power)) }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other : Color) -> Color {
        Color { intensities : self.intensities + other.intensities }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, scalar: f64) -> Color {
        Color { intensities : scalar * self.intensities }
    }
}

impl Mul<Point3<f64>> for Color {
    type Output = Color;
    fn mul(self, other: Point3<f64>) -> Color {
        Color { intensities :
            Vector3::new(
                self.intensities.x * other.x,
                self.intensities.y * other.y,
                self.intensities.z * other.z) }
    }
}

pub fn get_ndc(x: f64, y: f64, width: u32, height: u32) -> (f64, f64) {
    let x_ndc = 2.0 * (x / (width as f64)) - 1.0;
    let y_ndc = 2.0 * (y / (height as f64)) - 1.0;
    (x_ndc, y_ndc)
}

#[test]
fn test_gamma() {
    assert_eq!(8.0, gamma_encode(4.0, 0.25, 2.5));
}
