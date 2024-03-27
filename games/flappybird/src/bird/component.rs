use bevy::prelude::*;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
    pub gravity: f32,
}

impl Default for Bird {
    fn default() -> Self {
        Self {
            velocity: 0.0,
            gravity: 4.0,
        }
    }
}
