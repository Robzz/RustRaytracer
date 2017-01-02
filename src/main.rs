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
use camera::Orthographic;
use num_traits::Zero;
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
    let material = Simple::new(Rgb { data: [1.0, 0.0, 0.0] });
    let f = Face::new(50., 20., Isometry3::new(Vector3::z() * 50.,
                                               Vector3::z() * (PI / 4.)),
                      material);
    let cam_transform = Isometry3::new(Vector3::zero(), Vector3::z() * PI);
    let cam = Orthographic::new((width, height), (100., 100.), cam_transform);
    let scene = Scene::new(Rgb { data: [0.3, 0.3, 0.3] }, vec!(f), cam);
    let render = scene.render();
    render.save(output_path).expect("Cannot save output image");
}
