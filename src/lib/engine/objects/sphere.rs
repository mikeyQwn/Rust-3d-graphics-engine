use crate::lib::engine::scene::Renderer;
use crate::lib::math::projector::Projector;
use crate::lib::math::triangle3d::Triangle3d;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::window_manager::WindowManager;

struct Sphere {
    vertices: Vec<Vec3d>,
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

impl Renderer for Sphere {
    fn render(&self, projector: &mut Projector, window_manager: &mut WindowManager) {
        for vertex in &self.vertices {
            let vertex_projected = projector.project(vertex);

            let vertex_x_normalized = (vertex_projected.x + 1.0) * 2.0;
            let vertex_y_normalized = (vertex_projected.y + 1.0) * 2.0;

            let (width, height) = window_manager.get_window_size();
            let width_f64 = width as f64;
            let height_f64 = height as f64;

            let vertex_x_scaled = (vertex_x_normalized * width_f64) as i32;
            let vertex_y_scaled = (vertex_y_normalized * height_f64) as i32;

            window_manager.fill_point(vertex_x_scaled, vertex_y_scaled, 5)
        }
    }
}
