#[derive(PartialEq, Debug, Default)]
pub struct Matrix4x4 {
    pub matrix: [[f64; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self { matrix }
    }

    pub fn new_from_coordinates(
        a11: f64,
        a12: f64,
        a13: f64,
        a14: f64,
        a21: f64,
        a22: f64,
        a23: f64,
        a24: f64,
        a31: f64,
        a32: f64,
        a33: f64,
        a34: f64,
        a41: f64,
        a42: f64,
        a43: f64,
        a44: f64,
    ) -> Self {
        Self {
            matrix: [
                [a11, a12, a13, a14],
                [a21, a22, a23, a24],
                [a31, a32, a33, a34],
                [a41, a42, a43, a44],
            ],
        }
    }
}
