use ray::Ray;
use material::Material;
use intersection::Intersection;
use objects::Object;

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Surface: Intersectable {
    fn material(&self) -> &Box<Material>;

    fn box_clone(&self) -> Box<Surface>;

    fn as_object(&self) -> Object {
        Object::from_surface(self.box_clone())
    }
}
