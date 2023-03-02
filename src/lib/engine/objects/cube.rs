use crate::lib::engine::scene::Renderer;
use crate::lib::math::mesh::Mesh;
use crate::lib::math::projector::Projector;
use crate::lib::math::triangle3d::Triangle3d;
use crate::lib::misc::window_manager::WindowManager;

pub struct Cube {
    pub mesh: Mesh,
}

impl Cube {
    pub fn new(width: f64, height: f64, length: f64) -> Self {
        Self {
            mesh: Mesh::new(vec![
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
            ]),
        }
    }
}

impl Renderer for Cube {
    fn render(&self, projector: &mut Projector, window_manager: &mut WindowManager) {
        for triangle in &self.mesh.triangles {
            let a_projected = projector.project(&triangle.a);
            let b_projected = projector.project(&triangle.b);
            let c_projected = projector.project(&triangle.c);

            let a_x_normalized = (a_projected.x + 1.0) * 0.5;
            let a_y_normalized = 1.0 - (a_projected.y + 1.0) * 0.5;

            let b_x_normalized = (b_projected.x + 1.0) * 0.5;
            let b_y_normalized = 1.0 - (b_projected.y + 1.0) * 0.5;

            let c_x_normalized = (c_projected.x + 1.0) * 0.5;
            let c_y_normalized = 1.0 - (c_projected.y + 1.0) * 0.5;

            let (width, height) = window_manager.get_window_size();
            let width_f64 = width as f64;
            let height_f64 = height as f64;

            let a_x_scaled = (a_x_normalized * width_f64) as i32;
            let a_y_scaled = (a_y_normalized * height_f64) as i32;

            let b_x_scaled = (b_x_normalized * width_f64) as i32;
            let b_y_scaled = (b_y_normalized * height_f64) as i32;

            let c_x_scaled = (c_x_normalized * width_f64) as i32;
            let c_y_scaled = (c_y_normalized * height_f64) as i32;

            window_manager.draw_line(a_x_scaled, a_y_scaled, b_x_scaled, b_y_scaled);
            window_manager.draw_line(b_x_scaled, b_y_scaled, c_x_scaled, c_y_scaled);
            window_manager.draw_line(a_x_scaled, a_y_scaled, c_x_scaled, c_y_scaled);
        }
    }
}
