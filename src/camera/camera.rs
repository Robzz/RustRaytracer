use ray::Ray;
use std::fmt::Debug;

pub trait Camera: Debug {
    fn viewport(&self) -> (u32, u32);
    fn set_viewport(&mut self, viewport: (u32, u32));

    fn pixel_ray(&self, coords: (u32, u32)) -> Option<Ray>;
}
