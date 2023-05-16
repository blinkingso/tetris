use std::cmp::min;

use bevy::{
    prelude::{EventReader, Query, Res, ResMut},
    text::Text,
};

use crate::game::{
    components::GameArea,
    global::{calculate_score, MAX_LEVEL},
    matrix::Matrix,
    resources::{Score, ScoreEvent},
};

pub fn update_level(mut matrix: ResMut<Matrix>, mut level: Query<(&mut Text, &GameArea)>) {
    if matrix.lines_cleared >= matrix.level * 10 {
        matrix.level = min(matrix.level + 1, MAX_LEVEL);
        matrix.lines_cleared = 0;

        for (mut text, ga) in level.iter_mut() {
            if *ga == GameArea::TextLevel {
                text.sections[0].value = format!("{:07}", matrix.level);
            }
            if *ga == GameArea::TextLines {
                text.sections[0].value = format!("{:07}", 0);
            }
        }
    }
}

pub fn update_score(
    matrix: Res<Matrix>,
    mut read_event: EventReader<ScoreEvent>,
    mut res_score: ResMut<Score>,
    mut text: Query<(&mut Text, &GameArea)>,
) {
    for ev in read_event.into_iter() {
        let score = calculate_score(matrix.level as i32, ev.action);
        if let Some(cleared_lines) = res_score.cleared_lines.get_mut(&ev.action) {
            *cleared_lines += ev.cleared_lines;
        } else {
            res_score.cleared_lines.insert(ev.action, ev.cleared_lines);
        }
        res_score.value += score;
    }

    for (mut t, ga) in text.iter_mut() {
        if *ga == GameArea::TextScore {
            t.sections[0].value = format!("{:07}", res_score.value);
        }

        if *ga == GameArea::TextLines {
            t.sections[0].value = format!("{:07}", matrix.lines_cleared);
        }
    }
}
