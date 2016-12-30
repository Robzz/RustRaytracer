use nalgebra::{Vector3, Point3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin: origin, direction: direction}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ray() {
        let origin = Point3::new(0., 0., 0.);
        let direction = Vector3::new(0., 0., 1.);
        let ray = Ray::new(origin, direction);
        assert!(ray.direction == direction);
        assert!(ray.origin == origin);
    }
}
