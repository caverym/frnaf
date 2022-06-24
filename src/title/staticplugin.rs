use bevy::prelude::*;
use rand::Rng;

use super::OnTitleScreen;

pub struct StaticPlugin;

impl Plugin for StaticPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(static_changer)
            .add_system(static_op_changer);
    }
}

#[derive(Component, Deref, DerefMut)]
struct StaticTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct StaticOpTimer(Timer);

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

fn setup(
    mut commands: Commands,
    asr: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
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
        .insert(StaticOpTimer(Timer::from_seconds(0.09, true)))
        .insert(OnTitleScreen);
}
