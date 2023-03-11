#[derive(PartialEq, Debug)]
pub struct Vec2d {
    x: f64,
    y: f64,
}

#[allow(unused)]
impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
