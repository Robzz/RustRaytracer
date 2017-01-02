use image::Rgb;
use intersection::Intersection;
use material::Material;
use light::Light;
use scene::Scene;

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
    fn shade<'a>(&self, intersect: &'a Intersection<'a>, scene: &Scene) -> Rgb<f64> {
        let lights = &scene.lights();
        for light in lights {
            // Can the intersection see the light?
        }
        self.diffuse
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
