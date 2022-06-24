use std::time::Duration;

use super::GameState;
use crate::{despawn_unload, from_ct, save::Config};
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel};
use bevy_tweening::{
    lens::TransformPositionLens, Animator, EaseMethod, Tween, TweeningPlugin,
    TweeningType,
};

mod blipplugin;
mod freddyplugin;
mod menuplugin;
mod staticplugin;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(freddyplugin::FreddyPlugin)
            .add_plugin(staticplugin::StaticPlugin)
            .add_plugin(blipplugin::BlipPlugin)
            .add_plugin(TweeningPlugin)
            .add_audio_channel::<ChannelOne>()
            .add_audio_channel::<ChannelTwo>()
            .add_audio_channel::<ChannelThree>()
            .add_system_set(
                SystemSet::on_enter(GameState::Title)
                    .with_system(setup)
                    .with_system(night_counter),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Title)
                    .with_system(show_hide)
                    .with_system(button_system)
                    .with_system(arrow_keys),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Title).with_system(despawn_unload::<OnTitleScreen>),
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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum ArrowLocation {
    NewGame,
    Continue,
    SThNight,
    CustomNight,
}

#[derive(Component)]
struct NewGameButton;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct SThNightButton;

#[derive(Component)]
struct CustomButton;

fn night_counter(
    mut commands: Commands,
    asr: Res<AssetServer>,
    config: Res<Config>,
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
                index: config.level() as usize,
                ..default()
            },
            ..default()
        })
        .insert(NightDisplay)
        .insert(OnTitleScreen);
}

fn show_hide(glob: Res<ArrowLocation>, mut q: Query<&mut Visibility, With<NightDisplay>>) {
    if glob.is_changed() {
        for mut q in q.iter_mut() {
            q.is_visible = *glob == ArrowLocation::Continue
        }
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children, &ArrowLocation), With<Button>>,
    mut visa: Query<&mut Visibility>,
    mut glob: ResMut<ArrowLocation>,
    config: Res<Config>,
) {
    for (interaction, children, loc) in interaction_query.iter_mut() {
        let mut vis = visa.get_mut(children[0]).unwrap();

        if glob.is_changed() {
            if *glob == *loc {
                vis.is_visible = true;
            } else {
                vis.is_visible = false;
            }
        }

        match *interaction {
            Interaction::Clicked => todo!(),
            Interaction::Hovered => {
                match (
                    config.beatgame(),
                    config.beat_six(),
                    config.beat_seven(),
                    *loc,
                ) {
                    (false, false, false, ArrowLocation::NewGame | ArrowLocation::Continue) => {
                        vis.is_visible = true;
                        *glob = *loc;
                    },
                    (true, false, false, ArrowLocation::NewGame | ArrowLocation::Continue | ArrowLocation::SThNight) => {
                        vis.is_visible = true;
                        *glob = *loc;
                    },
                    (true, true, _, ArrowLocation::NewGame | ArrowLocation::Continue | ArrowLocation::SThNight | ArrowLocation::CustomNight) => {
                        vis.is_visible = true;
                        *glob = *loc;
                    },
                    _ => {},
                }
            }
            Interaction::None => {
                    if *glob != *loc {
                        vis.is_visible = false;
                    }
            }
        }
    }
}

fn arrow_keys(keys: Res<Input<KeyCode>>, mut glob: ResMut<ArrowLocation>, config: Res<Config>) {
    if keys.just_pressed(KeyCode::Up) {
        match *glob {
            ArrowLocation::NewGame => {
                *glob = {
                    match (config.beatgame(), config.beat_six(), config.beat_seven()) {
                        (false, false, false) => ArrowLocation::Continue,
                        (true, false, false) => ArrowLocation::SThNight,
                        (true, true, _) => ArrowLocation::CustomNight,
                        _ => panic!("invalid button config"),
                    }
                }
            }
            ArrowLocation::Continue => *glob = ArrowLocation::NewGame,
            ArrowLocation::SThNight => *glob = ArrowLocation::Continue,
            ArrowLocation::CustomNight => *glob = ArrowLocation::SThNight,
        }
    }

    if keys.just_pressed(KeyCode::Down) {
        match *glob {
            ArrowLocation::NewGame => *glob = ArrowLocation::Continue,
            ArrowLocation::Continue => {
                *glob = {
                    match (config.beatgame(), config.beat_six(), config.beat_seven()) {
                        (false, false, false) => ArrowLocation::NewGame,
                        _ => ArrowLocation::SThNight,
                    }
                }
            }
            ArrowLocation::SThNight => {
                *glob = {
                    match (config.beatgame(), config.beat_six(), config.beat_seven()) {
                        (false, false, false) => ArrowLocation::NewGame,
                        (true, false, false) => ArrowLocation::NewGame,
                        (true, true, _) => ArrowLocation::CustomNight,
                        _ => panic!("invalid button config"),
                    }
                }
            }
            ArrowLocation::CustomNight => *glob = ArrowLocation::NewGame,
        }
    }
}

