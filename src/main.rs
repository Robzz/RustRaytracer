extern crate image;
extern crate nalgebra;

mod ray;
mod face;
mod surface;
mod scene;
mod camera;

use image::Rgb;
use std::path::Path;
use scene::Scene;

fn main() {
    let output_path = Path::new("render.png");
    let scene = Scene::new(Rgb { data: [90, 90, 90] });
    let render = scene.render(400, 300);
    render.save(output_path).expect("Cannot save output image");
}
