//! Resources definitions.

use crate::game::tetromino::TetrominoType;
use bevy::prelude::{Plugin, Resource};
use std::collections::{BTreeMap, HashMap};

#[derive(Resource)]
pub struct ImagePathResources(BTreeMap<TetrominoType, &'static str>);

pub struct ImageLoadPlugin;

impl Plugin for ImageLoadPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut map = BTreeMap::new();
        map.insert(TetrominoType::I, "red.png");
        map.insert(TetrominoType::J, "orange.png");
        map.insert(TetrominoType::L, "yellow.png");
        map.insert(TetrominoType::S, "green.png");
        map.insert(TetrominoType::T, "cyan.png");
        map.insert(TetrominoType::Z, "blue.png");
        map.insert(TetrominoType::O, "purple.png");
        app.insert_resource(ImagePathResources(map));
    }
}

impl ImagePathResources {
    /// A function to get Image `Path` from resources
    pub fn get_path(&self, ty: TetrominoType) -> &'static str {
        self.0.get(&ty).unwrap()
    }

    /// A function to return background Image `Path`
    pub fn get_black(&self) -> &'static str {
        "black.png"
    }

    /// A function to get collections of resources
    pub fn hgetall(&self) -> &BTreeMap<TetrominoType, &'static str> {
        &self.0
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: usize,
    pub cleared_lines: HashMap<&'static str, u32>,
}