fn setup(
    mut commands: Commands,
    asr: Res<AssetServer>,
    config: Res<Config>,
    channelone: Res<AudioChannel<ChannelOne>>,
    channeltwo: Res<AudioChannel<ChannelTwo>>,
) {
    commands.insert_resource(match config.level() {
        1 => ArrowLocation::NewGame,
        _ => ArrowLocation::Continue,
    });
    // Title
    commands
        .spawn_bundle(ImageBundle {
            image: UiImage(load!(asr, T444)),
            style: Style {
                position: Rect {
                    left: Val::Px(175.0),
                    bottom: Val::Px(429.0),
                    ..default()
                },
                size: Size {
                    width: Val::Px(201.0),
                    height: Val::Px(212.0),
                },
                ..default()
            },
            ..default()
        })
        .insert(OnTitleScreen);

    // new game
    commands
        .spawn_bundle(ButtonBundle {
            image: UiImage(load!(asr, T448)),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(174.0),
                    // top: Val::Px(404.0),
                    bottom: Val::Px(283.0),
                    ..default()
                },
                size: Size {
                    width: Val::Px(203.0),
                    height: Val::Px(33.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
                image: UiImage(load!(asr, T450)),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(-70.0),
                        bottom: Val::Px(4.0),
                        ..default()
                    },
                    size: Size {
                        width: Val::Px(43.0),
                        height: Val::Px(26.0),
                    },
                    ..default()
                },
                visibility: Visibility {
                    is_visible: config.level() == 1,
                },
                ..default()
            });
        })
        .insert(OnTitleScreen)
        .insert(ArrowLocation::NewGame);

    // continue
    commands
        .spawn_bundle(ButtonBundle {
            image: UiImage(load!(asr, T449)),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(173.0),
                    bottom: Val::Px(211.0),
                    ..default()
                },
                size: Size {
                    width: Val::Px(204.0),
                    height: Val::Px(34.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
                image: UiImage(load!(asr, T450)),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(-69.0),
                        bottom: Val::Px(4.0),
                        ..default()
                    },
                    size: Size {
                        width: Val::Px(43.0),
                        height: Val::Px(26.0),
                    },
                    ..default()
                },
                visibility: Visibility {
                    is_visible: config.level() > 1,
                },
                ..default()
            });
        })
        .insert(OnTitleScreen)
        .insert(ArrowLocation::Continue);

    // 6th night
    commands
        .spawn_bundle(ButtonBundle {
            image: UiImage(load!(asr, T443)),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(172.0),
                    bottom: Val::Px(127.0),
                    ..default()
                },
                size: Size {
                    width: Val::Px(227.0),
                    height: Val::Px(44.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
                image: UiImage(load!(asr, T450)),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(-73.0),
                        bottom: Val::Px(9.0),
                        ..default()
                    },
                    size: Size {
                        width: Val::Px(43.0),
                        height: Val::Px(26.0),
                    },
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                ..default()
            });
        })
        .insert(OnTitleScreen)
        .insert(ArrowLocation::SThNight);

    // custom night
    commands
        .spawn_bundle(ButtonBundle {
            image: UiImage(load!(asr, T526)),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(171.0),
                    bottom: Val::Px(59.0),
                    ..default()
                },
                size: Size {
                    width: Val::Px(306.0),
                    height: Val::Px(44.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
                image: UiImage(load!(asr, T450)),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        //left: Val::Px(111.0),
                        //bottom: Val::Px(68.0),
                        left: Val::Px(-60.0),
                        bottom: Val::Px(9.0),
                        ..default()
                    },
                    size: Size {
                        width: Val::Px(43.0),
                        height: Val::Px(26.0),
                    },
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                ..default()
            });
        })
        .insert(OnTitleScreen)
        .insert(ArrowLocation::CustomNight);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(OnTitleScreen);

    // version
    commands
        .spawn_bundle(SpriteBundle {
            texture: load!(asr, T588),
            transform: Transform {
                translation: from_ct!(26.0, 682.0, 89.0, 15.0, -1.0, -7.0, 1.0),
                ..default()
            },

            ..default()
        })
        .insert(OnTitleScreen);

    commands
        .spawn_bundle(SpriteBundle {
            texture: load!(asr, Credit),
            transform: Transform {
                translation: from_ct!(1001.0, 642.0, 274.0, 66.0, 0.0, -5.0, 1.0),
                ..default()
            },

            ..default()
        })
        .insert(OnTitleScreen);

    // night
    commands
        .spawn_bundle(SpriteBundle {
            texture: load!(asr, T475),
            transform: Transform {
                translation: from_ct!(174.0, 512.0, 63.0, 22.0, -1.0, -5.0, 1.0),
                ..default()
            },

            ..default()
        })
        .insert(OnTitleScreen)
        .insert(NightDisplay);

    let animation = Tween::new(
        EaseMethod::Linear,
        TweeningType::Loop,
        Duration::from_secs(20),
        TransformPositionLens {
            start: Vec3::new(0.0, 392.0, 5.0),
            end: Vec3::new(0.0, -392.0, 5.0),
        },
    );

    commands
        .spawn_bundle(SpriteBundle {
            texture: load!(asr, T452),
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.125),
                ..default()
            },
            ..default()
        })
        .insert(OnTitleScreen)
        .insert(Animator::new(animation));

    channelone.play(load!(asr, Static2));
    channeltwo.play_looped(load!(asr, DarknessMusic));

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(OnTitleScreen);
}
