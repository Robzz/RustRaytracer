use nalgebra::*;
use material::Material;
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

    pub fn random_on_face(&self) -> Point3<f64> {
        use rand::distributions::*;
        use rand::*;
        let mut rng = thread_rng();
        let w = self.width / 2.;
        let h = self.height / 2.;
        let x = Range::new(-w, w).ind_sample(&mut rng);
        let y = Range::new(-h, h).ind_sample(&mut rng);

        self.transform.transform(&Point3::new(x, y, 0.))
    }
}

impl Clone for Face {
    fn clone(&self) -> Face {
        Face { width: self.width, height: self.height, transform: self.transform,
               material: self.material.box_clone() }
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Face) -> bool {
        self.width.approx_eq(&other.width) &&
        self.height.approx_eq(&other.height) &&
        self.transform.approx_eq(&other.transform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_traits::identities::*;
    use material::Simple;
    use image::Rgb;
    use ray::Ray;
    use intersection::ray_face;

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
    fn test_random_on_face() {
        let f = test_face();
        let p = f.random_on_face();
        assert!(-1.5 <= p.x && p.x <= 1.5 && -0.5 <= p.y && p.y <= 0.5 && p.z.approx_eq(&0.));
    }

    #[test]
    fn test_random_on_face_transformed() {
        let mut f = test_face();
        f.transform.translation = Vector3::z() * -5.;
        f.transform.rotation = Rotation3::new(Vector3::new(0., 0., 0.7));
        let p = f.random_on_face();
        let ray = Ray::between(Point3::new(0., 0., 0.), p);
        let inter_opt = ray_face(&ray, &f);
        assert!(inter_opt.is_some());
        if let Some(inter) = inter_opt {
            assert!(inter.0.approx_eq(&p));
        }
    }
}
