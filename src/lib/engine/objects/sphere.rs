use crate::lib::engine::scene::{impl_render_for_vertices, Renderer};
use crate::lib::math::projector::Projector;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::window_manager::WindowManager;

pub struct Sphere {
    pub vertices: Vec<Vec3d>,
}

impl Sphere {
    pub fn new(number_of_points: usize) -> Self {
        Self {
            vertices: Self::get_fibonacci_sphere_vertices(number_of_points),
        }
    }

    fn get_fibonacci_sphere_vertices(number_of_points: usize) -> Vec<Vec3d> {
        let mut vertices: Vec<Vec3d> = Vec::with_capacity(number_of_points);
        let phi = 3.1415 * (3.0 - 5.0_f64.sqrt());
        for i in 0..number_of_points {
            let y = 1.0 - (i as f64 / (number_of_points as f64 - 1.0)) * 2.0;
            let radius = (1.0 - y * y).sqrt();

            let theta = phi * i as f64;

            let x = theta.cos() * radius;
            let z = theta.sin() * radius;
            vertices.push(Vec3d::new(x, y, z));
        }
        vertices
    }
}

impl_render_for_vertices!(Sphere);
