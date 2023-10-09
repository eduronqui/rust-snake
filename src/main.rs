use bevy::prelude::{App, Update};

fn main() {
    App::new()
        .add_systems(Update, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, bevy!");
}
