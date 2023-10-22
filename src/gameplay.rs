use crate::{board::Position, snake::Snake};
use bevy::prelude::*;

pub fn tick(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<(Entity, &Position)>,
) {
    dbg!("tick!");

    let mut next_position = snake.segments[0].clone();
    next_position.x += 1;

    snake.segments.push_front(next_position);
    let old_tail = snake.segments.pop_back().unwrap();

    if let Some((entity, _)) =
        positions.iter().find(|(_, pos)| pos == &&old_tail)
    {
        commands.entity(entity).despawn_recursive();
    }

    dbg!(snake);
}