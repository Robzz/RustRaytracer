use nalgebra::*;
use objects::*;
use std::boxed::Box as StdBox;

#[derive(Clone)]
pub struct Intersection {
    pub position: Point3<f64>,
    pub distance: f64,
    pub object: Object
}

impl Intersection {
    pub fn new(position: Point3<f64>, distance: f64, object: Object) -> Intersection {
        Intersection { position: position, distance: distance, object: object }
    }
}

pub fn closest_intersection(intersections: Vec<Intersection>) -> Option<Intersection> {
    let inter_opt = intersections.iter().min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap());
    inter_opt.cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use objects::Face;
    use num_traits::One;
    use material::Simple;
    use image::Rgb;

    #[test]
    fn test_new_intersection() {
        let pos = Point3::new(0., 0., 0.);
        let d = 5.;
        let f = Face::new(1., 1., Isometry3::one(), StdBox::new(Simple::new(Rgb { data: [0., 0., 0.] })));
        let obj = Object::from_surface(StdBox::new(f));
        let i = Intersection::new(pos, d, obj);
        assert!(i.position == pos);
        assert!(i.distance == d);
    }
}
