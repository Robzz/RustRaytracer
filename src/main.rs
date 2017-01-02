#![feature(plugin, relaxed_adts)]
#![plugin(docopt_macros)]
#![allow(dead_code)]

extern crate docopt;
extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate rustc_serialize;

mod ray;
mod surface;
mod scene;
mod camera;
mod intersection;
mod material;
mod objects;
mod conversions;
mod light;
mod algrebra;

use image::Rgb;
use std::path::Path;
use scene::Scene;
use objects::Face;
use objects::Box as Box3D;
use nalgebra::*;
use std::f64::consts::PI;
use camera::Perspective;
use num_traits::{Zero, One, Float};
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
    let material_green = Simple::new(Rgb { data: [0.0, 1.0, 0.0] });
    let f1 = Face::new(50., 20.,
                       Isometry3::new(Vector3::z() * -50., Vector3::z() * (PI / 4.)),
                       Box::new(material_red));
    let f2 = Face::new(50., 20.,
                       Isometry3::new(Vector3::z() * -45., Vector3::z() * -(PI / 4.)),
                       Box::new(material_blue));
    let b = Box3D::new(Vector3::one() * 5.,
                       Isometry3::new(Vector3::new(5., 5., -15.), Vector3::zero()),
                       Box::new(material_green));
    let cam_transform = Isometry3::one();
    let cam = Perspective::new((width, height),
                               ((110.).to_radians(), (70.).to_radians()),
                               cam_transform);
    let scene = Scene::new(Rgb { data: [0.3, 0.3, 0.3] },
                           vec!(&f1, &f2, &b),
                           Box::new(cam));

    let render = scene.render();
    render.save(output_path).expect("Cannot save output image");
}
