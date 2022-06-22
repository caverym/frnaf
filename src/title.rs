use super::GameFrames;
use crate::{despawn_unload, from_ct, Count, assets::GameAssets};
use bevy::{asset::Asset, prelude::*, input::mouse::MouseMotion};
use bevy_kira_audio::{AudioApp, AudioChannel};
use rand::Rng;
use std::{fmt::Display, path::PathBuf};

macro_rules! load {
    ($asr:expr, $name:ident) => {
        $asr.load(crate::assets::GameAssets::$name)
    };
}

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app //.add_plugin(AnimationPlugin::default())
            .add_audio_channel::<ChannelOne>()
            .add_audio_channel::<ChannelTwo>()
            .add_audio_channel::<ChannelThree>()
            .add_system_set(SystemSet::on_enter(GameFrames::Title).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameFrames::Title)
                    // .with_system(freddy_timer)
                    .with_system(freddy_changer)
                    .with_system(freddy_op_changer)
                    .with_system(static_changer)
                    .with_system(static_op_changer)
                    .with_system(blip_changer)
                    .with_system(blip_op_changer)
                    .with_system(blip_vis_changer)
                    .with_system(night_counter)
                    .with_system(game_buttons)
                    .with_system(move_arrow)
                    ,
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

#[derive(Component, Deref, DerefMut)]
pub struct StaticTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct StaticOpTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct FreddyTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BlipTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BlipOpTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct FreddyOpTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BlipVisTimer(Timer);

#[derive(Debug, Component, Clone, Copy)]
pub enum ArrowHover {
    NewGame,
    Continue,
    ThNight,
    CustomNight,
}

pub struct ArrowLocation(ArrowHover);

fn night_counter(mut query: Query<(&NightDisplay, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>, night: Res<Count<usize>>) {
    for (_, mut sprite, _) in query.iter_mut() {
        sprite.index = night.0;
    }
}

fn blip_op_changer(time: Res<Time>, mut query: Query<(&mut BlipOpTimer, &mut TextureAtlasSprite)>) {
    let mut rng = rand::thread_rng();
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.color = Color::rgba(
                1.0,
                1.0,
                1.0,
                1.0 - ((rng.gen_range(0..100) as f32 + 100.0) / 255.0),
            )
        }
    }
}

fn freddy_op_changer(
    time: Res<Time>,
    mut query: Query<(&mut FreddyOpTimer, &mut TextureAtlasSprite)>,
) {
    let mut rng = rand::thread_rng();
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0 - (rng.gen_range(0..250) as f32 / 255.0))
        }
    }
}

// 0: mouth closed
// 1: mouth open
// 2: turned
// 3: exo
// 1-(rng.gen_range(0..100)+100);
fn freddy_changer(
    time: Res<Time>,
    mut query: Query<(
        &mut FreddyTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    let mut rng = rand::thread_rng();
    for (mut timer, mut sprite, _) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let number = rng.gen_range(0..100);

            let number = match number {
                0..=96 => 0,
                97 => 1,
                98 => 2,
                99 => 3,
                _ => panic!(),
            };

            sprite.index = number;
        }
    }
}

fn blip_vis_changer(
    time: Res<Time>,
    mut query: Query<(&mut BlipVisTimer, &mut Visibility, &Handle<TextureAtlas>)>,
) {
    let mut rng = rand::thread_rng();
    for (mut timer, mut visibility, _) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if rng.gen_range(0..3) == 1 {
                *visibility = Visibility { is_visible: true };
            } else {
                *visibility = Visibility { is_visible: false };
            }
        }
    }
}

fn static_changer(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut StaticTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len() % 8;
        }
    }
}

fn static_op_changer(
    time: Res<Time>,
    mut query: Query<(&mut StaticOpTimer, &mut TextureAtlasSprite)>,
) {
    let mut rng = rand::thread_rng();
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let alpha = 50 + rng.gen_range(0..99);
            sprite.color = Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0 - (alpha as f32 / 255.0),
            }
        }
    }
}

fn blip_changer(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut BlipTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len() % 8;
        }
    }
}

fn game_buttons(
    query: Query<(&ArrowHover, &Interaction)>,
    mut arrow_location: ResMut<ArrowLocation>,
) {
    for (loc, i) in query.iter() {
        match i {
            Interaction::Clicked => todo!("{:?} click", loc),
            Interaction::Hovered => arrow_location.0 = *loc,
            Interaction::None => {},
        }
    }
}

fn move_arrow(
    mut query: Query<(
        &mut Transform,
        &Arrow,
    )>,
    arrow_location: Res<ArrowLocation>,
) {
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
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ArrowLocation(ArrowHover::Continue));

    // title
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T444),// load_image(&asset_server, 444),
        transform: Transform {
            translation: from_ct!(172.0, 68.0, 201.0, 212.0, -3.0, -11.0, 1.0),
            ..default()
        },

        ..default()
    });

    // new game
    commands.spawn_bundle(SpriteBundle {
        texture: load!(asr, T448),
        transform: Transform {
            translation: from_ct!(275.0, 420.0, 203.0, 33.0, 101.0, 16.0, 1.0),
            ..default()
        },

        ..default()
    }).insert(ArrowHover::NewGame);

    /*commands.spawn_bundle(ButtonBundle {
        image: UiImage(load_image(&asset_server, 448)),
        transform: Transform {
            translation: from_ct!(275.0, 420.0, 203.0, 33.0, 101.0, 16.0, 1.0),
            ..default()
        },
        visibility: Visibility { is_visible: true }
        ..default()
    }).insert(ArrowHover::NewGame);*/

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

    let sheet = load!(asr, NightNumberTitleFrames);
    let texture_atlas = TextureAtlas::from_grid(sheet, Vec2::new(14.0, 17.0), 14, 1);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation:  from_ct!(263.0, 535.0, 14.0, 17.0, 14.0, 17.0, 1.0),
            ..default()
        },
        ..default()
    }).insert(NightDisplay);

    let sheet = load!(asr, StaticFrames);
    let texture_atlas = TextureAtlas::from_grid(sheet, Vec2::new(1280.0, 720.0), 3, 3);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),

            ..default()
        })
        .insert(StaticTimer(Timer::from_seconds(0.0168350168, true)))
        .insert(StaticOpTimer(Timer::from_seconds(0.09, true)));

    let sheet = load!(asr, FreddyFrames);
    let texture_atlas = TextureAtlas::from_grid(sheet, Vec2::new(1280.0, 720.0), 2, 2);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(FreddyTimer(Timer::from_seconds(0.08, true)))
        .insert(FreddyOpTimer(Timer::from_seconds(0.3, true)));

    let sheet = load!(asr, BlipFrames);
    let texture_atlas = TextureAtlas::from_grid(sheet, Vec2::new(1280.0, 720.0), 3, 3);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(BlipTimer(Timer::from_seconds(0.166666667, true)))
        .insert(BlipOpTimer(Timer::from_seconds(0.08, true)))
        .insert(BlipVisTimer(Timer::from_seconds(0.3, true)));

    channelone.play(load!(asr, Static2));
    channeltwo.play_looped(load!(asr, DarknessMusic));
}
