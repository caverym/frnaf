use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use bevy::prelude::*;

// const CONFIG: &str = "~/.config/freddy";

fn path() -> PathBuf {
    let p = format!("{}/.config/freddy", std::env::var("HOME").unwrap());
    PathBuf::from(p)
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component, serde::Serialize, serde::Deserialize)]
pub struct Config {
    freddy: Freddy,
}

impl Config {
    pub fn load() -> Config {
        let mut file: File = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path())
            .unwrap();
        let mut buf: String = String::new();
        file.read_to_string(&mut buf).unwrap();
        match toml::from_str(&buf) {
            Ok(conf) => {
                println!("config loaded");
                conf
            }
            Err(e) => {
                println!("creating new save: {}", e);
                Self {
                    freddy: Freddy {
                        level: 1,
                        beatgame: 0,
                        beat6: 0,
                        beat7: 0,
                    },
                }
            }
        }
    }

    pub fn save(&self) {
        let mut file: File = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path())
            .unwrap();

        let data = toml::to_vec(self).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write(&data).unwrap();
        file.sync_all().unwrap();
    }

    pub fn level(&self) -> u8 {
        self.freddy.level
    }

    pub fn set_level(&mut self, level: u8) {
        self.freddy.level = level;
        self.save();
    }

    pub fn beatgame(&self) -> bool {
        self.freddy.beatgame.to_bool()
    }

    pub fn set_beatgame(&mut self, beatgame: bool) {
        self.freddy.beatgame = u8::from_bool(beatgame);
        self.save();
    }

    pub fn beat_six(&self) -> bool {
        self.freddy.beat6.to_bool()
    }

    pub fn set_beat_six(&mut self, beat6: bool) {
        self.freddy.beat6 = u8::from_bool(beat6);
        self.save();
    }

    pub fn beat_seven(&self) -> bool {
        self.freddy.beat7.to_bool()
    }

    pub fn set_beat_seven(&mut self, beat7: bool) {
        self.freddy.beat7 = u8::from_bool(beat7);
        self.save();
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Freddy {
    level: u8,
    beatgame: u8,
    beat6: u8,
    beat7: u8,
}

impl Freddy {
    pub fn new(level: u8, beatgame: bool, beat6: bool, beat7: bool) -> Freddy {
        Freddy {
            level: level,
            beatgame: u8::from_bool(beatgame),
            beat6: u8::from_bool(beat6),
            beat7: u8::from_bool(beat7),
        }
    }
}

trait AsBool {
    fn to_bool(&self) -> bool;
    fn from_bool(b: bool) -> Self;
}

impl AsBool for u8 {
    fn to_bool(&self) -> bool {
        match self {
            0 => false,
            1 => true,
            i => panic!("can't convert '{}' to bool", i),
        }
    }

    fn from_bool(b: bool) -> Self {
        match b {
            true => 1,
            false => 0,
        }
    }
}

fn setup(mut commands: Commands) {
    let config = Config::load();
    config.save();

    commands.insert_resource(config);
}
