use bevy::prelude::*;

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub struct Counter<T: std::fmt::Display + PartialEq + Eq + PartialOrd + Ord> {
    texture_atlas: Handle<TextureAtlas>,
    index: T,
}
