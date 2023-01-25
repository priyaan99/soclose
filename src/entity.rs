use macroquad::prelude::*;

pub enum Art {
    Rect(Rect),
    Circle(Circle),
    Pointy(Rect),
}

pub struct Entity {
    alive: bool,
    art: Art,
    color: Color,
    borntime: f32,
    velocity: Vec2,
}

impl Entity {
    pub fn new(art: Art, velocity: Vec2, borntime: f32) -> Self {
        Self {
            alive: false,
            art,
            velocity,
            borntime,
            color: RED,
        }
    }

    pub fn is_out_of(&self, rect: Rect) -> bool {
        match self.art {
            Art::Rect(r) => r.y > rect.h,
            Art::Circle(c) => c.y - c.r > rect.h,
            Art::Pointy(_) => todo!(),
        }
    }

    pub fn update(&mut self, timer: f32) {
        if self.borntime <= timer {
            self.alive = true;
        }

        if self.alive {
            match &mut self.art {
                Art::Rect(r) => r.move_to(r.point() + self.velocity * get_frame_time()),
                Art::Circle(c) => c.move_to(c.point() + self.velocity * get_frame_time()),
                Art::Pointy(_) => todo!(),
            }
        }
    }

    pub fn draw(&self) {
        if self.alive {
            match self.art {
                Art::Rect(r) => draw_rectangle(r.x, r.y, r.w, r.h, self.color),
                Art::Circle(c) => draw_circle(c.x, c.y, c.r, self.color),
                Art::Pointy(_) => todo!(),
            }
        }
    }
}
