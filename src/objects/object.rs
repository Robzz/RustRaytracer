use light::Light;
use objects::*;
use ray::Ray;
use intersection::Intersection;
use material::Material;
use std::boxed::Box as StdBox;

pub enum Object {
    Light(Light),
    Surface(StdBox<Surface>)
}

impl Object {
    pub fn from_light(l: Light) -> Object {
        Object::Light(l)
    }

    pub fn from_surface(s: StdBox<Surface>) -> Object {
        Object::Surface(s)
    }

    pub fn is_light(&self) -> bool {
        match self {
            &Object::Light(_) => true,
            &Object::Surface(_) => false
        }
    }

    pub fn is_surface(&self) -> bool {
        match self {
            &Object::Light(_) => false,
            &Object::Surface(_) => true
        }
    }

    pub fn as_light(&self) -> Option<&Light> {
        match self {
            &Object::Light(ref l) => Some(l),
            &Object::Surface(_) => None
        }
    }

    pub fn as_surface(&self) -> Option<&StdBox<Surface>> {
        match self {
            &Object::Light(_) => None,
            &Object::Surface(ref s) => Some(s)
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Object {
        match self {
            &Object::Light(ref l) => Object::from_light(l.clone()),
            &Object::Surface(ref s) => Object::from_surface(s.box_clone())
        }
    }
}

impl Intersectable for Object {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            &Object::Light(ref l) => l.intersects(ray),
            &Object::Surface(ref s) => s.intersects(ray)
        }
    }
}
