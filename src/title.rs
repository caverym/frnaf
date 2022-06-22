use std::path::PathBuf;
use crate::{despawn_unload, from_ct};
use super::GameFrames;
use bevy::{prelude::*, asset::Asset};
use bevy_kira_audio::{AudioApp, AudioChannel};
use rand::Rng;

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
                    .with_system(blip_vis_changer),
            )
            .add_system_set(
                SystemSet::on_exit(GameFrames::Title).with_system(despawn_unload::<OnTitleScreen>),
            );
    }
}

struct ChannelOne;

struct ChannelTwo;

struct ChannelThree;

/*
macro_rules! valid {
    ($number:expr) => {
        !($number == 19
            || $number == 21
            || (23..430).contains(&$number)
            || $number == 451
            || $number == 446
            || $number == 447
            || (453..475).contains(&$number)
            || $number == 476
            || (478..526).contains(&$number)
            || (527..572).contains(&$number)
            || (573..588).contains(&$number))
    };
}
*/

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    channelone: Res<AudioChannel<ChannelOne>>,
    channeltwo: Res<AudioChannel<ChannelTwo>>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 444),
        transform: Transform {
            translation: from_ct!(172.0, 68.0, 201.0, 212.0, -3.0, -11.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 448),
        transform: Transform {
            translation: from_ct!(275.0, 420.0, 203.0, 33.0, 101.0, 16.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 449),
        transform: Transform {
            translation: from_ct!(275.0, 492.0, 204.0, 34.0, 102.0, 17.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 443),
        transform: Transform {
            translation: from_ct!(285.0, 571.0, 227.0, 44.0, 113.0, 22.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 526),
        transform: Transform {
            translation: from_ct!(324.0, 639.0, 306.0, 44.0, 153.0, 22.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 588),
        transform: Transform {
            translation: from_ct!(26.0, 682.0, 89.0, 15.0, -1.0, -7.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&asset_server, 588),
        transform: Transform {
            translation: from_ct!(26.0, 682.0, 89.0, 15.0, -1.0, -7.0, 1.0),
            ..default()
        },

        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("credit.png"),
        transform: Transform {
            translation: from_ct!(1001.0, 642.0, 274.0, 66.0, 0.0, -5.0, 1.0),
            ..default()
        },

        ..default()
    });

    let sheet = asset_server.load("StaticFrames.png");
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

    let sheet = asset_server.load("FreddyFrames.png");
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

    let sheet = asset_server.load("BlipFrames.png");
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

    channelone.play(asset_server.load("sounds/[35] static2.wav"));
    channeltwo.play_looped(asset_server.load("sounds/[36] darkness music.wav"));
}

fn load_image<T: Asset>(asset_server: &Res<AssetServer>, num: u16) -> Handle<T> {
    let path: PathBuf = format!("images/[1] 'title'/[unsorted]/{}.png", num).into();
    asset_server.load(path)
}
