use bevy::prelude::*;
use rand::Rng;

use crate::GameFrames;

pub struct BlipPlugin;

impl Plugin for BlipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameFrames::Title).with_system(setup))
        .add_system_set(
            SystemSet::on_update(GameFrames::Title)
                .with_system(blip_changer)
                .with_system(blip_op_changer)
                .with_system(blip_vis_changer)
        );
    }
}

#[derive(Component, Deref, DerefMut)]
struct BlipTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct BlipOpTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct BlipVisTimer(Timer);

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

fn setup(
    mut commands: Commands,
    asr: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
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
}
