use image::Rgb;
use camera::Camera;
use light::Light;
use std::boxed::Box as StdBox;
use util::filter_nones;
use ray::Ray;
use intersection::*;
use objects::*;

pub struct Scene {
    bg: Rgb<f64>,
    objects: Vec<Object>,
    camera: StdBox<Camera>
}

impl Scene {
    pub fn new(background: Rgb<f64>, objects: Vec<Object>,
               camera: StdBox<Camera>) -> Scene {
        Scene { bg: background, objects: objects, camera: camera }
    }

    pub fn background(&self) -> Rgb<f64> {
        self.bg
    }

    pub fn set_background(&mut self, background: Rgb<f64>) {
        self.bg = background;
    }

    pub fn camera(&self) -> &StdBox<Camera> {
        &self.camera
    }

    pub fn objects(&self) -> Vec<&Object> {
        self.objects.iter().collect()
    }

    pub fn surfaces(&self) -> Vec<&Surface> {
        self.objects.iter().filter_map(|o| o.as_surface()).collect()
    }

    pub fn lights(&self) -> Vec<&Light> {
        self.objects.iter().filter_map(|o| o.as_light()).collect()
    }
}

impl Intersectable for Scene {
    fn intersects(&self, ray: Ray) -> Option<Intersection> {
        let intersections = filter_nones(self.objects.iter().map(|o| o.intersects(ray.clone())).collect());
        closest_intersection(intersections)
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
        let scene = Scene::new(c, vec!(), StdBox::new(cam));
        assert!(scene.background() == c);
    }
}
