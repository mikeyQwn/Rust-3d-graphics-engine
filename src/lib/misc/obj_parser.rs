use crate::lib::math::vec3d::Vec3d;
use std::fs;

#[derive(Default)]
pub struct ObjParser {
    geometric_vertices: Vec<Vec3d>,
}

impl ObjParser {
    pub fn parse(path: &String) -> Result<Self, &'static str> {
        let contents = fs::read_to_string(path);

        let res = match contents {
            Err(_err) => return Err("Could not read the file"),
            Ok(val) => val,
        };

        let lines = res.split('\n');
        let mut geometric_vertices = Vec::new();
        lines.for_each(|line| match line.chars().nth(0) {
            Some('v') => {
                let vec = Self::parse_numbers(&line.to_string());
                geometric_vertices.push(vec);
            }
            _ => {}
        });
        return Ok(ObjParser { geometric_vertices });
    }

    pub fn geometric_vertices(&self) -> Vec<Vec3d> {
        self.geometric_vertices.clone()
    }

    fn parse_numbers(string: &String) -> Vec3d {
        let mut numbers: Vec<f64> = Vec::new();
        let splitted = string.split(" ");
        for item in splitted {
            match item.parse::<f64>() {
                Ok(val) => numbers.push(val),
                _ => {}
            }
        }
        let mut result = Vec3d::default();
        for (i, item) in numbers.iter().enumerate() {
            match i {
                0 => result.x = *item,
                1 => result.y = *item,
                2 => result.z = *item,
                _ => {}
            }
        }
        result
    }
}
