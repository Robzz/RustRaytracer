use camera::Camera;
use nalgebra::*;
use ray::Ray;

#[derive(Debug, PartialEq)]
/// Clipping plane z as fixed at 1
/// The default camera faces the -Z axis
pub struct Perspective {
    pub viewport: (u32, u32),
    pub fov: (f64, f64),
    pub transform: Isometry3<f64>
}

impl Perspective {
    pub fn new(viewport: (u32, u32), fov: (f64, f64), transform: Isometry3<f64>) -> Perspective {
        Perspective { viewport: viewport, fov: fov, transform: transform }
    }
}

impl Camera for Perspective {
    fn viewport(&self) -> (u32, u32) { self.viewport }
    fn set_viewport(&mut self, viewport: (u32, u32)) { self.viewport = viewport; }

    fn pixel_ray(&self, coords: (f64, f64)) -> Option<Ray> {
        let (xu, yu) = coords;
        let (x, y) = (xu as f64, yu as f64);
        let (wu, hu) = self.viewport;
        let (w, h) = (wu as f64, hu as f64);
        match x < w && y < h {
            false => None,
            true => {
                let (fov_x, fov_y) = self.fov;
                let (xf, yf) = (x / (w - 1.) - 0.5, y / (h - 1.) - 0.5);
                let (theta_x, theta_y) = (xf * fov_x, yf * fov_y);
                let direction = self.transform * Vector3::new(theta_x.tan(), theta_y.tan(), -1.).normalize();
                Some(Ray::new(self.transform.transform(&Point3::new(0., 0., 0.)),
                              direction))
            }
        }
    }

    fn eye_position(&self) -> Point3<f64> {
        self.transform.transform(&Point3::new(0., 0., 0.))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::{One, Float};
    use std::f64::consts::PI;

    #[test]
    fn test_pixel_ray_bottom_left() {
        let cam = Perspective::new((800, 600), ((90.).to_radians(), (90.).to_radians()), Isometry3::one());
        let ray_opt = cam.pixel_ray((0., 0.));
        let ray = ray_opt.unwrap();
        assert!(ray.origin.approx_eq(&Point3::new(0., 0., 0.)));
        let f = (-PI / 4.).tan();
        assert!(ray.direction.approx_eq(&Vector3::new(f, f, -1.).normalize()));
    }

    #[test]
    fn test_pixel_ray_top_right() {
        let cam = Perspective::new((800, 600), ((90.).to_radians(), (90.).to_radians()), Isometry3::one());
        let ray_opt = cam.pixel_ray((799., 599.));
        let ray = ray_opt.unwrap();
        assert!(ray.origin.approx_eq(&Point3::new(0., 0., 0.)));
        let f = (PI / 4.).tan();
        assert!(ray.direction.approx_eq(&Vector3::new(f, f, -1.).normalize()));
    }
}
