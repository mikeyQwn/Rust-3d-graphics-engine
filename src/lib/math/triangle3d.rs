use super::vec3d::Vec3d;

#[derive(PartialEq, Debug)]
pub struct Triangle3d {
    pub a: Vec3d,
    pub b: Vec3d,
    pub c: Vec3d,
}

impl Triangle3d {
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Self {
        Self { a, b, c }
    }

    pub fn new_from_coordinates(
        ax: f64,
        ay: f64,
        az: f64,
        bx: f64,
        by: f64,
        bz: f64,
        cx: f64,
        cy: f64,
        cz: f64,
    ) -> Self {
        Self {
            a: Vec3d::new(ax, ay, az),
            b: Vec3d::new(bx, by, bz),
            c: Vec3d::new(cx, cy, cz),
        }
    }
}
