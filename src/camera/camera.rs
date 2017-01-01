use nalgebra::{Point3, Vector3};
use ray::Ray;

pub trait Camera {
    fn viewport(&self) -> (u32, u32);
    fn set_viewport(&mut self, viewport: (u32, u32));

    fn pixel_ray(&self, coords: (u32, u32)) -> Option<Ray>;
}
