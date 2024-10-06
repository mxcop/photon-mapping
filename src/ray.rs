use glam::Vec2;

#[derive(Clone, Copy)]
pub struct Ray {
    pub o: Vec2,
    pub d: Vec2,
}

impl Ray {
    pub fn new(origin: Vec2, dir: Vec2) -> Self {
        Self {
            o: origin,
            d: dir
        }
    }
}
