//! Resources definitions.

use crate::game::tetromino::TetrominoType;
use bevy::prelude::{Plugin, Resource};
use std::collections::{BTreeMap, HashMap, LinkedList};

use super::{components::MatrixPosition, tetromino::Tetromino};

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

#[derive(Resource)]
pub struct StartPosition(pub MatrixPosition);

#[derive(Resource)]
pub struct HoldOnQueueResoure {
    pub start_pos: MatrixPosition,
    pub values: LinkedList<Tetromino>,
}

impl HoldOnQueueResoure {
    pub fn new(start_position: &StartPosition) -> Self {
        let mut values = LinkedList::new();
        for _ in 0..5 {
            let new = Tetromino::new(start_position.0.clone());
            values.push_front(new);
        }
        HoldOnQueueResoure {
            start_pos: start_position.0.clone(),
            values,
        }
    }

    pub fn pop_push(&mut self) -> Tetromino {
        let new = Tetromino::new(self.start_pos);
        let value = self.values.pop_front();
        self.values.push_back(new);
        value.unwrap()
    }

    pub fn first(&self) -> Option<&Tetromino> {
        self.values.front()
    }
}

pub enum ScoreAction {
    Single,
    Double,
    Triple,
    Tetris,
    MiniTSpin,
    MiniTSpinSingle,
    TSpin,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    BackToBackBonus,
    SoftDrop(usize),
    HardDrop(usize),
}
