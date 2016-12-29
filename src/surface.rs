use face::Face;

pub trait Surface {
    fn faces(&self) -> Vec<Face> {
        Vec::new()
    }
}
