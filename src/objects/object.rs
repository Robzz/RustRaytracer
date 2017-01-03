use light::Light;
use objects::*;
use ray::Ray;
use intersection::Intersection;
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

impl Intersectable for Surface {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            &Surface::Box(ref b) => b.intersects(ray),
            &Surface::Face(ref f) => f.intersects(ray)
        }
    }
}

impl Drawable for Surface {
    fn material(&self) -> StdBox<Material> {
        match self {
            &Surface::Box(ref b) => b.material(),
            &Surface::Face(ref f) => f.material()
        }
    }

    fn box_clone(&self) -> StdBox<Drawable> {
        match self {
            &Surface::Box(ref b) => b.box_clone(),
            &Surface::Face(ref f) => f.box_clone()
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
            &Object::Light(ref l) => l.intersects(ray),
            &Object::Surface(ref s) => s.intersects(ray)
        }
    }
}

impl Drawable for Object {
    fn material(&self) -> StdBox<Material> {
        match self {
            &Object::Light(ref l) => l.material(),
            &Object::Surface(ref s) => s.material()
        }
    }

    fn box_clone(&self) -> StdBox<Drawable> {
        match self {
            &Object::Light(ref l) => l.box_clone(),
            &Object::Surface(ref s) => s.box_clone()
        }
    }
}
