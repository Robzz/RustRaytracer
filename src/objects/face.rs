use nalgebra::*;
use intersection::Intersection;
use ray::Ray;
use material::Material;
use objects::*;
use std::boxed::Box as StdBox;

#[derive(Debug)]
/// Represent a rectangular face.
/// The default face (i.e. with the identity transform) is considered to be
/// aligned on the XY plane, facing the Z direction (i.e. normal has positive Z),
/// centered on the origin.
pub struct Face {
    pub width: f64,
    pub height: f64,
    pub transform: Isometry3<f64>,
    pub material: StdBox<Material>
}

impl Face {
    pub fn new(width: f64, height: f64, transform: Isometry3<f64>, material: StdBox<Material>) -> Face {
        Face { width: width, height: height, transform: transform, material: material }
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.transform * Vector3::<f64>::z()
    }
}

impl Clone for Face {
    fn clone(&self) -> Face {
        Face { width: self.width, height: self.height, transform: self.transform,
               material: self.material.box_clone() }
    }
}

impl Intersectable for Face {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        match ray.intersects_face(self) {
            Some((p, d)) => {
                Some(Intersection::new(p, d, self.as_object()))
            }
            None => None
        }
    }
}

impl Surface for Face {
    fn material(&self) -> &StdBox<Material> {
        &self.material
    }

    fn box_clone(&self) -> StdBox<Surface> {
        StdBox::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_traits::identities::One;
    use material::Simple;
    use image::Rgb;

    fn test_face() -> Face {
        Face::new(3., 1., Isometry3::one(), StdBox::new(Simple::new(Rgb { data: [1., 0., 0.] })))
    }

    #[test]
    fn test_new_face() {
        let w = 3.;
        let h = 1.;
        let m = Isometry3::one();
        let mat = Simple::new(Rgb { data: [0., 0., 0.] });
        let f = Face::new(w, h, m, StdBox::new(mat));
        assert!(f.width == w);
        assert!(f.height == h);
        assert!(f.transform == m);
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

    #[test]
    fn test_surface_intersects() {
        let f = Face::new(3., 3., Isometry3::from_rotation_matrix(Vector3::new(0., 0., -5.),
                                                                  Rotation3::one()),
                          StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let i_opt = f.intersects(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z()));
        assert!(i_opt.is_some());
        let i = i_opt.unwrap();
        assert!(i.position.approx_eq(&Point3::new(0., 0., -5.,)));
        assert!(i.distance.approx_eq(&5.));
    }

    #[test]
    fn test_surface_no_intersects() {
        let f = Face::new(3., 3., Isometry3::from_rotation_matrix(Vector3::new(2., 0., -5.),
                                                                  Rotation3::one()),
                          StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let i_opt = f.intersects(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z()));
        assert!(i_opt.is_none());
    }

    #[test]
    fn test_surface_no_intersects_behind() {
        let f = Face::new(3., 3., Isometry3::from_rotation_matrix(Vector3::new(0., 0., 5.),
                                                                  Rotation3::one()),
                          StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        assert!(f.intersects(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z())).is_none());
    }
}
