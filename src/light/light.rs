use material::{Material, LightMaterial};
use objects::*;
use nalgebra::*;
use std::boxed::Box as StdBox;
use ray::Ray;
use intersection::Intersection;

#[derive(Debug)]
pub struct Light {
    face: Face,
    material: StdBox<Material>
}

impl Light {
    pub fn new(face: Face, material: LightMaterial) -> Light {
        Light { face: face, material: material.to_material() }
    }

    pub fn transform(&self) -> Isometry3<f64> { self.face.transform }

    pub fn as_surface(&self) -> &Surface {
        self
    }
}

impl Clone for Light {
    fn clone(&self) -> Light {
        Light { face: self.face.clone(),
               material: self.material.box_clone() }
    }
}

impl Intersectable for Light {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.face.intersects(ray)
    }
}

impl Surface for Light {
    fn material<'a>(&'a self) -> &'a StdBox<Material> { &self.material }

    fn box_clone(&self) -> StdBox<Surface> {
        StdBox::new(self.clone())
    }
}
