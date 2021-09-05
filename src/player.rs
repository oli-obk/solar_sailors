use std::collections::HashMap;

use hex2d::Coordinate;
use macroquad::prelude::{
    animation::{AnimatedSprite, Animation, AnimationFrame},
    *,
};

use crate::{
    datastructures::{Sensor, SetGet},
    save::{ComplexSaveable, Saveable},
    ship::{Attachement, Segment, ATTACHEMENT_ANGLES, ATTACHEMENT_OFFSETS, SIZE, SPACING, SQRT3},
};

pub struct Player {
    pos: Sensor<ComplexSaveable<Coordinate>>,
    side: Sensor<Saveable<u8>>,
    /// Used to reduce the speed of animations.
    speed: u32,
    i: u32,
    x: Saveable<i32>,
    texture: Texture2D,
    anim: AnimatedSprite,
    animations: [Animation; 4],
    action: Action,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Action {
    Idle,
    Walk { right: bool },
    Sleep,
    Wake,
    Use { up: bool },
}

impl Player {
    pub fn new(coord: impl Into<Coordinate>, side: u8) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/Crab Sprite Sheet.png"),
            Some(ImageFormat::Png),
        );
        // Pixel art
        texture.set_filter(FilterMode::Nearest);

        let animations = [
            Animation {
                name: "idle".into(),
                row: 0,
                frames: 2,
                fps: 4,
            },
            Animation {
                name: "move".into(),
                row: 1,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "sleep".into(),
                row: 2,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "grab".into(),
                row: 3,
                frames: 4,
                fps: 4,
            },
        ];
        let anim = AnimatedSprite::new(32, 32, &animations, false);

