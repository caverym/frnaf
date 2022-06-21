use std::path::PathBuf;

use crate::despawn_unload;

use super::GameFrames;
use bevy::{prelude::*, utils::HashMap};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameFrames::Title).with_system(setup))
            .add_system_set(SystemSet::on_update(GameFrames::Title))
            .add_system_set(
                SystemSet::on_exit(GameFrames::Title).with_system(despawn_unload::<OnTitleScreen>),
            );
    }
}

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut map: HashMap<u16, Handle<Image>> = HashMap::new();

    for i in 12..=588 {
        if valid!(i) {
            load_image(&mut map, &asset_server, i)
        }
    }
}

fn load_image(map: &mut HashMap<u16, Handle<Image>>, asset_server: &Res<AssetServer>, num: u16) {
    let path: PathBuf = format!("images/[1] 'title'/[unsorted]/{}.png", num).into();
    let image = asset_server.load(path);
    map.insert(num, image);
}
