use face::Face;
use ray::Ray;
use intersection::Intersection;

pub trait Surface {
    fn faces(&self) -> Vec<Face>;

    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}
