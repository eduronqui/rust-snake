use bevy::prelude::Color;

pub struct Colors {
    pub background: Color,
    pub board: Color,
    pub tile_placeholder: Color,
    pub tile_placeholder_dark: Color,
    pub snake: Color,
}

pub const COLORS: Colors = Colors {
    background: Color::rgb(0.52, 0.73, 0.17),
    board: Color::rgb(0.42, 0.63, 0.07),
    tile_placeholder: Color::rgb(0.62, 0.83, 0.27),
    tile_placeholder_dark: Color::rgb(0.57, 0.78, 0.22),
    snake: Color::WHITE,
};
