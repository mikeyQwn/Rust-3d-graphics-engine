use super::triangle3d::Triangle3d;

#[derive(PartialEq, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle3d>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle3d>) -> Self {
        Self { triangles }
    }
}
