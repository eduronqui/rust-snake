use bevy::prelude::*;
use rust_snake::{board::spawn_board, colors::COLORS};

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
        .insert_resource(ClearColor(COLORS.background))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D camera will be used to spawn sprites later on
    commands.spawn(Camera2dBundle::default());
}
