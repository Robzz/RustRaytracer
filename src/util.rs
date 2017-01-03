use image::*;
use nalgebra::*;
use algebra::OrthoNormalBase;

pub fn rgb_01_to_255(pixel: &Rgb<f64>) -> Rgb<f64> {
    Rgb { data: [pixel[0] * 255., pixel[1] * 255., pixel[2] * 255.] }
}

pub fn rgb_to_u8(pixel: &Rgb<f64>) -> Rgb<u8> {
    Rgb { data: [pixel[0] as u8, pixel[1] as u8, pixel[2] as u8] }
}

pub fn rgb_to_f64<T>(pixel: &Rgb<T>) -> Rgb<f64>
    where T: Primitive + Into<f64> {
    Rgb { data: [pixel[0].into() , pixel[1].into(), pixel[2].into()] }
}

pub fn rgb_add(r1: &Rgb<f64>, r2: &Rgb<f64>) -> Rgb<f64> {
    r1.map2(r2, |c1, c2| c1 + c2)
}

pub fn rgb_div(r: &Rgb<f64>, f: f64) -> Rgb<f64> {
    r.map(|c| c / f)
}

pub fn rgb_mul(r: &Rgb<f64>, f: f64) -> Rgb<f64> {
    r.map(|c| c * f)
}

pub fn rgb_mul2(r1: &Rgb<f64>, r2: &Rgb<f64>) -> Rgb<f64> {
    r1.map2(r2, |c1, c2| c1 * c2)
}

pub fn rgb_clamp_0_1(r: &Rgb<f64>) -> Rgb<f64> {
    r.map(|c| clamp(c, 0., 1.))
}

pub fn random_in_cone(direction: Vector3<f64>, angle: f64) -> Vector3<f64> {
    use rand::distributions::*;
    use rand::*;
    let range = Range::new(-angle, angle);
    let mut rng = thread_rng();
    let theta = range.ind_sample(&mut rng);
    let phi = range.ind_sample(&mut rng);
    let base = OrthoNormalBase::from_w(direction);
    let v = Rotation3::new(base.u() * theta).transform(&Rotation3::new(base.v() * phi).transform(&direction));
    v.normalize()
}

pub fn filter_nones<T>(v: Vec<Option<T>>) -> Vec<T> {
    v.into_iter().filter_map(|i| i).collect::<Vec<T>>()
}
