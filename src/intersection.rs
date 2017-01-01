use nalgebra::Point3;

#[derive(Debug)]
pub struct Intersection {
    pub position: Point3<f64>,
    pub distance: f64
}

impl Intersection {
    pub fn new(position: Point3<f64>, distance: f64) -> Intersection {
        Intersection { position: position, distance: distance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_intersection() {
        let pos = Point3::new(0., 0., 0.);
        let d = 5.;
        let i = Intersection::new(pos, d);
        assert!(i.position == pos);
        assert!(i.distance == d);
    }
}

