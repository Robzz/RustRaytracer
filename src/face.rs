use nalgebra::{Matrix4};
use surface::Surface;

#[derive(Debug, Clone, PartialEq)]
/// Represent a rectangular face.
/// The default face (i.e. with the identity transform) is considered to be
/// aligned on the XY plane, facing the -Z direction, centered on the origin.
pub struct Face {
    pub width: f64,
    pub height: f64,
    pub transform: Matrix4<f64>
}

impl Face {
    pub fn new(width: f64, height: f64, transform: Matrix4<f64>) -> Face {
        Face { width: width, height: height, transform: transform }
    }
}

impl Surface for Face {
    fn faces(&self) -> Vec<Face> {
        vec![self.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Eye;

    fn test_face() -> Face {
        Face::new(3., 1., Matrix4::new_identity(4))
    }

    #[test]
    fn test_new_face() {
        let w = 3.;
        let h = 1.;
        let m = Matrix4::new_identity(4);
        let f = Face::new(w, h, m);
        assert!(f.width == w);
        assert!(f.height == h);
        assert!(f.transform == m);
    }

    #[test]
    fn test_surface_impl() {
        let f = test_face();
        let mut v = f.faces();
        assert!(v.len() == 1);
        assert!(v.pop() == Some(f));
    }
}
