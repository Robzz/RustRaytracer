use objects::Face;
use material::Material;
use nalgebra::*;
use std::boxed::Box as StdBox;
use num_traits::{One, Zero};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Box {
    pub top: Face,
    pub bottom: Face,
    pub left: Face,
    pub right: Face,
    pub front: Face,
    pub back: Face,
    pub transform: Isometry3<f64>,
    pub size: Vector3<f64>,
    pub material: StdBox<Material>
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

impl PartialEq for Box {
    fn eq(&self, other: &Box) -> bool {
        self.top == other.top &&
        self.bottom == other.bottom &&
        self.left == other.left &&
        self.right == other.right &&
        self.front == other.front &&
        self.back == other.back &&
        self.transform == other.transform &&
        self.size == other.size
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
