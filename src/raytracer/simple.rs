use image::*;
use nalgebra::*;
use rayon::prelude::*;

use std::boxed::Box as StdBox;
use std::sync::{Arc, Mutex};

use intersection::Intersection;
use light::Light;
use objects::*;
use ray::Ray;
use raytracer::sampler::*;
use scene::Scene;
use util::*;

pub struct SimpleSettings<F: Fn(f64)> {
    pub n_samples: u32,
    pub progress_callback: Option<StdBox<F>>
}

/// Simple ray tracer. Only does simple illumination and no reflections.
pub struct Simple<S: PixelSampler, F: Fn(f64)> {
    scene: Scene,
    settings: SimpleSettings<F>,
    sampler: S
}

pub trait Renderer {
    fn render(&self) -> RgbImage;
}

pub trait ParallelRenderer {
    fn render_parallel(&self) -> RgbImage;
}

impl<S: PixelSampler, F: Fn(f64)> Simple<S, F> {
    pub fn new(scene: Scene, settings: SimpleSettings<F>, sampler: S) -> Simple<S, F> {
        Simple { scene: scene, settings: settings, sampler: sampler }
    }

    fn cast_shadow_ray(&self, i: &Intersection, light: &Light) -> Option<Intersection> {
        let surface_normal = i.normal;
        let p = light.random_on_face();
        let ray_direction = p - i.position;
        let shadow_ray = Ray::new(i.position, ray_direction);
        match self.scene.intersects(shadow_ray) {
            None => None,
            Some(inter) => { if surface_normal.dot(&ray_direction) > 0. &&
                                inter.object == &Object::from_light(light.clone())
                                { Some(inter) } else { None }}
        }
    }

    fn ray_energy(&self, ray: Ray) -> Rgb<f64> {
        // Find closest intersection
        let intersect_opt = self.scene.intersects(ray);
        let mut pixel;

        if let Some(intersect) = intersect_opt {
            match intersect.object {
                &Object::Light(ref l) => {
                    // Paint the light with its diffuse color
                    pixel = l.light_material().diffuse_intensity;
                },
                &Object::Surface(_) => {
                    // Cast light ray and compute Phong shading
                    let surface_normal = intersect.normal;
                    pixel = intersect.object.material().ambient_color();
                    for light in self.scene.lights() {
                        match self.cast_shadow_ray(&intersect, light) {
                            None => (),
                            Some(inter) => {
                                let ray_diffuse_color = light.shade_diffuse(&intersect, &inter);
                                let ray_specular_color = light.shade_specular(self.scene.camera().eye_position(), &intersect, &inter);
                                pixel = rgb_add(&rgb_add(&ray_diffuse_color, &ray_specular_color), &pixel);
                            }
                        }
                    }
                }
            }
        }
        else {
            pixel = self.scene.background();
        }
        pixel
    }
}

impl<S, P> Renderer for Simple<S, P>
    where S: PixelSampler, P: Fn(f64)
{
    fn render(&self) -> RgbImage {
        let (width, height) = self.scene.camera().viewport();
        let mut img = RgbImage::new(width, height);
        let n2 = (self.settings.n_samples * self.settings.n_samples) as f64;
        let n_samples = (width * height) as f64;
        let mut i = 0.;

        for (x, y_inverted, pixel) in img.enumerate_pixels_mut() {
            let y = height - 1 - y_inverted;
            let mut energy = Rgb { data: [0., 0., 0.] };
            for sample in self.sampler.samples((x, y), self.settings.n_samples) {
                let (xf, yf) = sample;
                let ray = self.scene.camera().pixel_ray((xf, yf)).unwrap();
                energy = rgb_add(&energy, &self.ray_energy(ray));
            }
            energy = rgb_clamp_0_1(&rgb_div(&energy, n2));
            *pixel = rgb_to_u8(&rgb_01_to_255(&energy));

            if let Some(ref cb) = self.settings.progress_callback {
                i += 1.;
                cb(i / n_samples);
            }
        }
        img
    }
}

impl<S, P> ParallelRenderer for Simple<S, P>
    where S: PixelSampler + Sync,
          P: Fn(f64) + Sync
{
    fn render_parallel(&self) -> RgbImage {
        let (width, height) = self.scene.camera().viewport();
        let n2 = (self.settings.n_samples * self.settings.n_samples) as f64;
        let rows = (0..height).into_iter().map(|y| { (0..width).into_iter().map(|x| (x, y)).collect::<Vec<(u32, u32)>>() })
                                          .collect::<Vec<Vec<(u32, u32)>>>();
        let i = Arc::new(Mutex::new(0.));

        let pixels : Vec<Vec<((u32, u32), Rgb<u8>)>> = rows.par_iter().map(|row| -> Vec<((u32, u32), Rgb<u8>)> {
            let row_pixels = row.into_iter().map(|&(x, y)| {
                let mut energy = Rgb { data: [0., 0., 0.] };
                for sample in self.sampler.samples((x, y), self.settings.n_samples) {
                    let (xf, yf) = sample;
                    let ray = self.scene.camera().pixel_ray((xf, yf)).unwrap();
                    energy = rgb_add(&energy, &self.ray_energy(ray));
                }
                energy = rgb_clamp_0_1(&rgb_div(&energy, n2));
                let pixel = rgb_to_u8(&rgb_01_to_255(&energy));
                ((x, y), pixel)

            }).collect();
            if let Some(ref cb) = self.settings.progress_callback {
                let mut i_mut = i.lock().unwrap();
                *i_mut += 1.;
                cb(*i_mut / height as f64);
            }
            row_pixels
        }).collect();

        let mut img = RgbImage::new(width, height);
        for row in pixels {
            for ((x, y), pixel) in row {
                img.put_pixel(x, height - 1 - y, pixel);
            }
        }
        img
    }
}
