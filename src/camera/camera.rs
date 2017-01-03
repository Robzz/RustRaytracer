use ray::Ray;
use nalgebra::Point3;

pub trait Camera {
    fn viewport(&self) -> (u32, u32);
    fn set_viewport(&mut self, viewport: (u32, u32));

    fn pixel_ray(&self, coords: (f64, f64)) -> Option<Ray>;
    fn eye_position(&self) -> Point3<f64>;
}
