use super::GameFrames;
use crate::{assets::GameAssets, despawn_unload, from_ct, Count};
use bevy::{asset::Asset, input::mouse::MouseMotion, prelude::*};
use bevy_kira_audio::{AudioApp, AudioChannel};
use rand::Rng;
use std::{fmt::Display, path::PathBuf};

mod blipplugin;
mod freddyplugin;
mod staticplugin;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(freddyplugin::FreddyPlugin)
            .add_plugin(staticplugin::StaticPlugin)
            .add_plugin(blipplugin::BlipPlugin)
            .add_audio_channel::<ChannelOne>()
            .add_audio_channel::<ChannelTwo>()
            .add_audio_channel::<ChannelThree>()
            .add_system_set(
                SystemSet::on_enter(GameFrames::Title)
                    .with_system(setup)
                    .with_system(night_counter),
            )
            .add_system_set(
                SystemSet::on_update(GameFrames::Title)
                    .with_system(game_buttons)
                    .with_system(move_arrow),
            )
            .add_system_set(
                SystemSet::on_exit(GameFrames::Title).with_system(despawn_unload::<OnTitleScreen>),
            );
    }
}

#[derive(Component)]
struct Arrow;

#[derive(Component)]
struct NightDisplay;

struct ChannelOne;

struct ChannelTwo;

struct ChannelThree;

#[derive(Component)]
pub struct TitleScreen;

#[derive(Component)]
pub struct OnTitleScreen;

#[derive(Debug, Component, Clone, Copy)]
pub enum ArrowHover {
    NewGame,
    Continue,
    ThNight,
    CustomNight,
}

pub struct ArrowLocation(ArrowHover);

fn night_counter(
    mut commands: Commands,
    asr: Res<AssetServer>,
    night: Res<Count<usize>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let sheet = load!(asr, NightNumberTitleFrames);
    let texture_atlas = TextureAtlas::from_grid(sheet, Vec2::new(14.0, 17.0), 14, 1);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: from_ct!(263.0, 535.0, 14.0, 17.0, 14.0, 17.0, 1.0),
                ..default()
            },
            sprite: TextureAtlasSprite {
                index: night.0,
                ..default()
            },
            ..default()
        })
        .insert(NightDisplay);
}

fn game_buttons(
    query: Query<(&ArrowHover, &Interaction)>,
    mut arrow_location: ResMut<ArrowLocation>,
) {
    for (loc, i) in query.iter() {
        match i {
            Interaction::Clicked => todo!("{:?} click", loc),
            Interaction::Hovered => arrow_location.0 = *loc,
            Interaction::None => {}
        }
    }
}

fn move_arrow(mut query: Query<(&mut Transform, &Arrow)>, arrow_location: Res<ArrowLocation>) {
    for (mut trans, _) in query.iter_mut() {
        let mut new = from_ct!(275.0, 420.0, 203.0, 33.0, 101.0, 16.0, 1.0);

        match arrow_location.0 {
            ArrowHover::NewGame => new.x -= 150.0,
            ArrowHover::Continue => trans.translation = new,
            ArrowHover::ThNight => new.x -= 165.0,
            ArrowHover::CustomNight => new.x -= 192.0,
        }

        trans.translation = new;
    }
}

fn setup(
    mut commands: Commands,
    asr: Res<AssetServer>,
    channelone: Res<AudioChannel<ChannelOne>>,
    channeltwo: Res<AudioChannel<ChannelTwo>>,
) {
    // commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ArrowLocation(ArrowHover::Continue));

    // title
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T444), // load_image(&asset_server, 444),
        transform: Transform {
            translation: from_ct!(172.0, 68.0, 201.0, 212.0, -3.0, -11.0, 1.0),
            ..default()
        },

        ..default()
    });

    // new game
    commands
        .spawn_bundle(SpriteBundle {
            texture: load!(asr, T448),
            transform: Transform {
                translation: from_ct!(275.0, 420.0, 203.0, 33.0, 101.0, 16.0, 1.0),
                ..default()
            },

            ..default()
        })
        .insert(ArrowHover::NewGame);

    // continue
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T449),
        transform: Transform {
            translation: from_ct!(275.0, 492.0, 204.0, 34.0, 102.0, 17.0, 1.0),
            ..default()
        },

        ..default()
    });

    // 6th night
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T443),
        transform: Transform {
            translation: from_ct!(285.0, 571.0, 227.0, 44.0, 113.0, 22.0, 1.0),
            ..default()
        },

        ..default()
    });

    // custom night
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T526),
        transform: Transform {
            translation: from_ct!(324.0, 639.0, 306.0, 44.0, 153.0, 22.0, 1.0),
            ..default()
        },

        ..default()
    });

    // version
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T588),
        transform: Transform {
            translation: from_ct!(26.0, 682.0, 89.0, 15.0, -1.0, -7.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, Credit),
        transform: Transform {
            translation: from_ct!(1001.0, 642.0, 274.0, 66.0, 0.0, -5.0, 1.0),
            ..default()
        },

        ..default()
    });

    // arrow
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T450),
        transform: Transform {
            translation: from_ct!(132.0, 493.0, 43.0, 26.0, 21.0, 13.0, 1.0),
            ..default()
        },

        ..default()
    });

    // night
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T475),
        transform: Transform {
            translation: from_ct!(174.0, 512.0, 63.0, 22.0, -1.0, -5.0, 1.0),
            ..default()
        },

        ..default()
    });

    channelone.play(load!(asr, Static2));
    channeltwo.play_looped(load!(asr, DarknessMusic));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
