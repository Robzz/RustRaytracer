use face::Face;
use ray::Ray;
use intersection::Intersection;
use std::fmt::Debug;

pub trait Surface: Debug {
    fn faces(&self) -> Vec<Face>;

    fn intersects<'a>(&'a self, ray: &Ray) -> Option<Intersection<'a>>;
}
