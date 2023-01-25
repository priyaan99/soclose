use macroquad::prelude::*;

const MAX_SPEED: Vec2 = vec2(200., 0.);
const LERP_SPEED: f32 = 0.1;

pub struct Player {
    field_rect: Rect,
    position: Vec2,
    radius: f32,
    tension_radius: f32,
    velocity: Vec2,
}

impl Player {
    pub fn init(rect: Rect) -> Self {
        let radius = 10.;
        let position = vec2(rect.w / 2., rect.h - radius);
        let velocity = vec2(0., 0.);
        let tension_radius = radius + 5.;

        Self {
            field_rect: rect,
            position,
            radius,
            tension_radius,
            velocity,
        }
    }

    fn player_lerp(&mut self, dir: f32) {
        self.velocity = self.velocity.lerp(dir * MAX_SPEED, LERP_SPEED);
    }

    fn player_clamp(&mut self) {
        self.position = self.position.clamp(
            vec2(
                self.field_rect.x + self.radius,
                self.field_rect.y + self.radius,
            ),
            vec2(
                self.field_rect.w - self.radius,
                self.field_rect.h - self.radius,
            ),
        )
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.player_lerp(-1.);
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.player_lerp(1.);
        } else {
            self.player_lerp(0.);
        }

        self.position += self.velocity * get_frame_time();
        self.player_clamp();
    }

    pub fn draw(&self) {
        draw_circle(
            self.position.x,
            self.position.y,
            self.radius,
            crate::colors::PLAYER,
            // RED,
        );
    }
}
