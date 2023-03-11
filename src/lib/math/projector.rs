use super::{matrix4x4::Matrix4x4, vec3d::Vec3d};

#[derive(PartialEq, Debug)]
pub struct Projector {
    projection_matrix: Matrix4x4,
    far: f64,
    near: f64,
    right: f64,
    left: f64,
    top: f64,
    bottom: f64,
}

impl Projector {
    pub fn new(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> Projector {
        let mut projection_matrix = Matrix4x4::default();
        let (right, left, top, bottom) = Projector::get_perspective(fov, aspect_ratio, near);
        Self::fill_projection_matrix(&mut projection_matrix, right, left, top, bottom, far, near);

        Projector {
            projection_matrix,
            right,
            left,
            top,
            bottom,
            far,
            near,
        }
    }

    pub fn get_perspective(fov: f64, aspect_ratio: f64, near: f64) -> (f64, f64, f64, f64) {
        let right = near * (fov / 2.0 * std::f64::consts::PI / 180.0).tan();
        let left = -right;
        let top = right / aspect_ratio;
        let bottom = -top;
        (right, left, top, bottom)
    }

    fn fill_projection_matrix(
        projection_matrix: &mut Matrix4x4,
        right: f64,
        left: f64,
        top: f64,
        bottom: f64,
        far: f64,
        near: f64,
    ) {
        projection_matrix.matrix[0][0] = 2.0 * near / (right - left);
        projection_matrix.matrix[1][1] = 2.0 * near / (top - bottom);
        projection_matrix.matrix[2][0] = (right + left) / (right - left);
        projection_matrix.matrix[2][1] = (top + bottom) / (top - bottom);
        projection_matrix.matrix[2][2] = -(far + near) / (far - near);
        projection_matrix.matrix[2][3] = -1.0;
        projection_matrix.matrix[3][2] = -2.0 * far * near / (far - near);
    }

    pub fn project(&mut self, point: &Vec3d) -> Vec3d {
        let mut result = Vec3d {
            x: point.x * self.projection_matrix.matrix[0][0]
                + point.y * self.projection_matrix.matrix[1][0]
                + point.z * self.projection_matrix.matrix[2][0]
                + self.projection_matrix.matrix[3][0],
            y: point.x * self.projection_matrix.matrix[0][1]
                + point.y * self.projection_matrix.matrix[1][1]
                + point.z * self.projection_matrix.matrix[2][1]
                + self.projection_matrix.matrix[3][1],
            z: point.x * self.projection_matrix.matrix[0][2]
                + point.y * self.projection_matrix.matrix[1][2]
                + point.z * self.projection_matrix.matrix[2][2]
                + self.projection_matrix.matrix[3][2],
        };
        let w: f64 = result.x * self.projection_matrix.matrix[0][3]
            + result.y * self.projection_matrix.matrix[1][3]
            + result.z * self.projection_matrix.matrix[2][3]
            + self.projection_matrix.matrix[3][3];
        if w != 1.0 {
            result.x /= w;
            result.y /= w;
            result.z /= w;
        }
        result
    }
}

impl Default for Projector {
    fn default() -> Self {
        Self::new(90.0, 16.0 / 9.0, 1.0, 1000.0)
    }
}
