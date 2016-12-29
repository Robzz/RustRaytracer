use image::Rgb;

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
}
