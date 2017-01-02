#![feature(plugin, relaxed_adts, conservative_impl_trait)]
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
mod util;

use image::{Rgb, RgbImage, Primitive};
use std::path::Path;
use scene::Scene;
use nalgebra::*;
use std::f64::consts::PI;
use camera::Perspective;
use num_traits::{Zero, One, Float};
use material::{Phong, LightMaterial};
use light::Light;
use ray::Ray;
use objects::*;
use std::boxed::Box as StdBox;

docopt!(Args, "
Usage: raytrace <output> <width> <height>

Options:
    -a, --archive  Copy everything.
",
arg_width: u32, arg_height: u32);

fn rgb_01_to_255(pixel: &Rgb<f64>) -> Rgb<f64> {
    Rgb { data: [pixel[0] * 255., pixel[1] * 255., pixel[2] * 255.] }
}

fn rgb_to_u8(pixel: &Rgb<f64>) -> Rgb<u8> {
    Rgb { data: [pixel[0] as u8, pixel[1] as u8, pixel[2] as u8] }
}

fn rgb_to_f64<T>(pixel: &Rgb<T>) -> Rgb<f64>
    where T: Primitive + Into<f64> {
    Rgb { data: [pixel[0].into() , pixel[1].into(), pixel[2].into()] }
}

pub fn render(scene: &Scene) -> RgbImage {
    let (width, height) = scene.camera().viewport();
    let mut img = RgbImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = rgb_to_u8(&rgb_01_to_255(&scene.background()));

        let ray = scene.camera().pixel_ray((x, (height - 1 - y))).unwrap();
        use std::f64::MAX;
        let mut min_distance = MAX;

        // Find closest intersection
        let intersect_opt = scene.intersects(&ray);

        if let Some(intersect) = intersect_opt {
            match intersect.object {
                Object::Light(ref l) => {
                    *pixel = rgb_to_u8(&rgb_01_to_255(&l.material().shade(&intersect, scene)));
                },
                Object::Surface(ref s) => {
                    // Cast shadow rays and compute diffuse component
                    *pixel = rgb_to_u8(&rgb_01_to_255(&s.material().shade(&intersect, scene)));
                    let diffuse = Rgb { data: [0., 0., 0.] };
                    for light in scene.lights() {
                        let shadow_ray = Ray::new(intersect.position, *light.transform().translation.as_point() - intersect.position);
                    }
                }
            }
        }
        else {
            *pixel = rgb_to_u8(&rgb_01_to_255(&scene.background()));
        }
    }
        img
    }

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
                              StdBox::new(material_grey.clone()));
    let wall_right = Face::new(5., 5.,
                               Isometry3::new(Vector3::new(2., 0., -2.), Vector3::y() * (PI / 2.)),
                               StdBox::new(material_grey.clone()));
    let light = Light::new(Face::new(0.5, 0.5,
                                     Isometry3::new(Vector3::new(0., 1., -2.), Vector3::x() * (PI / 2.)),
                                     material_light.to_material()),
                           material_light.clone());

    let cam_transform = Isometry3::one();
    let cam = Perspective::new((width, height),
                               ((110.).to_radians(), (70.).to_radians()),
                               cam_transform);
    let scene = Scene::new(Rgb { data: [0.3, 0.3, 0.3] },
                           vec!(Object::from_surface(StdBox::new(wall_left)),
                                Object::from_surface(StdBox::new(wall_right)),
                                Object::from_light(light)),
                           StdBox::new(cam));

    let render = render(&scene);
    render.save(output_path).expect("Cannot save output image");
}
