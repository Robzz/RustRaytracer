use intersection::Intersection;
use image::Rgb;
use material::Material;
use light::Light;
use scene::Scene;

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
    fn shade<'a>(&self, _: &'a Intersection<'a>, _: &Scene) -> Rgb<f64> {
        self.color
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
