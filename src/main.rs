use bevy::{
    prelude::*,
    DefaultPlugins, render::camera::ScalingMode,
};

mod warning;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameFrames {
    Frame17,
    Title,
    WhatDay,
    Frame1,
    Died,
    Freddy,
    NextDay,
    Wait,
    GameOver,
    TheEnd,
    Ad,
    TheEnd2,
    Customize,
    TheEnd3,
    CreepyStart,
    CreepyEnd,
    EndOfDemo,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        // .add_plugin(TiledCameraPlugin)
        .add_startup_system(setup)
        .add_state(GameFrames::Frame17)
        .add_plugin(warning::WarningPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = UiCameraBundle::default();
    camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
    commands.spawn_bundle(camera);
    // commands.spawn_bundle()
    // commands.spawn_bundle(camera);
    // commands.spawn_bundle(UiCameraBundle::default());
    // commands.spawn_bundle(TiledCameraBundle::new().with_target_resolution(1, [1280, 720]));
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
