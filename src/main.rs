#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate rand;
extern crate rustc_serialize;
extern crate rust_raytracer;

use image::*;
use num_traits::*;
use nalgebra::*;

use std::boxed::Box as StdBox;
use std::f64::consts::PI;
use std::path::Path;

use rust_raytracer::camera::Perspective;
use rust_raytracer::light::Light;
use rust_raytracer::material::{Phong, LightMaterial};
use rust_raytracer::objects::*;
use rust_raytracer::ray::Ray;
use rust_raytracer::scene::Scene;
use rust_raytracer::util::*;

docopt!(Args, "
Usage: raytrace <output> <width> <height> <N> <B>",
arg_width: u32, arg_height: u32, arg_N: u32, arg_B: u32);

fn print_progress(progress: f64) {
    println!("\x1B[1A\x1B[2K{}%", progress * 100.);
}

fn reflection_ray(scene: &Scene, ray: &Ray, bounces: u32) -> Rgb<f64> {
    let mut pixel = Rgb { data: [0., 0., 0.] };
    let intersect_opt = scene.intersects(ray);
    if let Some(intersect) = intersect_opt {
        match intersect.object {
            Object::Light(ref l) => {
                // Return the light diffuse color and stop bouncing
                pixel = l.light_material().diffuse_intensity;
            },
            Object::Surface(ref s) => {
                // Cast light ray and compute Phong shading
                let surface_normal = intersect.normal;
                for light in scene.lights() {
                    let p = light.random_on_face();
                    let light_ray = Ray::between(intersect.position, p);
                    match scene.intersects(&light_ray) {
                        None => (),
                        Some(light_inter) => {
                            if light_inter.object == Object::from_light(light.clone()) {
                                let ray_diffuse_color = light.shade_diffuse(surface_normal, &intersect.object, &light_ray, &light_inter);
                                let ray_specular_color = light.shade_specular(scene.camera().eye_position(), surface_normal, &intersect.object, &light_ray, &light_inter);
                                let mut color = rgb_add(&ray_diffuse_color, &s.material().ambient_color());
                                color = rgb_add(&color, &ray_specular_color);

                                // Cast a reflection ray for glossy reflection
                                if bounces != 0 {
                                    let l = light_ray.direction.normalize();
                                    let dln = l.dot(&surface_normal);
                                    let r = 2. * dln * surface_normal - l;
                                    let refl_ray = Ray::new(intersect.position, random_in_cone(r, (30.).to_radians()));
                                    let reflection_color = reflection_ray(scene, &refl_ray, bounces - 1);
                                    color = rgb_add(&color, &reflection_color);
                                    pixel = color;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    else {
        pixel = scene.background();
    }
    pixel
}

fn ray_energy(scene: &Scene, ray: &Ray, bounces: u32) -> Rgb<f64> {
    // Find closest intersection
    let intersect_opt = scene.intersects(&ray);
    let mut pixel = Rgb { data: [0., 0., 0.] };

    if let Some(intersect) = intersect_opt {
        match intersect.object {
            Object::Light(ref l) => {
                // Paint the light with its diffuse color
                pixel = l.light_material().diffuse_intensity;
            },
            Object::Surface(ref s) => {
                // Cast light ray and compute Phong shading
                let surface_normal = intersect.normal;
                for light in scene.lights() {
                    let p = light.random_on_face();
                    let light_ray = Ray::between(intersect.position, p);
                    match scene.intersects(&light_ray) {
                        None => (),
                        Some(light_inter) => {
                            if light_inter.object == Object::from_light(light.clone()) {
                                let ray_diffuse_color = light.shade_diffuse(surface_normal, &intersect.object, &light_ray, &light_inter);
                                let ray_specular_color = light.shade_specular(scene.camera().eye_position(), surface_normal, &intersect.object, &light_ray, &light_inter);
                                let mut color = rgb_add(&ray_diffuse_color, &s.material().ambient_color());
                                color = rgb_add(&color, &ray_specular_color);

                                // Cast a reflection ray for glossy reflection
                                //if bounces != 0 {
                                    //let l = light_ray.direction.normalize();
                                    //let dln = l.dot(&surface_normal);
                                    //let r = 2. * dln * surface_normal - l;
                                    //let refl_ray = Ray::new(intersect.position, random_in_cone(r, (30.).to_radians()));
                                    //let reflection_color = rgb_div(&reflection_ray(scene, &refl_ray, bounces), bounces as f64);
                                    //color = rgb_add(&color, &reflection_color);
                                    //color = rgb_div(&color, 2.);
                                //}

                                pixel = color;
                            }
                        }
                    }
                }
            }
        }
    }
    else {
        pixel = scene.background();
    }
    pixel
}

fn correct_gamma(p: &Rgb<f64>) -> Rgb<f64> {
    const A: f64 = 0.055;
    p.map(|c| {
        match c <= 0.0031308 {
            true => 12.92 * c,
            false => (1. + A) * c.powf(1. / 2.4) - A
        }
    })
}

fn render(scene: &Scene, n: u32, bounces: u32) -> RgbImage {
    let (width, height) = scene.camera().viewport();
    let mut img = RgbImage::new(width, height);
    let n_pixels = (width * height) as f64;
    let mut i = 0;

    for (x, y_inverted, pixel) in img.enumerate_pixels_mut() {
        print_progress(i as f64 / n_pixels);

        let step = 1. / (n + 1) as f64;
        let y = height - 1 - y_inverted;
        let mut yf = (y as f64) + step / 2.;
        let mut energy = Rgb { data: [0., 0., 0.] };
        while yf < (y + 1) as f64 {
            let mut xf = (x as f64) + step / 2.;
            while xf < (x + 1) as f64 {
                let ray = scene.camera().pixel_ray((xf, yf)).unwrap();
                let ray_energy = ray_energy(scene, &ray, bounces);
                energy = rgb_add(&energy, &ray_energy);
                xf += step;
            }
            yf += step;
        }
        energy = rgb_clamp_0_1(&rgb_div(&energy, (n * n) as f64));
        *pixel = rgb_to_u8(&rgb_01_to_255(&correct_gamma(&energy)));
        //*pixel = rgb_to_u8(&rgb_01_to_255(&energy));
        i += 1;
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
    let material_blue = Phong::new(ambient,
                                   Rgb { data: [0.1, 0.2, 1.] },
                                   Rgb { data: [0.4, 0.4, 0.4] },
                                   2.);
    let material_red  = Phong::new(ambient,
                                   Rgb { data: [1., 0.2, 0.1] },
                                   Rgb { data: [0.6, 0.6, 0.6] },
                                   2.);
    let material_grey = Phong::new(ambient,
                                   Rgb { data: [0.6, 0.6, 0.6] },
                                   Rgb { data: [0.6, 0.6, 0.6] },
                                   2.);
    let material_light  = LightMaterial::new(Rgb { data: [0.6, 0.6, 0.6] },
                                             Rgb { data: [0.25, 0.25, 0.25] });
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
    let box2 = Box::new(Vector3::new(1., 2., 1.),
                        Isometry3::new(Vector3::new(-1., 1., -4.), Vector3::y() * PI / 4.),
                        StdBox::new(material_red));
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
                                Object::from_surface(Surface::from_box(box2)),
                                Object::from_light(light)),
                           StdBox::new(cam));

    println!("");
    let render = render(&scene, args.arg_N, args.arg_B);
    render.save(output_path).expect("Cannot save output image");
}
