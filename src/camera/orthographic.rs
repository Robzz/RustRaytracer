use nalgebra::*;
use camera::Camera;
use ray::Ray;

#[derive(Debug)]
pub struct Orthographic {
    pub viewport: (u32, u32),
    pub plane_size: (f64, f64),
    pub transform: Isometry3<f64>
}

impl Orthographic {
    pub fn new(viewport: (u32, u32), plane_size: (f64, f64),
               transform: Isometry3<f64>) -> Orthographic {
        Orthographic { viewport: viewport, plane_size: plane_size,
                       transform: transform }
    }
}

impl Camera for Orthographic {
    fn viewport(&self) -> (u32, u32) { self.viewport }
    fn set_viewport(&mut self, viewport: (u32, u32)) { self.viewport = viewport; }

    fn pixel_ray(&self, coords: (u32, u32)) -> Option<Ray> {
        let (pw, ph) = self.plane_size;
        let (vw, vh) = self.viewport;
        let (x, y) = coords;
        if x < vw && y < vh {
            let right = self.transform * Vector3::x();
            let up = self.transform * Vector3::y();
            let forward = self.transform * Vector3::z();
            let cam_x = -pw / 2. + (x as f64) / (vw as f64);
            let cam_y = -ph / 2. + (y as f64) / (vh as f64);
            Some(Ray::new((self.transform.translation + cam_x * right + cam_y * up).to_point(),
                           forward.normalize()))
        }
        else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_traits::Zero;

    #[test]
    fn test_new_camera() {
        let transform = Isometry3::new(Vector3::new(0., 0., 0.), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        assert!(cam.viewport() == (800, 600));
        assert!(cam.transform == transform);
    }

    #[test]
    fn test_pixel_ray_bottom_left() {
        let transform = Isometry3::new(Vector3::new(0., 0., 0.), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        let ray_opt = cam.pixel_ray((0, 0));
        assert!(ray_opt.is_some());
        let ray = ray_opt.unwrap();
        assert!(ray.origin.approx_eq(&Point3::new(-50., -50., 0.)));
        assert!(ray.direction.approx_eq(&Vector3::z()));
    }
}
