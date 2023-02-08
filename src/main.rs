use lib::engine::scene::Scene;

mod lib;

fn main() {
    let scene = Scene::new();
    scene.run();
}

#[cfg(test)]
mod test;