        Self {
            pos: Sensor::raw(Saveable::new(coord.into(), "player")),
            side: Sensor::raw(Saveable::new(side, "player/side")),
            texture,
            speed: 0,
            i: 0,
            x: Saveable::default("player/side_pos"),
            anim,
            animations,
            action: Action::Idle,
        }
    }

    fn attachement<'a>(
        &self,
        grid: &'a HashMap<hex2d::Coordinate, Segment>,
    ) -> Option<&'a (dyn Attachement + 'static)> {
        grid.get(&self.pos.get())
            .expect("crab not on grid")
            .attachements[self.side.get() as usize]
            .as_ref()
            .map(|a| &**a)
    }

    fn attachement_mut<'a>(
        &self,
        grid: &'a mut HashMap<hex2d::Coordinate, Segment>,
    ) -> Option<&'a mut (dyn Attachement + 'static)> {
        grid.get_mut(&self.pos.get())
            .expect("crab not on grid")
            .attachements[self.side.get() as usize]
            .as_mut()
            .map(|a| &mut **a)
    }

    pub fn update(&mut self, grid: &mut HashMap<hex2d::Coordinate, Segment>) {
        let attachement = self.attachement(grid).is_some();
        let next_action = match (is_key_down(KeyCode::S), is_key_down(KeyCode::W)) {
            (true, false) if attachement => Some(Action::Use { up: false }),
            (false, true) if attachement => Some(Action::Use { up: true }),
            _ => match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
                (true, true) => Some(Action::Sleep),
                (false, true) => Some(Action::Walk { right: true }),
                (true, false) => Some(Action::Walk { right: false }),
                (false, false) => None,
            },
        };
        match self.action {
            // Wake up whenever in the final sleeping frames
            Action::Sleep if self.i > 1 => match next_action {
                None | Some(Action::Sleep) => {}
                Some(_) => {
                    self.action = Action::Wake;
                    self.i = 0;
                    self.speed = 0;
                }
            },
            // The idle and walk action can immediately be overwritten
            Action::Walk { .. } | Action::Idle | Action::Use { .. } => {
                let next_action = next_action.unwrap_or(Action::Idle);
                if next_action == self.action {
                    let dir = match next_action {
                        Action::Use { up } => Some(up),
                        _ => None,
                    };

                    if let Some(attachement) = self.attachement_mut(grid) {
                        attachement.control(dir, Some(self.x()));
                    }
                } else {
                    self.action = next_action;
                    self.i = 0;
                    self.speed = 0;
                }
            }
            _ => {}
        }

        self.speed += 1;
        // Only step the animation every few frames.
        let speed_limit = match self.action {
            Action::Walk { .. } => 3,
            Action::Sleep => 30,
            Action::Wake => 10,
            Action::Idle => 20,
            Action::Use { .. } => 8,
        };
        if self.speed == speed_limit {
            self.speed = 0;
            self.i += 1;
            if let Action::Walk { right } = self.action {
                if right {
                    self.x += 2;
                } else {
                    self.x -= 2;
                }
                if self.x.abs() > (SIZE / SQRT3) as i32 / SCALE / 2 {
                    // Notify attachement that we left
                    if let Some(attachement) = self.attachement_mut(grid) {
                        attachement.control(None, None);
                    }
                    if self.x > 0 {
                        self.side += 1;
                    } else {
                        self.side += 5;
                    }
                    self.side %= 6;
                    self.x *= -1;
                    // Prevent oscillating on a corner
                    if right {
                        self.x += 2;
                    } else {
                        self.x -= 2;
                    }
                    let coord = self.pos.get().neighbors()[self.side.get() as usize];
                    // Walk onto next hex if there is one
                    if grid.contains_key(&coord) {
                        self.pos.set(coord.into());
                        self.side += 3;
                        if self.x > 0 {
                            self.side -= 1;
                        } else {
                            self.side += 1;
                        }
                        self.side %= 6;
                        // Notify attachement that we arrived
                        if let Some(attachement) = self.attachement_mut(grid) {
                            attachement.control(None, Some(self.x()));
                        }
                    }
                }
            }
            let action_id = match self.action {
                Action::Idle => 0,
                Action::Walk { .. } => 1,
                Action::Wake | Action::Sleep => 2,
                Action::Use { .. } => 3,
            };
            if self.i == self.animations[action_id].frames {
                match self.action {
                    Action::Sleep => self.i -= 2,
                    Action::Use { .. } | Action::Wake | Action::Walk { .. } | Action::Idle => {
                        self.action = next_action.unwrap_or(Action::Idle);
                        self.i = 0;
                    }
                }
            }
            self.anim.set_animation(action_id);
        }

        self.anim.set_frame(match self.action {
            Action::Idle | Action::Walk { .. } | Action::Sleep => self.i,
            Action::Wake => 3 - self.i,
            Action::Use { up: true } => self.i,
            Action::Use { up: false } => 3 - self.i,
        });
    }

    pub(crate) fn draw(&self) {
        let (x, y) = self.pos.get().to_pixel(SPACING);
        let center = vec2(x, y);

        let AnimationFrame {
            source_rect,
            mut dest_size,
        } = self.anim.frame();

        dest_size *= SCALE as f32;

        let base = ATTACHEMENT_OFFSETS[self.side.get() as usize];
        let x = self.x() - dest_size.x / 2.0;
        let offset = x * base.perp().normalize();
        const ANIM_OFFSET: i32 = 32 * SCALE;
        // Lower the animation onto the object by shifting away the empty
        // pixels below it.
        const BASE_SCALE: f32 = (SIZE / 2.0 + ANIM_OFFSET as f32) / (SIZE / 2.0);
        let pos = center + base * BASE_SCALE + offset;

        let flip_x = match self.action {
            Action::Idle => false,
            Action::Walk { right } => right,
            Action::Wake | Action::Sleep => false,
            Action::Use { .. } => false,
        };

        draw_texture_ex(
            self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                source: Some(source_rect),
                rotation: ATTACHEMENT_ANGLES[self.side.get() as usize],
                pivot: Some(pos),
                flip_x,
                ..Default::default()
            },
        );
    }

    fn x(&self) -> f32 {
        (*self.x * SCALE) as f32
    }
}

const SCALE: i32 = (SIZE / 100.0) as _;
