use image::*;

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

pub fn filter_nones<T>(v: Vec<Option<T>>) -> Vec<T> {
    v.into_iter().filter_map(|i| i).collect::<Vec<T>>()
}
