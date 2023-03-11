use crate::lib::engine::scene::{impl_render_for_mesh, Renderer};
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

impl_render_for_mesh!(Cube);
