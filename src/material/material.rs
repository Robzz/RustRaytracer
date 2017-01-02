use intersection::Intersection;
use scene::Scene;
use image::Rgb;
use std::fmt::Debug;

pub trait Material: Debug {
    fn shade<'a>(&self, intersect: &'a Intersection<'a>, scene: &Scene) -> Rgb<f64>;

    fn box_clone(&self) -> Box<Material>;
}
