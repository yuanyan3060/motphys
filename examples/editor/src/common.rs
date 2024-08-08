use std::collections::HashMap;

use bevy::prelude::{Entity, Resource, Transform};

#[derive(Resource)]
pub enum Picked {
    None,
    Cuboid(Entity),
    Joint(Entity)
}

#[derive(Resource, Default)]
pub struct TransformStore {
    pub inner: HashMap<Entity, Transform>
}

#[derive(Resource, PartialEq, Eq)]
pub enum GameState {
    Editor,
    Play
}

impl GameState {
    pub fn is_editor(&self) -> bool {
        self == &Self::Editor
    }

    pub fn is_play(&self) -> bool {
        self == &Self::Play
    }
}