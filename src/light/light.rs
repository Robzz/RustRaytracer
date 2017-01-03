use material::{Material, LightMaterial};
use objects::*;
use nalgebra::*;
use std::boxed::Box as StdBox;
use ray::Ray;
use intersection::Intersection;
use image::Rgb;
use util::*;
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub struct Light {
    face: Face,
    material: LightMaterial
}

impl Light {
    pub fn new(face: Face, material: LightMaterial) -> Light {
        Light { face: face, material: material }
    }

    pub fn transform(&self) -> Isometry3<f64> { self.face.transform }

    pub fn as_drawable(&self) -> &Drawable {
        self
    }

    pub fn random_on_face(&self) -> Point3<f64> {
        self.face.random_on_face()
    }

    pub fn light_material(&self) -> &LightMaterial {
        &self.material
    }

    pub fn shade_diffuse(&self, n: Vector3<f64>, obj: &Object, ray: &Ray, inter: &Intersection) -> Rgb<f64> {
        let l = (inter.position - ray.origin).normalize();
        let d = l.dot(&n);
        let norm_factor = 1. / PI;
        let mut c = rgb_mul(&self.material.diffuse_intensity, d * norm_factor);
        c = rgb_mul2(&c, &obj.material().diffuse_color());

        c
    }

    pub fn shade_specular(&self, eye: Point3<f64>, n: Vector3<f64>, obj: &Object,
                          ray: &Ray, inter: &Intersection) -> Rgb<f64> {
        let l = (inter.position - ray.origin).normalize();
        let dln = l.dot(&n);
        let r = 2. * dln * n - l;
        let v = (eye - ray.origin).normalize();
        let d = r.dot(&v).powf(obj.material().shininess());
        let norm_factor = (obj.material().shininess() + 2.) / (2. * PI);
        let mut c = rgb_mul(&self.material.specular_intensity, d * norm_factor);
        c = rgb_mul2(&c, &obj.material().specular_color());

        c
    }
}

impl Clone for Light {
    fn clone(&self) -> Light {
        Light { face: self.face.clone(),
                material: self.material.clone() }
    }
}

impl Intersectable for Light {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let mut inter_opt = self.face.intersects(ray);
        if let Some(ref mut inter) = inter_opt {
            inter.object = Object::from_light(self.clone())
        }
        inter_opt
    }
}

impl Drawable for Light {
    fn material(&self) -> StdBox<Material> {
        self.face.material()
    }

    fn box_clone(&self) -> StdBox<Drawable> {
        StdBox::new(self.clone())
    }
}
