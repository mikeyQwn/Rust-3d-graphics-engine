use crate::lib::engine::scene::Renderer;
use crate::lib::math::projector::Projector;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::obj_parser::ObjParser;
use crate::lib::misc::window_manager::WindowManager;

pub struct Teapot {
    pub vertices: Vec<Vec3d>,
}

impl Teapot {
    pub fn new() -> Self {
        let parsed = ObjParser::parse(&"assets/teapot.obj".to_string());
        if let Err(error) = parsed {
            panic!(
                "Could not find assets/teapot.obj due to the following error: {}",
                error
            );
        }
        let unwrapped = parsed.unwrap();
        let vertices = unwrapped.geometric_vertices();
        Self { vertices }
    }
}

impl Renderer for Teapot {
    fn render(&self, projector: &mut Projector, window_manager: &mut WindowManager) {
        for vertex in &self.vertices {
            let vertex_projected = projector.project(vertex);

            let vertex_x_normalized = (vertex_projected.x + 1.0) * 0.5;
            let vertex_y_normalized = 1.0 - (vertex_projected.y + 1.0) * 0.5;

            let (width, height) = window_manager.get_window_size();
            let width_f64 = width as f64;
            let height_f64 = height as f64;

            let vertex_x_scaled = (vertex_x_normalized * width_f64) as i32;
            let vertex_y_scaled = (vertex_y_normalized * height_f64) as i32;

            window_manager.fill_point(vertex_x_scaled, vertex_y_scaled, 5);
        }
    }
}
