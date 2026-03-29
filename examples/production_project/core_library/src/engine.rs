/// A native vector type used in our game engine.
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// A pure-rust game engine.
#[derive(Default, Debug)]
pub struct GameEngine {
    pub object_count: u32,
}

impl GameEngine {
    pub fn new() -> Self {
        Self { object_count: 0 }
    }

    pub fn place_object(&mut self, _name: &str, _position: Vec2) {
        self.object_count += 1;
    }

    pub fn num_objects(&self) -> u32 {
        self.object_count
    }
}
