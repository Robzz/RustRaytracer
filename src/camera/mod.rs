mod orthographic;
mod perspective;

pub trait Camera {
    fn viewport(&self) -> (u32, u32);
    fn set_viewport(&mut self, width: u32, height: u32);
}

pub use self::orthographic::*;
pub use self::perspective::*;
