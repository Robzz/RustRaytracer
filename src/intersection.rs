use nalgebra::*;
use objects::*;
use ray::Ray;
use util::filter_nones;

#[derive(Clone)]
pub struct Intersection<'a> {
    pub position: Point3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>,
    pub object: &'a Object
}

impl<'a> Intersection<'a> {
    pub fn new(position: Point3<f64>, distance: f64, normal: Vector3<f64>,
               object: &'a Object) -> Intersection<'a> {
        Intersection { position: position, distance: distance, normal: normal,
                       object: object }
    }
}

pub type HitRecord = (Point3<f64>, f64, Vector3<f64>);

pub fn closest_intersection(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections.into_iter().min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
}

fn closest_hit(hits: Vec<HitRecord>) -> Option<HitRecord> {
    hits.into_iter().min_by(|h1, h2| h1.1.partial_cmp(&h2.1).unwrap())
}

pub fn ray_face(ray: &Ray, face: &Face) -> Option<HitRecord> {
    match ray.intersects_face(face) {
        Some((p, d)) => {
            Some((p, d, face.normal()))
        }
        None => None
    }
}

pub fn ray_box(ray: &Ray, _box: &Box) -> Option<HitRecord> {
    let hits = vec!(ray_face(ray, &_box.top),
                    ray_face(ray, &_box.bottom),
                    ray_face(ray, &_box.left),
                    ray_face(ray, &_box.right),
                    ray_face(ray, &_box.front),
                    ray_face(ray, &_box.back));
    closest_hit(filter_nones(hits))
}

#[cfg(test)]
mod tests {
    use super::*;
    use objects::Face;
    use num_traits::{One, Zero};
    use material::Simple;
    use image::Rgb;
    use std::boxed::Box as StdBox;
    use std::f64::consts::PI;

    #[test]
    fn test_new_intersection() {
        let pos = Point3::new(0., 0., 0.);
        let d = 5.;
        let f = Face::new(1., 1., Isometry3::one(), StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let obj = Object::from_surface(Surface::from_face(f.clone()));
        let i = Intersection::new(pos, d, f.normal(), &obj);
        assert!(i.position == pos);
        assert!(i.distance == d);
    }

    #[test]
    fn test_ray_face_intersects() {
        let f = Face::new(3., 3., Isometry3::new(Vector3::new(0., 0., -5.),
                                                 Vector3::zero()),
                          StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let i_opt = ray_face(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z()), &f);
        assert!(i_opt.is_some());
        let i = i_opt.unwrap();
        assert!(i.0.approx_eq(&Point3::new(0., 0., -5.,)));
        assert!(i.1.approx_eq(&5.));
    }

    #[test]
    fn test_face_no_intersects() {
        let f = Face::new(3., 3., Isometry3::from_rotation_matrix(Vector3::new(2., 0., -5.),
                                                                  Rotation3::one()),
                          StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let i_opt = ray_face(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z()), &f);
        assert!(i_opt.is_none());
    }

    #[test]
    fn test_face_no_intersects_back() {
        let f = Face::new(3., 3., Isometry3::new(Vector3::new(0., 0., -5.),
                                                 Vector3::y() * PI),
                          StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let i_opt = ray_face(&Ray::new(Point3::new(0., 0., 0.), -Vector3::z()), &f);
        assert!(i_opt.is_none());
    }

    #[test]
    fn test_ray_box_intersects_straight() {
        let mat = Simple::new(Rgb { data: [1.0, 1.0, 1.0] });
        let b = Box::new(Vector3::one(),
                         Isometry3::new(Vector3::z() * 5., Vector3::zero()),
                         StdBox::new(mat));
        let ray = Ray::new(Point3::new(0., 0., 0.), Vector3::z());
        let inter_opt = ray_box(&ray, &b);
        assert!(inter_opt.is_some());
        if let Some(i) = inter_opt {
            assert!(i.0.approx_eq(&Point3::new(0., 0., 4.5)));
            assert!(i.1.approx_eq(&4.5));
        }
    }

    #[test]
    fn test_ray_box_intersects_translated() {
        let mat = Simple::new(Rgb { data: [1.0, 1.0, 1.0] });
        let b = Box::new(Vector3::one(),
                         Isometry3::new(Vector3::new(5., 5., 5.), Vector3::zero()),
                         StdBox::new(mat));
        let ray = Ray::new(Point3::new(5., 5., 0.), Vector3::z());
        let inter_opt = ray_box(&ray, &b);
        assert!(inter_opt.is_some());
        if let Some(i) = inter_opt {
            assert!(i.0.approx_eq(&Point3::new(5., 5., 4.5)));
            assert!(i.1.approx_eq(&4.5));
        }
    }
}
