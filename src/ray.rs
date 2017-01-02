use nalgebra::*;
use objects::Face;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin: origin, direction: direction }
    }

    /// Returns the point of intersection and distance between the ray and the
    /// given face, if any
    pub fn intersects_face(&self, f: &Face) -> Option<(Point3<f64>, f64)> {
        let p = f.transform.transform(&Point3::new(0., 0., 0.));
        let n = f.normal();
        let d = dot(&self.direction, &n);
        match d.approx_eq(&0.) {
            true => None,
            false => {
                // Ray is not parallel to plane
                // Find if the intersection is in the positive direction
                let t = dot(&(p - self.origin), &n) / d;
                match t < 0. {
                    true => None,
                    false => {
                        // Find the intersection point on the face's plane and
                        // make sure it's within the face
                        let i_world = self.origin + t * self.direction;
                        let i_local = f.transform.inverse().unwrap().transform(&i_world);
                        match (abs(&i_local.x) <= (f.width / 2.)) && (abs(&i_local.y) <= (f.height / 2.)) {
                            true => Some((i_world, norm(&(i_world - self.origin)))),
                            false => None
                        }
                    }
                }
            }
        }
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
