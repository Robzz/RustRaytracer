use image::*;
use nalgebra::*;

use std::boxed::Box as StdBox;

use objects::*;
use ray::Ray;
use scene::Scene;
use util::*;
use raytracer::sampler::*;

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

impl<S: PixelSampler, F: Fn(f64)> Simple<S, F> {
    pub fn new(scene: Scene, settings: SimpleSettings<F>, sampler: S) -> Simple<S, F> {
        Simple { scene: scene, settings: settings, sampler: sampler }
    }

    fn ray_energy(&self, ray: &Ray) -> Rgb<f64> {
        // Find closest intersection
        let intersect_opt = self.scene.intersects(&ray);
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
                        let p = light.random_on_face();
                        let ray_direction = p - intersect.position;
                        let light_ray = Ray::new(intersect.position, ray_direction);
                        if surface_normal.dot(&ray_direction) > 0. {
                            match self.scene.intersects(&light_ray) {
                                None => (),
                                Some(light_inter) => {
                                    if light_inter.object == &Object::from_light(light.clone()) {
                                        let ray_diffuse_color = light.shade_diffuse(surface_normal, &intersect.object, &light_ray, &light_inter);
                                        let ray_specular_color = light.shade_specular(self.scene.camera().eye_position(), surface_normal, &intersect.object, &light_ray);
                                        pixel = rgb_add(&rgb_add(&ray_diffuse_color, &ray_specular_color), &pixel);
                                    }
                                }
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

    pub fn render(&mut self) -> RgbImage {
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
                energy = rgb_add(&energy, &self.ray_energy(&ray));
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
