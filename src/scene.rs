use image::{Rgb, RgbImage, Primitive};
use camera::Camera;
use surface::Surface;
use light::Light;

#[derive(Debug)]
pub struct Scene<'a> {
    bg: Rgb<f64>,
    faces: Vec<&'a Surface>,
    lights: Vec<&'a Light>,
    camera: Box<Camera>
}

fn rgb_01_to_255(pixel: &Rgb<f64>) -> Rgb<f64> {
    Rgb { data: [pixel[0] * 255., pixel[1] * 255., pixel[2] * 255.] }
}

fn rgb_to_u8(pixel: &Rgb<f64>) -> Rgb<u8> {
    Rgb { data: [pixel[0] as u8, pixel[1] as u8, pixel[2] as u8] }
}

fn rgb_to_f64<T>(pixel: &Rgb<T>) -> Rgb<f64>
    where T: Primitive + Into<f64> {
    Rgb { data: [pixel[0].into() , pixel[1].into(), pixel[2].into()] }
}

impl<'a> Scene<'a> {
    pub fn new(background: Rgb<f64>, faces: Vec<&'a Surface>, lights: Vec<&'a Light>,
               camera: Box<Camera>) -> Scene<'a> {
        Scene { bg: background, faces: faces, lights: lights, camera: camera }
    }

    pub fn background(&self) -> Rgb<f64> {
        self.bg
    }

    pub fn set_background(&mut self, background: Rgb<f64>) {
        self.bg = background;
    }

    pub fn lights(&self) -> Vec<&'a Light> {
        self.lights.clone()
    }

    pub fn render(&self) -> RgbImage {
        let (width, height) = self.camera.viewport();
        let mut img = RgbImage::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = rgb_to_u8(&rgb_01_to_255(&self.bg));

            let ray = self.camera.pixel_ray((x, (height - 1 - y))).unwrap();
            use std::f64::MAX;
            let mut min_distance = MAX;
            let mut closest_intersection = None;
            let mut closest_is_light = false;

            // Find closest intersection
            for face in &self.faces {
                let intersect_opt = face.intersects(&ray);
                if let Some(inter) = intersect_opt.clone() {
                    if inter.distance < min_distance {
                        min_distance = inter.distance;
                        closest_intersection = intersect_opt.clone();
                    }
                }
            }
            for light in &self.lights {
                let intersect_opt = light.intersects(&ray);
                if let Some(inter) = intersect_opt.clone() {
                    if inter.distance < min_distance {
                        min_distance = inter.distance;
                        closest_intersection = intersect_opt.clone();
                        closest_is_light = true;
                    }
                }
            }

            if let Some(intersect) = closest_intersection {
                *pixel = rgb_to_u8(&rgb_01_to_255(&intersect.surface.material().shade(&intersect, self)));
            }
            else {
                *pixel = rgb_to_u8(&rgb_01_to_255(&self.bg));
            }
        }
        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camera::Orthographic;
    use nalgebra::*;
    use num_traits::Zero;

    #[test]
    fn test_new_scene() {
        let c = Rgb { data: [0.3, 0.3, 0.3] };
        let transform = Isometry3::new(Vector3::zero(), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        let scene = Scene::new(c, vec!(), vec!(), Box::new(cam));
        assert!(scene.background() == c);
    }

    #[test]
    fn test_scene_render() {
        let c = Rgb { data: [0.3, 0.3, 0.3] };
        let transform = Isometry3::new(Vector3::zero(), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        let scene = Scene::new(c, vec!(), vec!(), Box::new(cam));
        let rendered_img = scene.render();
        for pixel in rendered_img.pixels() {
            assert!(*pixel == rgb_to_u8(&rgb_01_to_255(&c)));
        }
    }
}
