use nalgebra::*;
use surface::Surface;
use intersection::Intersection;
use ray::Ray;

#[derive(Debug, Clone, PartialEq)]
/// Represent a rectangular face.
/// The default face (i.e. with the identity transform) is considered to be
/// aligned on the XY plane, facing the Z direction (i.e. normal has positive Z),
/// centered on the origin.
pub struct Face {
    pub width: f64,
    pub height: f64,
    pub transform: Isometry3<f64>
}

impl Face {
    pub fn new(width: f64, height: f64, transform: Isometry3<f64>) -> Face {
        Face { width: width, height: height, transform: transform }
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.transform * Vector3::<f64>::z()
    }
}

impl Surface for Face {
    fn faces(&self) -> Vec<Face> {
        vec![self.clone()]
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Stub
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_traits::identities::One;

    fn test_face() -> Face {
        Face::new(3., 1., Isometry3::one())
    }

    #[test]
    fn test_new_face() {
        let w = 3.;
        let h = 1.;
        let m = Isometry3::one();
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

    #[test]
    fn test_normal_untransformed_face() {
        let f = test_face();
        let n = f.normal();
        assert!(n.approx_eq(&Vector3::z()));
    }

    #[test]
    fn test_normal_rotated_face() {
        let mut f = test_face();
        f.transform.rotation = Rotation3::new(Vector3::y() * (PI / 2.));
        let n = f.normal();
        assert!(n.approx_eq(&Vector3::x()));
    }
}
