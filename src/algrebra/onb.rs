use nalgebra::*;

pub struct OrthoNormalBase {
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>
}

impl OrthoNormalBase {
    fn u(&self) -> Vector3<f64> { self.u }
    fn v(&self) -> Vector3<f64> { self.v }
    fn w(&self) -> Vector3<f64> { self.w }

    fn from_u(u: Vector3<f64>) -> OrthoNormalBase {
        let un = u.normalize();
        let mut v = un.cross(&Vector3::x());
        if v.norm().approx_eq(&0.) {
            v = un.cross(&Vector3::y());
        }
        let w = un.cross(&v);
        OrthoNormalBase { u:un, v:v, w:w }
    }

    fn from_v(v: Vector3<f64>) -> OrthoNormalBase {
        let vn = v.normalize();
        let mut u = vn.cross(&Vector3::x());
        if u.norm().approx_eq(&0.) {
            u = vn.cross(&Vector3::y());
        }
        let w = u.cross(&vn);
        OrthoNormalBase { u:u, v:vn, w:w }
    }

    fn from_w(w: Vector3<f64>) -> OrthoNormalBase {
        let wn = w.normalize();
        let mut u = wn.cross(&Vector3::x());
        if u.norm().approx_eq(&0.) {
            u = wn.cross(&Vector3::y());
        }
        let v = wn.cross(&u);
        OrthoNormalBase { u:u, v:v, w:wn }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_normalized(onb: &OrthoNormalBase) -> bool {
        onb.u().norm().approx_eq(&1.) &&
        onb.v().norm().approx_eq(&1.) &&
        onb.w().norm().approx_eq(&1.)
    }

    fn is_orthogonal(onb: &OrthoNormalBase) -> bool {
        onb.u().cross(&onb.v()).approx_eq(&onb.w()) &&
        onb.v().cross(&onb.w()).approx_eq(&onb.u()) &&
        onb.w().cross(&onb.u()).approx_eq(&onb.v())
    }

    #[test]
    fn test_onb_from_u() {
        let onb = OrthoNormalBase::from_u(Vector3::x());
        assert!(is_normalized(&onb));
        assert!(is_orthogonal(&onb));
    }

    #[test]
    fn test_onb_from_v() {
        let onb = OrthoNormalBase::from_v(Vector3::x());
        assert!(is_normalized(&onb));
        assert!(is_orthogonal(&onb));
    }

    #[test]
    fn test_onb_from_w() {
        let onb = OrthoNormalBase::from_w(Vector3::x());
        assert!(is_normalized(&onb));
        assert!(is_orthogonal(&onb));
    }
}
