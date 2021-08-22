use hex2d::Coordinate;
use macroquad::prelude::{
    animation::{AnimatedSprite, Animation, AnimationFrame},
    *,
};

use crate::ship::{ATTACHEMENT_ANGLES, ATTACHEMENT_OFFSETS, SIZE, SPACING};

pub struct Player {
    pos: Coordinate,
    side: u8,
    /// Used to reduce the speed of animations.
    speed: u32,
    i: u32,
    x: f32,
    texture: Texture2D,
    anim: AnimatedSprite,
    animations: [Animation; 7],
    action: Action,
    next_action: Action,
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Dance = 0,
    WalkLeft = 1,
    WalkRight = 2,
    Idle = 3,
    Attack = 4,
    Sit = 5,
    Jump = 6,
}

impl Player {
    pub fn new(coord: impl Into<Coordinate>, side: u8) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/redCrab.png"),
            Some(ImageFormat::Png),
        );
        // Pixel art
        texture.set_filter(FilterMode::Nearest);

        let animations = [
            Animation {
                name: "dance".into(),
                row: 0,
                frames: 7,
                fps: 7,
            },
            Animation {
                name: "walk left".into(),
                row: 1,
                frames: 4,
                fps: 7,
            },
            Animation {
                name: "walk right".into(),
                row: 2,
                frames: 4,
                fps: 7,
            },
            Animation {
                name: "idle".into(),
                row: 3,
                frames: 6,
                fps: 7,
            },
            Animation {
                name: "attack".into(),
                row: 4,
                frames: 7,
                fps: 7,
            },
            Animation {
                name: "sit".into(),
                row: 5,
                frames: 4,
                fps: 7,
            },
            Animation {
                name: "jump".into(),
                row: 4,
                frames: 3,
                fps: 7,
            },
        ];
        let anim = AnimatedSprite::new(16, 16, &animations, false);

        Self {
            pos: coord.into(),
            side,
            texture,
            speed: 0,
            i: 0,
            x: 0.0,
            anim,
            animations,
            action: Action::Idle,
            next_action: Action::Idle,
        }
    }

    pub fn update(&mut self) {
        match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, true) => self.next_action = Action::Dance,
            (false, true) => self.next_action = Action::WalkRight,
            (true, false) => self.next_action = Action::WalkLeft,
            (false, false) => match self.next_action {
                Action::Dance | Action::WalkLeft | Action::WalkRight => {
                    self.next_action = Action::Idle
                }
                Action::Idle | Action::Attack | Action::Sit | Action::Jump => {}
            },
        }

        self.speed += 1;
        // Only step the animation every 8 frames.
        if self.speed == 8 {
            self.speed = 0;
            self.i += 1;
            let (i, x) = match self.action {
                Action::Idle => (6, 0.0),
                Action::Dance => (7, 0.0),
                Action::WalkLeft => (3, -1.0),
                Action::WalkRight => (3, 1.0),
                Action::Attack => todo!(),
                Action::Sit => todo!(),
                Action::Jump => todo!(),
            };
            if self.i == i {
                self.x += x;
            }
            if self.i == self.animations[self.action as usize].frames {
                self.action = match self.action {
                    Action::Dance | Action::WalkLeft | Action::WalkRight | Action::Idle => {
                        self.next_action
                    }
                    Action::Attack => todo!(),
                    Action::Sit => todo!(),
                    Action::Jump => todo!(),
                };
                self.i = 0;
            }
        }
        self.anim.set_animation(self.action as _);
        self.anim.set_frame(self.i);
    }

    pub(crate) fn draw(&self) {
        let AnimationFrame {
            source_rect,
            dest_size,
        } = self.anim.frame();

        let base = ATTACHEMENT_OFFSETS[self.side as usize];
        let x = self.x + dest_size.x / 2.0;
        let offset = x * base.perp().normalize();
        let (x, y) = self.pos.to_pixel(SPACING);
        const ANIM_OFFSET: f32 = 3.0;
        // Lower the animation onto the object by shifting away the empty
        // pixels below it.
        const BASE_SCALE: f32 = (SIZE / 2.0 - ANIM_OFFSET) / (SIZE / 2.0);
        let pos = vec2(x, y) + base * BASE_SCALE + offset;

        draw_texture_ex(
            self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                source: Some(source_rect),
                rotation: ATTACHEMENT_ANGLES[self.side as usize],
                ..Default::default()
            },
        );
    }
}
