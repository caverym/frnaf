use std::{marker::PhantomData, path::PathBuf};

use crate::despawn_unload;

use super::GameFrames;
use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<ChannelOne>()
            .add_audio_channel::<ChannelTwo>()
            .add_audio_channel::<ChannelThree>()
            .add_system_set(SystemSet::on_enter(GameFrames::Title).with_system(setup))
            .add_system_set(SystemSet::on_update(GameFrames::Title))
            .add_system_set(
                SystemSet::on_exit(GameFrames::Title).with_system(despawn_unload::<OnTitleScreen>),
            );
    }
}

struct ChannelOne;

struct ChannelTwo;

struct ChannelThree;

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

#[derive(Component)]
pub struct TitleScreen;

#[derive(Component)]
pub struct OnTitleScreen;

#[derive(Deref, DerefMut)]
pub struct StaticTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    channelone: Res<AudioChannel<ChannelOne>>,
    channeltwo: Res<AudioChannel<ChannelTwo>>,
) {
    let mut map: HashMap<u16, Handle<Image>> = HashMap::new();

    for i in 12..=588 {
        if valid!(i) {
            load_image(&mut map, &asset_server, i)
        }
    }

    commands.spawn_bundle(ImageBundle {
        image: UiImage(map[&431].clone()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
            ..default()
        },
        ..default()
    });

    channelone.play(asset_server.load("sounds/[35] static2.wav"));
    channeltwo.play_looped(asset_server.load("sounds/[36] darkness music.wav"));
}

fn load_image(map: &mut HashMap<u16, Handle<Image>>, asset_server: &Res<AssetServer>, num: u16) {
    let path: PathBuf = format!("images/[1] 'title'/[unsorted]/{}.png", num).into();
    let image = asset_server.load(path);
    map.insert(num, image);
}
