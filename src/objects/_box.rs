use objects::Face;
use material::Material;
use nalgebra::*;
use std::boxed::Box as StdBox;
use num_traits::{One, Zero};
use std::f64::consts::PI;
use objects::*;
use intersection::*;
use ray::Ray;
use std::cmp::PartialOrd;
use util::filter_nones;

pub struct Box {
    top: Face,
    bottom: Face,
    left: Face,
    right: Face,
    front: Face,
    back: Face,
    transform: Isometry3<f64>,
    size: Vector3<f64>,
    material: StdBox<Material>
}

impl Box {
    pub fn new(size: Vector3<f64>, transform: Isometry3<f64>, material: StdBox<Material>) -> Box {
        let top = Face::new(size.x, size.z, Isometry3::one(), material.box_clone());
        let bottom = Face::new(size.x, size.z, Isometry3::one(), material.box_clone());
        let left = Face::new(size.z, size.y, Isometry3::one(), material.box_clone());
        let right = Face::new(size.z, size.y, Isometry3::one(), material.box_clone());
        let front = Face::new(size.x, size.y, Isometry3::one(), material.box_clone());
        let back = Face::new(size.x, size.y, Isometry3::one(), material.box_clone());
        let mut b = Box { top: top, bottom: bottom,
                          left: left, right: right, front: front, back: back,
                          transform: Isometry3::one(),
                          size: size, material: material };
        b.set_transform(transform);
        b
    }

    pub fn set_transform(&mut self, transform: Isometry3<f64>) {
        let top_transform = Isometry3::new(self.size.y / 2. * Vector3::y(), Vector3::x() * -PI / 2.);
        self.top.transform = top_transform.append_transformation(&transform);
        let bottom_transform = Isometry3::new(-self.size.y / 2. * Vector3::y(), Vector3::x() * PI / 2.);
        self.bottom.transform = bottom_transform.append_transformation(&transform);
        let left_transform = Isometry3::new(-self.size.x / 2. * Vector3::x(), Vector3::y() * -PI / 2.);
        self.left.transform = left_transform.append_transformation(&transform);
        let right_transform = Isometry3::new(self.size.x / 2. * Vector3::x(), Vector3::y() * PI / 2.);
        self.right.transform = right_transform.append_transformation(&transform);
        let front_transform = Isometry3::new(self.size.z / 2. * Vector3::z(), Vector3::zero());
        self.front.transform = front_transform.append_transformation(&transform);
        let back_transform = Isometry3::new(-self.size.z / 2. * Vector3::z(), Vector3::y() * PI);
        self.back.transform = back_transform.append_transformation(&transform);
    }
}

impl Clone for Box {
    fn clone(&self) -> Box {
        Box { top: self.top.clone(), bottom: self.bottom.clone(),
              left: self.left.clone(), right: self.right.clone(),
              front: self.front.clone(), back: self.back.clone(),
              transform: self.transform, size: self.size,
              material: self.material.box_clone() }
    }
}

impl Surface for Box {
    fn material<'a>(&'a self) -> &'a StdBox<Material> {
        &self.material
    }

    fn box_clone(&self) -> StdBox<Surface> {
        StdBox::new(self.clone())
    }
}

impl Intersectable for Box {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let intersections = vec!(self.top.intersects(ray),
                                 self.bottom.intersects(ray),
                                 self.left.intersects(ray),
                                 self.right.intersects(ray),
                                 self.front.intersects(ray),
                                 self.back.intersects(ray));
        closest_intersection(filter_nones(intersections))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use material::Simple;
    use ray::Ray;
    use image::Rgb;

    #[test]
    fn test_intersects_straight() {
        let mat = Simple::new(Rgb { data: [1.0, 1.0, 1.0] });
        let b = Box::new(Vector3::one(),
                         Isometry3::new(Vector3::z() * 5., Vector3::zero()),
                         StdBox::new(mat));
        let ray = Ray::new(Point3::new(0., 0., 0.), Vector3::z());
        let inter_opt = b.intersects(&ray);
        assert!(inter_opt.is_some());
        if let Some(i) = inter_opt {
            assert!(i.distance.approx_eq(&4.5));
            assert!(i.position.approx_eq(&Point3::new(0., 0., 4.5)));
        }
    }

    #[test]
    fn test_intersects_translated() {
        let mat = Simple::new(Rgb { data: [1.0, 1.0, 1.0] });
        let b = Box::new(Vector3::one(),
                         Isometry3::new(Vector3::new(5., 5., 5.), Vector3::zero()),
                         StdBox::new(mat));
        let ray = Ray::new(Point3::new(5., 5., 0.), Vector3::z());
        let inter_opt = b.intersects(&ray);
        assert!(inter_opt.is_some());
        if let Some(i) = inter_opt {
            assert!(i.distance.approx_eq(&4.5));
            assert!(i.position.approx_eq(&Point3::new(5., 5., 4.5)));
        }
    }
}
