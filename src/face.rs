use nalgebra::{Matrix4};

pub struct Face {
    pub width: f64,
    pub height: f64,
    pub transform: Matrix4<f64>
}

impl Face  {
    pub fn new(width: f64, height: f64, transform: Matrix4<f64>) -> Face {
        Face { width: width, height: height, transform: transform }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Eye;

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
}
