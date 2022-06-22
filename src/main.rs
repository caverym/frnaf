#![feature(core_intrinsics)]

use assets::GameAssetsIo;
use bevy::{app::AppExit, prelude::*, window::WindowResizeConstraints, DefaultPlugins, asset::AssetPlugin};
use bevy_embasset::EmbassetPlugin;
use bevy_kira_audio::AudioPlugin;

mod assets;
mod counter;
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
            // mode: WindowMode::SizedFullscreen,
            resize_constraints: WindowResizeConstraints {
                min_width: 1280.0,
                min_height: 720.0,
                ..default()
            },
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<AssetPlugin, _>(EmbassetPlugin::new(|io| {
                io.add_handler(GameAssetsIo::new().into());
            }))
        })
        .add_plugin(AudioPlugin)
        .add_system(escape)
        .add_state(GameFrames::Frame17)
        .add_plugin(warning::WarningPlugin)
        .add_plugin(title::TitlePlugin)
        .insert_resource(Count::<usize>(1))
        .run();
}

fn escape(
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    game_state: Res<State<GameFrames>>,
) {
    if keys.just_pressed(KeyCode::Escape)
        && ![
            GameFrames::CreepyEnd,
            GameFrames::CreepyStart,
            GameFrames::Ad,
            GameFrames::WhatDay,
            GameFrames::Wait,
        ]
        .contains(game_state.current())
    {
        exit.send(AppExit);
    }
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

#[macro_export]
macro_rules! from_ct {
    ($x:expr, $y:expr, $w:expr, $h:expr, $ax:expr, $ay:expr, $z:expr) => {
        Vec3::new(
            (($x + $ax / -1.0) + $w / 2.0) - 1280.0 / 2.0,
            ((($y + $ay / -1.0) + $h / 2.0) - 720.0 / 2.0) / -1.0,
            $z,
        )
    };
}

#[derive(Component, Deref, DerefMut)]
struct Count<T: std::fmt::Display>(T);