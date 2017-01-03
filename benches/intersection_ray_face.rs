#![feature(test)]

extern crate test;
extern crate rust_raytracer;
extern crate nalgebra;
extern crate image;

use test::Bencher;
use nalgebra::*;
use image::Rgb;

use rust_raytracer::objects::{Face, Intersectable};
use rust_raytracer::material::Phong;
use rust_raytracer::ray::Ray;

#[bench]
fn test_intersection_face_intersects(b: &mut Bencher) -> () {
    let mat = Phong::new(Rgb { data: [ 0., 0., 0. ] },
                         Rgb { data: [0., 0., 0.] },
                         Rgb { data: [0., 0., 0.] },
                         2.);
    let face = Face::new(1., 1., Isometry3::new(Vector3::new(0., 0., -1.), Vector3::new(0., 0. ,0.)), Box::new(mat));
    let ray = Ray::new(Point3::new(0., 0., 0.), -Vector3::z());
    b.iter(|| {
        face.intersects(&ray);
    });
}

#[bench]
fn test_intersection_face_no_intersects(b: &mut Bencher) -> () {
    let mat = Phong::new(Rgb { data: [ 0., 0., 0. ] },
                         Rgb { data: [0., 0., 0.] },
                         Rgb { data: [0., 0., 0.] },
                         2.);
    let face = Face::new(1., 1., Isometry3::new(Vector3::new(0., 0., 1.), Vector3::new(0., 0. ,0.)), Box::new(mat));
    let ray = Ray::new(Point3::new(0., 0., 0.), -Vector3::z());
    b.iter(|| {
        face.intersects(&ray);
    });
}
