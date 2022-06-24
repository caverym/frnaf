use bevy::prelude::*;
use rand::Rng;

pub struct FreddyPlugin;

impl Plugin for FreddyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(freddy_changer)
            .add_system(freddy_op_changer);
    }
}

#[derive(Component, Deref, DerefMut)]
struct FreddyOpTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct FreddyTimer(Timer);

fn setup(
    mut commands: Commands,
    asr: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
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
