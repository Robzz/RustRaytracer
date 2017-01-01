use nalgebra::{Point3, Vector3};

pub trait Camera {
    fn viewport(&self) -> (u32, u32);
    fn set_viewport(&mut self, width: u32, height: u32);
}
