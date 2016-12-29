use image::{Rgb, RgbImage};

pub struct Scene {
    bg: Rgb<u8>,
}

impl Scene {
    pub fn new(background: Rgb<u8>) -> Scene {
        Scene { bg: background }
    }

    pub fn background(&self) -> Rgb<u8> {
        self.bg
    }

    pub fn set_background(&mut self, background: Rgb<u8>) {
        self.bg = background;
    }

    pub fn render(&self, width: u32, height: u32) -> RgbImage {
        let mut img = RgbImage::new(width, height);
        for pix in img.pixels_mut() {
            *pix = self.bg;
        }
        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scene() {
        let c = Rgb { data: [90, 90, 90] };
        let scene = Scene::new(c);
        assert!(scene.background() == c);
    }

    #[test]
    fn test_scene_render() {
        let c = Rgb { data: [90, 90, 90] };
        let scene = Scene::new(c);
        let rendered_img = scene.render(400, 300);
        for pixel in rendered_img.pixels() {
            assert!(*pixel == c);
        }
    }
}
