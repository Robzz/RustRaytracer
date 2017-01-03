#![feature(plugin, relaxed_adts, conservative_impl_trait)]
#![plugin(docopt_macros)]
#![allow(dead_code)]

extern crate docopt;
extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate rand;
extern crate rustc_serialize;

mod ray;
mod scene;
mod camera;
mod intersection;
mod material;
mod objects;
mod conversions;
mod light;
mod algrebra;
mod util;

use image::*;
use std::path::Path;
use scene::Scene;
use nalgebra::*;
use std::f64::consts::PI;
use camera::Perspective;
use num_traits::*;
use material::{Phong, LightMaterial};
use light::Light;
use ray::Ray;
use objects::*;
use std::boxed::Box as StdBox;
use util::*;

docopt!(Args, "
Usage: raytrace <output> <width> <height>

Options:
    -a, --archive  Copy everything.
",
arg_width: u32, arg_height: u32);

const RAYS_PER_PIXEL: u32 = 100;

fn print_progress(progress: f64) {
    println!("\x1B[1A\x1B[2K{}%", progress * 100.);
}

fn render(scene: &Scene) -> RgbImage {
    let (width, height) = scene.camera().viewport();
    let mut img = RgbImage::new(width, height);
    let n_pixels = (width * height) as f64;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        println!("Doing pixel ({}, {})", x, y);
        print_progress((x + y * width) as f64 / n_pixels);
        *pixel = rgb_to_u8(&rgb_01_to_255(&scene.background()));

        let ray = scene.camera().pixel_ray((x, (height - 1 - y))).unwrap();

        // Find closest intersection
        let intersect_opt = scene.intersects(&ray);

        if let Some(intersect) = intersect_opt {
            match intersect.object {
                Object::Light(ref l) => {
                    // Paint the light with its diffuse color
                    *pixel = rgb_to_u8(&rgb_01_to_255(&l.light_material().diffuse_intensity));
                },
                Object::Surface(ref s) => {
                    // Cast light rays and compute diffuse component
                    let mut diffuse = Rgb { data: [0., 0., 0.] };
                    let surface_normal = intersect.face.normal();
                    for light in scene.lights() {
                        let mut i = 0;
                        loop {
                            let p = light.random_on_face();
                            let light_ray = Ray::between(intersect.position, p);
                            match scene.intersects(&light_ray) {
                                None => (),
                                Some(light_inter) => {
                                    if light_inter.object == Object::from_light(light.clone()) {
                                        let ray_color = light.shade(surface_normal, &intersect.object, &light_ray, &light_inter);
                                        diffuse = rgb_add(&diffuse, &ray_color);
                                    }
                                }
                            }
                            if i == RAYS_PER_PIXEL {
                                break;
                            }
                            i += 1;
                        }
                        diffuse = rgb_01_to_255(&rgb_div(&diffuse, RAYS_PER_PIXEL as f64));
                        let color = rgb_add(&diffuse, &rgb_01_to_255(&s.material().ambient_color()));
                        *pixel = rgb_to_u8(&color);
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
    let material_blue   = Phong::new(ambient, Rgb { data: [0.1, 0.2, 0.7] }, specular, 0.);
    let material_grey   = Phong::new(ambient, Rgb { data: [0.6, 0.6, 0.6] }, specular, 0.);
    let material_light  = LightMaterial::new(Rgb { data: [1.0, 1.0, 1.0] });
    let wall_left = Face::new(50., 50.,
                              Isometry3::new(Vector3::new(-2., 0., -2.), Vector3::y() * (PI / 2.)),
                              StdBox::new(material_grey.clone()));
    let wall_right = Face::new(50., 50.,
                               Isometry3::new(Vector3::new(2., 0., -2.), Vector3::y() * -(PI / 2.)),
                               StdBox::new(material_grey.clone()));
    let wall_back = Face::new(50., 50.,
                              Isometry3::new(Vector3::new(0., 0., -5.), Vector3::zero()),
                              StdBox::new(material_grey.clone()));
    let ceiling = Face::new(50., 50.,
                            Isometry3::new(Vector3::new(0., 3., 0.), Vector3::x() * PI / 2.),
                            StdBox::new(material_grey.clone()));
    let ground = Face::new(50., 50.,
                           Isometry3::new(Vector3::new(0., 0., -2.5), Vector3::x() * -(PI / 2.)),
                           StdBox::new(material_grey.clone()));
    let box1 = Box::new(Vector3::new(1., 1., 1.),
                        Isometry3::new(Vector3::new(1., 0.5, -4.), Vector3::zero()),
                        StdBox::new(material_blue));
    let light = Light::new(Face::new(0.5, 0.5,
                                     Isometry3::new(Vector3::new(0., 2.99, -3.), Vector3::x() * (PI / 2.)),
                                     StdBox::new(material_grey.clone())),
                           material_light.clone());

    let cam_transform = Isometry3::new(Vector3::new(0., 1.8, 0.), Vector3::zero());
    let cam = Perspective::new((width, height),
                               ((90.).to_radians(), (70.).to_radians()),
                               cam_transform);
    let scene = Scene::new(Rgb { data: [0.3, 0.3, 0.3] },
                           vec!(Object::from_surface(Surface::from_face(wall_left)),
                                Object::from_surface(Surface::from_face(wall_right)),
                                Object::from_surface(Surface::from_face(wall_back)),
                                Object::from_surface(Surface::from_face(ceiling)),
                                Object::from_surface(Surface::from_face(ground)),
                                Object::from_surface(Surface::from_box(box1)),
                                Object::from_light(light)),
                           StdBox::new(cam));

    println!("");
    let render = render(&scene);
    render.save(output_path).expect("Cannot save output image");
}
