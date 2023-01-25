use macroquad::prelude::*;

use crate::colors::*;
use crate::{entity::Entity, map::Level, player::Player};

const OFFSET: f32 = 15.;

pub struct PlayField {
    rect: Rect,
    player: Player,
    spawners: Vec<Entity>,
    timer: f32,
}

impl PlayField {
    pub fn init() -> Self {
        let rect = Rect::new(OFFSET, 0., screen_width() - OFFSET, screen_height());
        let player = Player::init(rect);
        let spawners = Level::easy();

        Self {
            rect,
            player,
            spawners,
            timer: 0.,
        }
    }

    pub fn update(&mut self) {
        self.timer += get_frame_time();

        self.player.update();
        self.update_entities();

        if self.spawners.is_empty() {
            self.spawners = Level::easy();
            self.timer = 0.;
        }
    }

    pub fn draw(&self) {
        self.player.draw();
        self.draw_spawners();
        self.draw_side_wall();
    }

    fn draw_side_wall(&self) {
        draw_rectangle(0., 0., OFFSET, screen_height(), SIDE_WALL);
        draw_rectangle(
            screen_width() - OFFSET,
            0.,
            OFFSET,
            screen_height(),
            SIDE_WALL,
        );
    }
}

// for spawners
impl PlayField {
    fn update_entities(&mut self) {
        // store out of play field entites index
        let mut dead_entities = vec![];
        for (i, e) in self.spawners.iter_mut().enumerate() {
            if e.is_out_of(self.rect) {
                dead_entities.push(i);
            }
            e.update(self.timer);
        }

        // removes out of playfield enities
        for i in dead_entities {
            self.spawners.swap_remove(i);
        }
    }

    fn draw_spawners(&self) {
        for s in &self.spawners {
            s.draw();
        }
    }
}
