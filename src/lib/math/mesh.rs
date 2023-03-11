use super::{triangle3d::Triangle3d, vec3d::Vec3d};

#[derive(PartialEq, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle3d>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle3d>) -> Self {
        Self { triangles }
    }

    pub fn from_vertices_and_faces(vertices: Vec<Vec3d>, faces: Vec<Vec3d>) -> Self {
        let triangles = faces
            .iter()
            .map(|face| {
                let a = match vertices.get(face.x as usize - 1) {
                    Some(a) => a.clone(),
                    None => {
                        eprintln!(
                            "An error occured while trying to combine object vertices and faces"
                        );
                        Vec3d::default()
                    }
                };
                let b = match vertices.get(face.y as usize - 1) {
                    Some(b) => b.clone(),
                    None => {
                        eprintln!(
                            "An error occured while trying to combine object vertices and faces"
                        );
                        Vec3d::default()
                    }
                };
                let c = match vertices.get(face.z as usize - 1) {
                    Some(c) => c.clone(),
                    None => {
                        eprintln!(
                            "An error occured while trying to combine object vertices and faces"
                        );
                        Vec3d::default()
                    }
                };

                Triangle3d::new(a, b, c)
            })
            .collect();

        Mesh { triangles }
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
        for triangle in self.triangles.iter_mut() {
            triangle.a.translate(dx, dy, dz);
            triangle.b.translate(dx, dy, dz);
            triangle.c.translate(dx, dy, dz);
        }
    }
}
