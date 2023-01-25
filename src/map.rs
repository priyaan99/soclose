use crate::entity::{Art, Entity};
use macroquad::prelude::*;

pub struct Level {}

impl Level {
    pub fn easy() -> Vec<Entity> {
        let mut entities = vec![];

        entities.push(Entity::new(
            Art::Rect(Rect::new(screen_width() / 2., -20., 20., 20.)),
            vec2(0., 120.),
            1.2,
        ));

        entities.push(Entity::new(
            Art::Rect(Rect::new(0., -20., 20., 20.)),
            vec2(0., 50.),
            2.3,
        ));

        entities.push(Entity::new(
            Art::Rect(Rect::new(screen_width() / 2., -20., 20., 20.)),
            vec2(0., 100.),
            0.5,
        ));

        entities.push(Entity::new(
            Art::Rect(Rect::new(screen_width() / 2., -20., 20., 20.)),
            vec2(0., 100.),
            3.2,
        ));

        entities.push(Entity::new(
            Art::Circle(Circle::new(30., -24., 12.)),
            vec2(0., 200.),
            2.8,
        ));

        entities
    }
}
