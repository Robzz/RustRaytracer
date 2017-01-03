#![feature(plugin, relaxed_adts, conservative_impl_trait)]
#![allow(dead_code)]

extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate rand;

pub mod algebra;
pub mod camera;
pub mod intersection;
pub mod light;
pub mod material;
pub mod objects;
pub mod ray;
pub mod raytracer;
pub mod scene;
pub mod util;
