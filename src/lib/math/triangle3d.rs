use super::vec3d::Vec3d;

#[derive(PartialEq, Debug, Default)]
pub struct Triangle3d {
    pub a: Vec3d,
    pub b: Vec3d,
    pub c: Vec3d,
}

impl Triangle3d {
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Self {
        Self { a, b, c }
    }

    pub fn new_from_coordinates(coords: &[f64]) -> Self {
        Self {
            a: Vec3d::new(coords[0], coords[1], coords[2]),
            b: Vec3d::new(coords[3], coords[4], coords[5]),
            c: Vec3d::new(coords[6], coords[7], coords[8]),
        }
    }
}
