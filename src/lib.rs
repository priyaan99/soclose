pub mod entity;
pub mod game;
pub mod map;
pub mod player;

pub mod colors {
    use macroquad::{color::Color, color_u8};

    pub const PLAYER: Color = color_u8!(250, 213, 102, 255);
    pub const BACKGROUD: Color = color_u8!(38, 38, 38, 255);
    pub const SIDE_WALL: Color = color_u8!(0x32, 0x32, 0x32, 255);

    // 7c7c7c            RED,
}
