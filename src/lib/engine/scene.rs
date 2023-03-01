use crate::lib::math::projector::Projector;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::window_manager::WindowManager;

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Renderer>>,
    window_manger: WindowManager,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            objects: Vec::new(),
            window_manger: WindowManager::new(),
        }
    }
    pub fn run(mut self) -> i32 {
        self.window_manger.fill_screen_saver_data();
        let start_time = std::time::Instant::now();
        let mut is_running = true;
        'running: loop {
            self.window_manger
                .fill_window(start_time.elapsed().as_millis() as f64 / 25.0);
            self.window_manger.update_window(&mut is_running);
            if !is_running {
                break 'running;
            }
        }

        return 0;
    }
}

struct Camera {
    position: Vec3d,
    rotation: Vec3d,
}

impl Camera {
    pub fn new(position: Vec3d, rotation: Vec3d) -> Self {
        return Self { position, rotation };
    }
}

impl Default for Camera {
    fn default() -> Self {
        return Self {
            position: Vec3d::default(),
            rotation: Vec3d::default(),
        };
    }
}

pub trait Renderer {
    fn render(&self, projector: &mut Projector, window_manger: &mut WindowManager) -> ();
}
