use image::Rgb;
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
    fn ambient_color(&self) -> Rgb<f64> {
        self.color
    }

    fn diffuse_color(&self) -> Rgb<f64> {
        Rgb { data: [0., 0., 0.] }
    }

    fn specular_color(&self) -> Rgb<f64> {
        Rgb { data: [0., 0., 0.] }
    }

    fn shininess(&self) -> f64 {
        1.
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
