use image::*;

use objects::*;
use ray::Ray;
use scene::Scene;
use util::*;
use raytracer::sampler::*;

pub struct SimpleSettings {
    pub correct_gamma: bool,
    pub n_samples: u32
}

/// Simple ray tracer. Only does simple illumination and no reflections.
pub struct Simple<S: PixelSampler> {
    scene: Scene,
    settings: SimpleSettings,
    sampler: S
}

impl<S: PixelSampler> Simple<S> {
    pub fn new(scene: Scene, settings: SimpleSettings, sampler: S) -> Simple<S> {
        Simple { scene: scene, settings: settings, sampler: sampler }
    }

    fn ray_energy(&self, ray: &Ray) -> Rgb<f64> {
        // Find closest intersection
        let intersect_opt = self.scene.intersects(&ray);
        let mut pixel = Rgb { data: [0., 0., 0.] };

        if let Some(intersect) = intersect_opt {
            match intersect.object {
                &Object::Light(ref l) => {
                    // Paint the light with its diffuse color
                    pixel = l.light_material().diffuse_intensity;
                },
                &Object::Surface(ref s) => {
                    // Cast light ray and compute Phong shading
                    let surface_normal = intersect.normal;
                    for light in self.scene.lights() {
                        let p = light.random_on_face();
                        let light_ray = Ray::between(intersect.position, p);
                        match self.scene.intersects(&light_ray) {
                            None => (),
                            Some(light_inter) => {
                                if light_inter.object == &Object::from_light(light.clone()) {
                                    let ray_diffuse_color = light.shade_diffuse(surface_normal, &intersect.object, &light_ray, &light_inter);
                                    let ray_specular_color = light.shade_specular(self.scene.camera().eye_position(), surface_normal, &intersect.object, &light_ray, &light_inter);
                                    let mut color = rgb_add(&ray_diffuse_color, &intersect.object.material().ambient_color());
                                    color = rgb_add(&color, &ray_specular_color);

                                    pixel = color;
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

        for (x, y_inverted, pixel) in img.enumerate_pixels_mut() {
            let y = height - 1 - y_inverted;
            let mut energy = Rgb { data: [0., 0., 0.] };
            for sample in self.sampler.samples((x, y), self.settings.n_samples) {
                let (xf, yf) = sample;
                let ray = self.scene.camera().pixel_ray((xf, yf)).unwrap();
                energy = rgb_add(&energy, &self.ray_energy(&ray));
            }
            energy = rgb_clamp_0_1(&rgb_div(&energy, self.settings.n_samples as f64));
            *pixel = rgb_to_u8(&rgb_01_to_255(&energy));
        }
        img
    }
}
