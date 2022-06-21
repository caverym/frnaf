#![feature(core_intrinsics)]

use bevy::{
    prelude::*,
    window::{WindowMode, WindowResizeConstraints},
    DefaultPlugins,
};

mod title;
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
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Five Nights at Freddy's".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::SizedFullscreen,
            resize_constraints: WindowResizeConstraints {
                min_width: 1280.0,
                min_height: 720.0,
                ..default()
            },
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(GameFrames::Frame17)
        .add_plugin(warning::WarningPlugin)
        .add_plugin(title::TitlePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_unload<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    commands: Commands,
    asset_server: Res<AssetServer>,
) {
    despawn_screen(to_despawn, commands);
    asset_server.mark_unused_assets();
    asset_server.free_unused_assets();
}
