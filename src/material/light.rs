use image::Rgb;
use intersection::Intersection;
use scene::Scene;
use material::Material;

#[derive(Debug, Clone)]
pub struct LightMaterial {
    color: Rgb<f64>
}

impl LightMaterial {
    pub fn new(color: Rgb<f64>) -> LightMaterial {
        LightMaterial { color: color }
    }

    pub fn to_material(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}

impl Material for LightMaterial {
    fn shade<'a>(&self, _: &'a Intersection<'a>, _: &Scene) -> Rgb<f64> {
        self.color
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
