use image::Rgb;
use std::fmt::Debug;

pub trait Material: Debug {
    fn diffuse_color(&self) -> Rgb<f64>;
    fn ambient_color(&self) -> Rgb<f64>;

    fn box_clone(&self) -> Box<Material>;
}
