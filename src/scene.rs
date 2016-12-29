use image::Rgb;

pub struct Scene {
    pub background: Rgb<u8>
}

impl Scene {
    pub fn new(background: Rgb<u8>) -> Scene {
        Scene { background: background }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scene() {
        let c = Rgb { data: [90, 90, 90] };
        let scene = Scene::new(c);
        assert!(scene.background == c);
    }
}
