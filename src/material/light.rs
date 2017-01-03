use image::Rgb;

#[derive(Debug, Clone, PartialEq)]
pub struct LightMaterial {
    pub diffuse_intensity: Rgb<f64>,
    pub specular_intensity: Rgb<f64>
}

impl LightMaterial {
    pub fn new(diffuse: Rgb<f64>, specular: Rgb<f64>) -> LightMaterial {
        LightMaterial { diffuse_intensity: diffuse, specular_intensity: specular }
    }
}
