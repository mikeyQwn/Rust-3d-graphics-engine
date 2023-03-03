use crate::lib::math::projector::Projector;
use crate::lib::math::vec3d::Vec3d;
use crate::lib::misc::window_manager::WindowManager;

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Renderer>>,
    window_manger: WindowManager,
}

pub enum SceneObject {
    CUBE,
    SPHERE,
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
            SceneObject::CUBE => Box::new(crate::lib::engine::objects::cube::Cube::new(
                10.0, 10.0, 10.0,
            )),
            SceneObject::SPHERE => Box::new(crate::lib::engine::objects::sphere::Sphere::new(100)),
        })
    }

    pub fn run(mut self) -> i32 {
        let start_time = std::time::Instant::now();
        let mut is_running = true;
        let mut projector = crate::lib::math::projector::Projector::default();
        let mut prev_frame = std::time::Instant::now();
        'running: loop {
            let elapsed = std::time::Instant::now()
                .duration_since(prev_frame)
                .as_nanos();
            prev_frame = std::time::Instant::now();
            self.window_manger
                .change_color(crate::lib::misc::window_manager::WMColor::BLACK);
            self.window_manger.clear_window();
            self.window_manger
                .change_color(crate::lib::misc::window_manager::WMColor::WHITE);
            self.objects
                .iter_mut()
                .for_each(|object| object.render(&mut projector, &mut self.window_manger));
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
