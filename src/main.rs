use lib::engine::scene::{Scene, SceneObject};

mod lib;

fn main() {
    let mut scene = Scene::new();
    scene.spawn_object(SceneObject::SPHERE);
    scene.spawn_object(SceneObject::CUBE);
    scene.run();
}

#[cfg(test)]
mod test;
