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

use image::Rgb;
use std::path::Path;
use scene::Scene;

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
    let scene = Scene::new(Rgb { data: [90, 90, 90] });
    let render = scene.render(width, height);
    render.save(output_path).expect("Cannot save output image");
}
