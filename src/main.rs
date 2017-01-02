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
use material::{Phong, LightMaterial};
use light::Light;

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

    let ambient = Rgb { data: [0.1, 0.1, 0.1] };
    let specular = Rgb { data: [0., 0., 0.] };
    let material_grey   = Phong::new(ambient, Rgb { data: [0.6, 0.6, 0.6] }, specular, 0.);
    let material_light  = LightMaterial::new(Rgb { data: [1.0, 1.0, 1.0] });
    let wall_left = Face::new(5., 5.,
                              Isometry3::new(Vector3::new(-2., 0., -2.), Vector3::y() * (PI / 2.)),
                              Box::new(material_grey.clone()));
    let wall_right = Face::new(5., 5.,
                               Isometry3::new(Vector3::new(2., 0., -2.), Vector3::y() * (PI / 2.)),
                               Box::new(material_grey.clone()));
    let light = Light::new(Face::new(0.5, 0.5,
                                     Isometry3::new(Vector3::new(0., 1., -2.), Vector3::x() * (PI / 2.)),
                                     material_light.to_material()),
                           material_light.clone());

    let cam_transform = Isometry3::one();
    let cam = Perspective::new((width, height),
                               ((110.).to_radians(), (70.).to_radians()),
                               cam_transform);
    let scene = Scene::new(Rgb { data: [0.3, 0.3, 0.3] },
                           vec!(&wall_left, &wall_right),
                           vec!(&light),
                           Box::new(cam));

    let render = scene.render();
    render.save(output_path).expect("Cannot save output image");
}
