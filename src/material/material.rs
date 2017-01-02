use intersection::Intersection;
use scene::Scene;
use image::Rgb;
use std::fmt::Debug;

pub trait Material: Debug {
    fn shade(&self, intersect: &Intersection, scene: &Scene) -> Rgb<f64>;

    fn box_clone(&self) -> Box<Material>;
}
