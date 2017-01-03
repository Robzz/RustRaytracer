use ray::Ray;
use material::Material;
use intersection::Intersection;

// TODO: rewrite the trait
//
pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Drawable: Intersectable {
    fn material(&self) -> &Box<Material>;

    fn box_clone(&self) -> Box<Drawable>;
}
