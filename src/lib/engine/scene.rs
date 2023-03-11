use crate::lib::engine::objects;
use crate::lib::math::projector::Projector;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::window_manager::WindowManager;

#[allow(unused)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Renderer>>,
    window_manger: WindowManager,
}

pub enum SceneObject {
    Cube,
    Sphere,
    Teapot,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            objects: Vec::new(),
            window_manger: WindowManager::new(),
        }
    }

    pub fn spawn_object(&mut self, object: SceneObject) {
        self.objects.push(match object {
            SceneObject::Cube => Box::new(objects::cube::Cube::new(10.0, 10.0, 10.0)),
            SceneObject::Sphere => Box::new(objects::sphere::Sphere::new(100)),
            SceneObject::Teapot => Box::new(objects::teapot::Teapot::new()),
        })
    }

    pub fn run(mut self) -> i32 {
        let _start_time = std::time::Instant::now();
        let mut is_running = true;
        let mut projector = crate::lib::math::projector::Projector::default();
        let mut prev_frame = std::time::Instant::now();
        'running: loop {
            let _elapsed = std::time::Instant::now()
                .duration_since(prev_frame)
                .as_nanos();
            prev_frame = std::time::Instant::now();
            self.window_manger
                .change_color(crate::lib::misc::window_manager::WMColor::Black);
            self.window_manger.clear_window();
            self.window_manger
                .change_color(crate::lib::misc::window_manager::WMColor::White);
            self.objects
                .iter_mut()
                .for_each(|object| object.render(&mut projector, &mut self.window_manger));
            self.window_manger.update_window(&mut is_running);
            if !is_running {
                break 'running;
            }
        }

        0
    }
}

#[derive(Default)]
#[allow(unused)]
struct Camera {
    position: Vec3d,
    rotation: Vec3d,
}

#[allow(unused)]
impl Camera {
    pub fn new(position: Vec3d, rotation: Vec3d) -> Self {
        Self { position, rotation }
    }
}

pub trait Renderer {
    fn render(&self, projector: &mut Projector, window_manger: &mut WindowManager);
}

macro_rules! impl_render_for_mesh {
    ($n:ty) => {
        impl Renderer for $n {
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
    };
}

macro_rules! impl_render_for_vertices {
    ($s:ty) => {
        impl Renderer for $s {
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
    };
}

pub(crate) use impl_render_for_mesh;
pub(crate) use impl_render_for_vertices;
