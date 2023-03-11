#[derive(PartialEq, Debug, Default)]
pub struct Matrix4x4 {
    pub matrix: [[f64; 4]; 4],
}

#[allow(unused)]
impl Matrix4x4 {
    pub fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self { matrix }
    }

    pub fn new_from_coordinates(coords: &[f64]) -> Self {
        Self {
            matrix: [
                [coords[0], coords[1], coords[2], coords[3]],
                [coords[4], coords[5], coords[6], coords[7]],
                [coords[8], coords[9], coords[10], coords[11]],
                [coords[12], coords[13], coords[14], coords[15]],
            ],
        }
    }
}
