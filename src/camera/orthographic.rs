use nalgebra::*;
use camera::Camera;

pub struct Orthographic {
    pub view_width: u32,
    pub view_height: u32,
    pub position: Point3<f64>,
    pub direction: Vector3<f64>
}


impl Orthographic {
    pub fn new(view_width: u32, view_height: u32, position: Point3<f64>,
               direction: Vector3<f64>) -> Orthographic {
        Orthographic { view_width: view_width, view_height: view_height,
                       position: position, direction: direction }
    }
}

impl Camera for Orthographic {
    fn viewport(&self) -> (u32, u32) { (self.view_width, self.view_height) }
    fn set_viewport(&mut self, width: u32, height: u32) {
        self.view_width = width;
        self.view_height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_camera() {
        let pos = Point3::new(0., 0., 0.);
        let dir = -Vector3::z();
        let cam = Orthographic::new(800, 600, pos, dir);
        assert!(cam.viewport() == (800, 600));
        assert!(cam.position == pos);
        assert!(cam.direction == dir);
    }
}
