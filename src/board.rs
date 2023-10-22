use bevy::prelude::*;
use itertools::Itertools;

use crate::{colors::COLORS, snake::Snake};

const BOARD_SIZE: u8 = 20;
const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;

const BOARD_LAYER_LEVEL: f32 = 1.0;
const SNAKE_LAYER_LEVEL: f32 = 2.0;

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

#[derive(Debug, Component, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }
}

pub fn spawn_board(mut commands: Commands, snake: Res<Snake>) {
    let board = Board::new(BOARD_SIZE);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLORS.board,
                custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for (x, y) in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            COLORS.tile_placeholder
                        } else {
                            COLORS.tile_placeholder_dark
                        },
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(x),
                        board.cell_position_to_physical(y),
                        BOARD_LAYER_LEVEL,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);

    let board = Board::new(BOARD_SIZE);

    for segment in snake.segments.iter() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLORS.snake,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(segment.x),
                    board.cell_position_to_physical(segment.y),
                    SNAKE_LAYER_LEVEL,
                ),
                ..default()
            })
            .insert(segment.clone());
    }
}
