#![feature(plugin)]
#![plugin(docopt_macros)]
#![allow(dead_code)]

extern crate docopt;
extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate rustc_serialize;

mod ray;
mod face;
mod surface;
mod scene;
mod camera;
mod intersection;
mod material;
//mod object;

use image::Rgb;
use std::path::Path;
use scene::Scene;
use face::Face;
use nalgebra::*;
use std::f64::consts::PI;
use camera::Perspective;
use num_traits::{Zero, Float};
use material::Simple;

docopt!(Args, "
Usage: raytrace <output> <width> <height>

Options:
    -a, --archive  Copy everything.
",
arg_width: u32, arg_height: u32);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let width = args.arg_width;
    let height = args.arg_height;
    let output = args.arg_output;

    let output_path = Path::new(&output);
    let material_red = Simple::new(Rgb { data: [1.0, 0.0, 0.0] });
    let material_blue = Simple::new(Rgb { data: [0.0, 0.0, 1.0] });
    let f1 = Face::new(50., 20., Isometry3::new(Vector3::z() * 50.,
                                                Vector3::z() * (PI / 4.)),
                      material_red);
    let f2 = Face::new(50., 20., Isometry3::new(Vector3::z() * 30.,
                                                Vector3::z() * -(PI / 4.)),
                      material_blue);
    let cam_transform = Isometry3::new(Vector3::zero(), Vector3::z() * PI);
    let cam = Perspective::new((width, height), ((110.).to_radians(), (70.).to_radians()), cam_transform);
    let scene = Scene::new(Rgb { data: [0.3, 0.3, 0.3] }, vec!(f1, f2), cam);
    let render = scene.render();
    render.save(output_path).expect("Cannot save output image");
}
