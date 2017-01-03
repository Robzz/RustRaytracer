use light::Light;
use objects::*;
use ray::Ray;
use intersection::{ray_face, ray_box, Intersection};
use material::Material;
use std::boxed::Box as StdBox;

#[derive(Debug, Clone, PartialEq)]
pub enum Surface {
    Box(Box),
    Face(Face)
}

impl Surface {
    pub fn from_box(_box: Box) -> Surface {
        Surface::Box(_box)
    }

    pub fn from_face(face: Face) -> Surface {
        Surface::Face(face)
    }

    pub fn is_box(&self) -> bool {
        match self {
            &Surface::Box(_) => true,
            _ => false
        }
    }

    pub fn is_face(&self) -> bool {
        match self {
            &Surface::Face(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Light(Light),
    Surface(Surface)
}

impl Object {
    pub fn from_light(l: Light) -> Object {
        Object::Light(l)
    }

    pub fn from_surface(s: Surface) -> Object {
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

    pub fn as_surface(&self) -> Option<&Surface> {
        match self {
            &Object::Light(_) => None,
            &Object::Surface(ref s) => Some(s)
        }
    }
}

//impl PartialEq for Object {
    //fn eq(&self, other: &Object) -> bool {
        //match (self, other) {
            //(&Object::Light(ref l1), &Object::Light(ref l2)) => {
                //l1 == l2
            //},
            //(&Object::Surface(ref s1), &Object::Surface(ref s2)) => {
                //s1 == s2
            //}
            //(_, _) => false
        //}
    //}
//}

//impl Clone for Object {
    //fn clone(&self) -> Object {
        //match self {
            //&Object::Light(ref l) => Object::from_light(l.clone()),
            //&Object::Surface(ref s) => Object::from_surface(s.box_clone())
        //}
    //}
//}

impl Intersectable for Object {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            &Object::Light(ref l) => {
                match ray_face(ray, &l.face) {
                    Some(hit) => Some(Intersection::new(hit.0, hit.1, hit.2, self)),
                    None => None
                }
            }
            &Object::Surface(ref s) => {
                match s {
                    &Surface::Face(ref f) => {
                        match ray_face(ray, f) {
                            Some(hit) => Some(Intersection::new(hit.0, hit.1, hit.2, self)),
                            None => None
                        }
                    }
                    &Surface::Box(ref b) => {
                        match ray_box(ray, b) {
                            Some(hit) => Some(Intersection::new(hit.0, hit.1, hit.2, self)),
                            None => None
                        }
                    }
                }
            }
        }
    }
}

impl Drawable for Object {
    fn material(&self) -> &StdBox<Material> {
        match self {
            &Object::Light(ref l) => &l.face.material,
            &Object::Surface(ref s) => {
                match s {
                    &Surface::Face(ref f) => &f.material,
                    &Surface::Box(ref b) => &b.material
                }
            }
        }
    }

    fn box_clone(&self) -> StdBox<Drawable> {
        StdBox::new(self.clone())
    }
}
