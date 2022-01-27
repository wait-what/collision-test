use macroquad::prelude::*;

pub struct Player {
    velocity: Vec2,
    friction: f32,
    acceleration: f32,
    pub position: Rect,
}

pub struct State {
    pub player: Player,
    pub objects: Vec<Rect>,
}

impl State {
    pub fn new() -> Self {
        Self {
            player: Player {
                velocity: Vec2::new(0., 0.),
                friction: 16.,
                acceleration: 700.,
                position: Rect::new(10., 10., 50., 50.),
            },
            objects: vec![
                Rect::new(300., 300., 100., 100.),
                Rect::new(500., 120., 100., 20.),
                Rect::new(140., 120., 10., 150.),
            ],
        }
    }

    pub fn update(&mut self, delta: f32) {
        let position = &mut self.player.position;
        let velocity = &mut self.player.velocity;

        // Controls
        let controls = Self::controls();

        if controls.x != 0. {
            velocity.x = controls.x * self.player.acceleration;
        }
        if controls.y != 0. {
            velocity.y = controls.y * self.player.acceleration;
        }

        // Friction
        velocity.x *= 1. - self.player.friction * delta;
        velocity.y *= 1. - self.player.friction * delta;

        // Diagonal movement
        if velocity.x != 0. && velocity.y != 0. {
            // 1 / sqrt(2)
            velocity.x *= 0.707;
            velocity.y *= 0.707;
        }

        // Collision
        let mut future_position = position.clone();
        future_position.x += velocity.x * delta;
        future_position.y += velocity.y * delta;

        for object in &self.objects {
            let intersection = future_position.intersect(*object);

            if intersection.is_some() {
                let intersection = intersection.unwrap();

                let w = (future_position.w + object.w) / 2.;
                let h = (future_position.h + object.h) / 2.;

                let center_x = (future_position.x + future_position.w / 2.) - (object.x + object.h / 2.);
                let center_y = (future_position.y + future_position.h / 2.) - (object.y + object.h / 2.);

                let wy = w * center_y;
                let hx = h * center_x;

                if wy > hx {
                    if wy > -hx {
                        position.y += intersection.h;
                    } else {
                        position.x -= intersection.w;
                    }
                } else {
                    if wy > -hx {
                        position.x += intersection.w;
                    } else {
                        position.y -= intersection.h;
                    }
                }
            }
        }

        // Movement
        position.x += velocity.x * delta;
        position.y += velocity.y * delta;

        // World bounds
        if future_position.x < 0. {
            position.x = 0.;
        }
        if future_position.y < 0. {
            position.y = 0.;
        }
        if future_position.x + position.w > screen_width() {
            position.x = screen_width() - position.w;
        }
        if future_position.y + position.h > screen_height() {
            position.y = screen_height() - position.h;
        }
    }

    fn controls() -> Vec2 {
        let mut x = 0.;
        let mut y = 0.;

        if is_key_down(KeyCode::A) {
            x -= 1.;
        }
        if is_key_down(KeyCode::D) {
            x += 1.;
        }
        if is_key_down(KeyCode::W) {
            y -= 1.;
        }
        if is_key_down(KeyCode::S) {
            y += 1.;
        }

        Vec2::new(x, y)
    }
}
