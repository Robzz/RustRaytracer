use material::{LightMaterial};
use objects::*;
use nalgebra::*;
use ray::Ray;
use intersection::Intersection;
use image::Rgb;
use util::*;
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub struct Light {
    pub face: Face,
    pub material: LightMaterial
}

impl Light {
    pub fn new(face: Face, material: LightMaterial) -> Light {
        Light { face: face, material: material }
    }

    pub fn transform(&self) -> Isometry3<f64> { self.face.transform }

    pub fn random_on_face(&self) -> Point3<f64> {
        self.face.random_on_face()
    }

    pub fn light_material(&self) -> &LightMaterial {
        &self.material
    }

    pub fn shade_diffuse(&self, obj_inter: &Intersection, shadow_ray_inter: &Intersection) -> Rgb<f64> {
        let l = shadow_ray_inter.ray.direction.normalize();
        let d = l.dot(&obj_inter.normal);
        let norm_factor = 1. / PI;
        let mut c = rgb_mul(&self.material.diffuse_intensity, d * norm_factor);
        c = rgb_mul2(&c, &obj_inter.object.material().diffuse_color());

        c
    }

    pub fn shade_specular(&self, eye: Point3<f64>, obj_inter: &Intersection,
                          shadow_ray_inter: &Intersection) -> Rgb<f64> {
        let l = shadow_ray_inter.ray.direction.normalize();
        let n = obj_inter.normal;
        let dln = l.dot(&n);
        let r = 2. * dln * n - l;
        let v = (eye - obj_inter.ray.origin).normalize();
        let d = r.dot(&v).powf(obj_inter.object.material().shininess());
        let norm_factor = (obj_inter.object.material().shininess() + 2.) / (2. * PI);
        let mut c = rgb_mul(&self.material.specular_intensity, d * norm_factor);
        c = rgb_clamp_0_1(&rgb_mul2(&c, &obj_inter.object.material().specular_color()));

        c
    }
}

impl Clone for Light {
    fn clone(&self) -> Light {
        Light { face: self.face.clone(),
                material: self.material.clone() }
    }
}
