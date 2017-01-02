use nalgebra::*;
use surface::Surface;
use intersection::Intersection;
use ray::Ray;
use material::Simple;

#[derive(Debug, Clone, PartialEq)]
/// Represent a rectangular face.
/// The default face (i.e. with the identity transform) is considered to be
/// aligned on the XY plane, facing the Z direction (i.e. normal has positive Z),
/// centered on the origin.
pub struct Face {
    pub width: f64,
    pub height: f64,
    pub transform: Isometry3<f64>,
    pub material: Simple
}

impl Face {
    pub fn new(width: f64, height: f64, transform: Isometry3<f64>, material: Simple) -> Face {
        Face { width: width, height: height, transform: transform, material: material }
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
        let p = self.transform.transform(&Point3::new(0., 0., 0.));
        let n = self.normal();
        let d = dot(&ray.direction, &n);
        match d.approx_eq(&0.) {
            true => None,
            false => {
                // Ray is not parallel to plane
                // Find if the intersection is in the positive direction
                let t = dot(&(p - ray.origin), &n) / d;
                match t < 0. {
                    true => None,
                    false => {
                        // Find the intersection point on the face's plane and
                        // make sure it's within the face
                        let i_world = ray.origin + t * ray.direction;
                        let i_local = self.transform.inverse().unwrap().transform(&i_world);
                        match (abs(&i_local.x) <= (self.width / 2.)) && (abs(&i_local.y) <= (self.height / 2.)) {
                            true => Some(Intersection::new(i_world, norm(&(i_world - ray.origin)), self as &Surface)),
                            false => None
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use num_traits::identities::One;
    use image::Rgb;

    fn test_face() -> Face {
        Face::new(3., 1., Isometry3::one(), Simple::new(Rgb { data: [1., 0., 0.] }))
    }

    #[test]
    fn test_new_face() {
        let w = 3.;
        let h = 1.;
        let m = Isometry3::one();
        let mat = Simple::new(Rgb { data: [0., 0., 0.] });
        let f = Face::new(w, h, m, mat);
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

    #[test]
    fn test_surface_intersects() {
        let f = Face::new(3., 3., Isometry3::from_rotation_matrix(Vector3::new(0., 0., -5.),
                                                                  Rotation3::one()),
                          Simple::new(Rgb { data: [0., 0., 0.] }));
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
                          Simple::new(Rgb { data: [0., 0., 0.] }));
        let i_opt = f.intersects(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z()));
        assert!(i_opt.is_none());
    }

    #[test]
    fn test_surface_no_intersects_behind() {
        let f = Face::new(3., 3., Isometry3::from_rotation_matrix(Vector3::new(0., 0., 5.),
                                                                  Rotation3::one()),
                          Simple::new(Rgb { data: [0., 0., 0.] }));
        assert!(f.intersects(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z())).is_none());
    }
}
