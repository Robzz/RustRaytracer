#![feature(plugin, relaxed_adts, conservative_impl_trait)]
#![allow(dead_code)]

extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate rand;

pub mod ray;
pub mod scene;
pub mod camera;
pub mod intersection;
pub mod material;
pub mod objects;
pub mod light;
pub mod algebra;
pub mod util;
