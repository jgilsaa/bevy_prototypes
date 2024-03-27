use bevy::prelude::*;

#[derive(Resource)]
pub struct PipeSpawner {
    pub timer: Timer,
}

impl Default for PipeSpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct ApproachingPipe;
