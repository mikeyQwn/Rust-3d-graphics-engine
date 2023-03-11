#[derive(PartialEq, Debug, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(unused)]
impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec_of_points(points_vec: &[f64]) -> Self {
        let mut result = Vec3d::default();
        for (i, item) in points_vec.iter().enumerate() {
            match i {
                0 => result.x = *item,
                1 => result.y = *item,
                2 => result.z = *item,
                _ => {}
            }
        }
        result
    }

    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}

impl Default for Vec3d {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
