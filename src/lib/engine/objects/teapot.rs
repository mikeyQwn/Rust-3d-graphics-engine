use crate::lib::engine::scene::{impl_render_for_mesh, Renderer};
use crate::lib::math::mesh::Mesh;
use crate::lib::math::projector::Projector;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::obj_parser::ObjParser;
use crate::lib::misc::window_manager::WindowManager;

pub struct Teapot {
    pub vertices: Vec<Vec3d>,
    pub mesh: Mesh,
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
        Self {
            vertices,
            mesh: Mesh::from_vertices_and_faces(unwrapped.geometric_vertices(), unwrapped.faces()),
        }
    }
}

impl_render_for_mesh!(Teapot);
