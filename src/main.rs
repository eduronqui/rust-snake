use bevy::prelude::*;

const BG_GREEN: Color = Color::rgb(0.52, 0.73, 0.17);

fn main() {
    // DefaultPlugins bundles plugins for window, log and much more
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(BG_GREEN))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D camera will be used to spawn sprites later on
    commands.spawn(Camera2dBundle::default());
}
