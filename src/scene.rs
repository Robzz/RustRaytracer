use image::{Rgb, RgbImage, Primitive};
use face::Face;
use camera::Camera;
use surface::Surface;
use material::Material;

#[derive(Debug)]
pub struct Scene<C: Camera> {
    bg: Rgb<f64>,
    faces: Vec<Face>,
    camera: C
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

impl<C: Camera> Scene<C> {
    pub fn new(background: Rgb<f64>, faces: Vec<Face>, camera: C) -> Scene<C> {
        Scene { bg: background, faces: faces, camera: camera }
    }

    pub fn background(&self) -> Rgb<f64> {
        self.bg
    }

    pub fn set_background(&mut self, background: Rgb<f64>) {
        self.bg = background;
    }

    pub fn render(&self) -> RgbImage {
        let (width, height) = self.camera.viewport();
        let mut img = RgbImage::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = rgb_to_u8(&rgb_01_to_255(&self.bg));

            let ray = self.camera.pixel_ray((x, y)).unwrap();
            println!("{:?}", ray);
            use std::f64::MAX;
            let mut min_distance = MAX;
            let mut color = self.bg;
            for face in &self.faces {
                if let Some(inter) = face.intersects(&ray) {
                    if inter.distance < min_distance {
                        min_distance = inter.distance;
                        color = face.material.shade(&inter, self);
                    }
                }
            }
            *pixel = rgb_to_u8(&rgb_01_to_255(&color));
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
        let scene = Scene::new(c, vec!(), cam);
        assert!(scene.background() == c);
    }

    #[test]
    fn test_scene_render() {
        let c = Rgb { data: [0.3, 0.3, 0.3] };
        let transform = Isometry3::new(Vector3::zero(), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        let scene = Scene::new(c, vec!(), cam);
        let rendered_img = scene.render();
        for pixel in rendered_img.pixels() {
            assert!(*pixel == rgb_to_u8(&c));
        }
    }
}
