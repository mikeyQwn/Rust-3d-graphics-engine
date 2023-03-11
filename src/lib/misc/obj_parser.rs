use crate::lib::math::vec3d::Vec3d;
use std::fs;

#[derive(Default)]
pub struct ObjParser {
    geometric_vertices: Vec<Vec3d>,
    faces: Vec<Vec3d>,
}

impl ObjParser {
    pub fn parse(path: &str) -> Result<Self, &'static str> {
        let contents = fs::read_to_string(path);

        let res = match contents {
            Err(_err) => return Err("Could not read the file"),
            Ok(val) => val,
        };

        let lines = res.split('\n');
        let mut geometric_vertices = Vec::new();
        let mut faces = Vec::new();
        lines.for_each(|line| match line.chars().next() {
            Some('v') => {
                let parsed = Self::parse_numbers(line);
                let vec = Vec3d::from_vec_of_points(&parsed);
                geometric_vertices.push(vec);
            }
            Some('f') => {
                let parsed = Self::parse_numbers(line);
                let vec = Vec3d::from_vec_of_points(&parsed);
                faces.push(vec)
            }
            _ => {}
        });
        Ok(ObjParser {
            geometric_vertices,
            faces,
        })
    }

    pub fn geometric_vertices(&self) -> Vec<Vec3d> {
        self.geometric_vertices.clone()
    }

    pub fn faces(&self) -> Vec<Vec3d> {
        self.faces.clone()
    }

    fn parse_numbers(string: &str) -> Vec<f64> {
        let mut numbers: Vec<f64> = Vec::new();
        let splitted = string.split(' ');
        for item in splitted {
            if let Ok(val) = item.parse::<f64>() {
                numbers.push(val)
            }
        }
        numbers
    }
}
