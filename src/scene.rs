use image::{Rgb, RgbImage};
use face::Face;
use camera::Camera;
use surface::Surface;

#[derive(Debug)]
pub struct Scene<C: Camera> {
    bg: Rgb<u8>,
    faces: Vec<Face>,
    camera: C
}

impl<C: Camera> Scene<C> {
    pub fn new(background: Rgb<u8>, faces: Vec<Face>, camera: C) -> Scene<C> {
        Scene { bg: background, faces: faces, camera: camera }
    }

    pub fn background(&self) -> Rgb<u8> {
        self.bg
    }

    pub fn set_background(&mut self, background: Rgb<u8>) {
        self.bg = background;
    }

    pub fn render(&self) -> RgbImage {
        let (width, height) = self.camera.viewport();
        let mut img = RgbImage::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = self.bg;
            let ray = self.camera.pixel_ray((x, y)).unwrap();
            for face in &self.faces {
                if let Some(inter) = face.intersects(&ray) {
                    println!("Intersection at pixel ({}, {}): {:?}", x, y, inter);
                    *pixel = Rgb { data: [0, 0, 0] };
                    break;
                }
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
        let c = Rgb { data: [90, 90, 90] };
        let transform = Isometry3::new(Vector3::zero(), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        let scene = Scene::new(c, vec!(), cam);
        assert!(scene.background() == c);
    }

    #[test]
    fn test_scene_render() {
        let c = Rgb { data: [90, 90, 90] };
        let transform = Isometry3::new(Vector3::zero(), Vector3::zero());
        let cam = Orthographic::new((800, 600), (100., 100.), transform);
        let scene = Scene::new(c, vec!(), cam);
        let rendered_img = scene.render();
        for pixel in rendered_img.pixels() {
            assert!(*pixel == c);
        }
    }
}
