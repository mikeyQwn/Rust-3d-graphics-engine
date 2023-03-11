use lib::engine::scene::{Scene, SceneObject};

mod lib;

fn main() {
    let mut scene = Scene::new();
    scene.spawn_object(SceneObject::Sphere);
    scene.spawn_object(SceneObject::Cube);
    scene.spawn_object(SceneObject::Teapot);

    scene.run();
}

#[cfg(test)]
mod test;
