use objects::Face;
use image::Rgb;
use surface::Surface;
use material::{Material, LightMaterial};

#[derive(Debug)]
pub struct Light {
    face: Face,
    material: Box<Material>
}

impl Light {
    pub fn new(face: Face, material: LightMaterial) -> Light {
        Light { face: face, material: material.to_material() }
    }
}

impl Surface for Light {
    fn faces<'a>(&'a self) -> Vec<&'a Face> { vec!(&self.face) }

    fn material<'a>(&'a self) -> &'a Box<Material> { &self.material }
}
