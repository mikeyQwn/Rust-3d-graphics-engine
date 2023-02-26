use super::triangle3d::Triangle3d;

#[derive(PartialEq, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle3d>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle3d>) -> Self {
        Self { triangles }
    }

    pub fn get_cube_mesh() -> Self {
        Mesh::new(vec![
            //South
            Triangle3d::new_from_coordinates(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0),
            Triangle3d::new_from_coordinates(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0),
            //East
            Triangle3d::new_from_coordinates(1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0),
            Triangle3d::new_from_coordinates(1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),
            //North
            Triangle3d::new_from_coordinates(1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0),
            Triangle3d::new_from_coordinates(1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0),
            //West
            Triangle3d::new_from_coordinates(0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0),
            Triangle3d::new_from_coordinates(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            //Top
            Triangle3d::new_from_coordinates(0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0),
            Triangle3d::new_from_coordinates(0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0),
            //Bottom
            Triangle3d::new_from_coordinates(1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
            Triangle3d::new_from_coordinates(1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0),
        ])
    }

    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        for mut triangle in self.triangles.iter_mut() {
            triangle.a.x += dx;
            triangle.a.y += dy;
            triangle.a.z += dz;
            triangle.b.x += dx;
            triangle.b.y += dy;
            triangle.b.z += dz;
            triangle.c.x += dx;
            triangle.c.y += dy;
            triangle.c.z += dz;
        }
    }
}
