use intersection::Intersection;
use scene::Scene;
use image::Rgb;
use camera::Camera;
use std::fmt::Debug;

pub trait Material: Debug {
    fn shade<'a, C: Camera>(&self, intersect: &'a Intersection<'a>, scene: &Scene<C>) -> Rgb<f64>;
}
