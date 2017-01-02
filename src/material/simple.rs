use intersection::Intersection;
use scene::Scene;
use image::Rgb;
use camera::Camera;
use material::Material;

#[derive(Debug, Clone, PartialEq)]
pub struct Simple {
    color: Rgb<f64>
}

impl Simple {
    pub fn new(color: Rgb<f64>) -> Simple {
        Simple { color: color }
    }
}

impl Material for Simple {
    fn shade<'a, C: Camera>(&self, _: &'a Intersection<'a>, _: &Scene<C>) -> Rgb<f64> {
        self.color
    }
}
