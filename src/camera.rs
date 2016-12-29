use nalgebra::{Point3, Vector3};

pub struct Camera {
    pub position: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Camera {
    pub fn new(position: Point3<f64>, direction: Vector3<f64>) -> Camera {
        Camera { position: position, direction: direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_camera() {
        let pos = Point3::new(0., 0., 0.);
        let dir = -Vector3::z();
        let cam = Camera::new(pos, dir);
        assert!(cam.position == pos);
        assert!(cam.direction == dir);
    }
}
