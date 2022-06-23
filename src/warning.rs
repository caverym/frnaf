use std::intrinsics::unlikely;

use super::GameFrames;
use crate::{assets::GameAssets, despawn_unload};
use bevy::prelude::*;

pub struct WarningPlugin;

impl Plugin for WarningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameFrames::Frame17).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameFrames::Frame17)
                    .with_system(fade)
                    .with_system(countdown)
                    .with_system(keyboard)
                    .with_system(mouse),
            )
            .add_system_set(
                SystemSet::on_exit(GameFrames::Frame17)
                    .with_system(despawn_unload::<OnWarningScreen>),
            );
    }
}

#[derive(Component)]
pub struct WarningScreen;

#[derive(Component)]
pub struct OnWarningScreen;

// #[derive(Deref, DerefMut)]
// pub struct WarningTimer(Timer);

#[derive(Deref, DerefMut)]
pub struct WarningTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ImageBundle {
            color: UiColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            image: UiImage(asset_server.load(GameAssets::Warning.path())),
            style: Style {
                //margin: Rect {
                //    top: Val::Px(249.0),
                //    left: Val::Px(426.0),
                //    bottom: Val::Px(373.0),
                // right: Val::Px(891.0),
                //    ..default()
                //},
                // margin: Rect { top: Val::Px(249.0), left: Val::Px(426.0), ..default()}, // right: Val::Px(891.0), ..default() }, // bottom: Val::Px(373.0) },
                // position: Rect { right: Val::Px(389.0), top: Val::Px(249.0), ..default() },
                position: Rect {
                    //right: Val::Percent(30.390625),
                    top: Val::Percent(34.583333333),
                    bottom: Val::Percent(48.194444444),
                    left: Val::Percent(33.28125),
                    ..default()
                },
                position_type: PositionType::Absolute,
                // size: Size::new(Val::Px(465.0), Val::Px(124.0)),
                ..default()
            },
            ..default()
        })
        .insert(OnWarningScreen);

    commands.insert_resource(WarningTimer(Timer::from_seconds(2.0, false)));
    commands.insert_resource(FadeTimer(Timer::from_seconds(1.1, false)));
}

#[derive(Deref, DerefMut)]
pub struct FadeTimer(Timer);

fn countdown(time: Res<Time>, mut timer: ResMut<WarningTimer>, mut fade_timer: ResMut<FadeTimer>) {
    if timer.tick(time.delta()).finished() || timer.paused() {
        fade_timer.tick(time.delta());
    }
}

fn fade(
    timer: ResMut<FadeTimer>,
    mut query: Query<&mut UiColor>,
    mut game_state: ResMut<State<GameFrames>>,
) {
    if timer.percent_left() == 100.0 {
        return;
    }

    if unlikely(timer.finished()) {
        for mut color in query.iter_mut() {
            color.0.set_a(0.0);
            game_state.set(GameFrames::Title).unwrap();
        }
    }

    for mut color in query.iter_mut() {
        color.0.set_a(1.0 * timer.percent_left());
    }
}

fn keyboard(
    mut game_state: ResMut<State<GameFrames>>,
    keys: Res<Input<KeyCode>>,
    mut timer: ResMut<WarningTimer>,
) {
    if keys.just_pressed(KeyCode::Return) || keys.just_pressed(KeyCode::NumpadEnter) {
        if timer.finished() {
            game_state.set(GameFrames::Title).unwrap();
        } else {
            timer.pause();
            // timer.set_duration(Duration::from_secs_f32(2.0));
        }
    }
}

fn mouse(
    mut game_state: ResMut<State<GameFrames>>,
    mouse: Res<Input<MouseButton>>,
    mut timer: ResMut<WarningTimer>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if timer.finished() {
            game_state.set(GameFrames::Title).unwrap();
        } else {
            timer.pause();
            // timer.set_duration(Duration::from_secs_f32(2.0));
        }
    }
}
