use image::Rgb;
use material::Material;

#[derive(Debug, Clone)]
pub struct Phong {
    ambient: Rgb<f64>,
    diffuse: Rgb<f64>,
    specular: Rgb<f64>,
    shininess: f64
}

impl Phong {
    pub fn new(ambient: Rgb<f64>, diffuse: Rgb<f64>, specular: Rgb<f64>, shininess: f64) -> Phong {
        Phong { ambient: ambient, diffuse: diffuse, specular: specular, shininess: shininess }
    }
}

impl Material for Phong {
    fn ambient_color(&self) -> Rgb<f64> {
        self.ambient
    }

    fn diffuse_color(&self) -> Rgb<f64> {
        self.diffuse
    }

    fn specular_color(&self) -> Rgb<f64> {
        self.specular
    }

    fn shininess(&self) -> f64 {
        self.shininess
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
