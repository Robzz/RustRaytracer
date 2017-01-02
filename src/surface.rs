use objects::Face;
use ray::Ray;
use intersection::Intersection;
use std::fmt::Debug;
use std::cmp::PartialOrd;
use material::Material;

pub trait Surface: Debug {
    fn faces<'a>(&'a self) -> Vec<&'a Face>;

    fn intersects<'a>(&'a self, ray: &Ray) -> Option<Intersection<'a>> {
        let faces = self.faces();
        let intersections = faces.iter().map(|f| f.intersects(ray))
                                        .filter_map(|i| i);
        intersections.min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }

    fn material<'a>(&'a self) -> &'a Box<Material>;
}
