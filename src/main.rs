use std::time::Duration;

use bevy::prelude::*;
use rust_snake::{
    board::spawn_board, colors::COLORS, gameplay::tick, snake::Snake,
};

fn main() {
    // DefaultPlugins bundles plugins for window, log and much more
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                ..default()
            }),
            ..default()
        }))
        // Controlling the time interval for fixed updates (tick)
        .insert_resource(FixedTime::new(Duration::from_millis(1000)))
        .insert_resource(ClearColor(COLORS.background))
        .init_resource::<Snake>()
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        // events that run on a fixed interval
        .add_systems(FixedUpdate, tick)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D camera will be used to spawn sprites later on
    commands.spawn(Camera2dBundle::default());
}
