use bevy::{ecs::system::Command, prelude::*};
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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct SpawnSnakeSegments {
    pub position: Position,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size =
            f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
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
                custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size,
                )),
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

    for segment in snake.segments.iter() {
        commands.add(SpawnSnakeSegments { position: *segment });
    }
}

impl Command for SpawnSnakeSegments {
    fn apply(self, world: &mut World) {
        let board = world.query::<&Board>().iter(world).next().unwrap();

        world
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLORS.snake,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(self.position.x),
                    board.cell_position_to_physical(self.position.y),
                    SNAKE_LAYER_LEVEL,
                ),
                ..default()
            })
            .insert(self.position);
    }
}
