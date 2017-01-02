use nalgebra::*;
use surface::Surface;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub position: Point3<f64>,
    pub distance: f64,
    pub surface: &'a (Surface + 'a)
}

impl<'a> Intersection<'a> {
    pub fn new(position: Point3<f64>, distance: f64, surface: &'a Surface) -> Intersection<'a> {
        Intersection { position: position, distance: distance, surface: surface }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use face::Face;
    use num_traits::One;
    use material::Simple;
    use image::Rgb;

    #[test]
    fn test_new_intersection() {
        let pos = Point3::new(0., 0., 0.);
        let d = 5.;
        let f = Face::new(1., 1., Isometry3::one(), Simple::new(Rgb { data: [0., 0., 0.] }));
        let i = Intersection::new(pos, d, &f);
        assert!(i.position == pos);
        assert!(i.distance == d);
    }
}

