use ggez::mint::{Point2, Vector2};
use ggez::Context;
use ggez::graphics;
use ggez::GameResult;

use crate::entities::enums::Colour;
use crate::assets::Assets;

pub struct FallingPiece {
    pub current_pos: Point2<f32>,
    pub target_pos: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub target_coords: Point2<usize>,
    pub colour: Colour,
    stopped: bool,
}

impl FallingPiece {
    pub fn new(start: Point2<f32>, end: Point2<f32>, 
        t_coord: Point2<usize>, colour: Colour) -> Self {
        FallingPiece {
            current_pos: start,
            target_pos: end,
            velocity: Vector2 { x: 0.0, y: 800.0 },
            target_coords: t_coord,
            colour,
            stopped: false
        }
    }

    pub fn update(&mut self, seconds: f32) {
        self.current_pos.x += self.velocity.x * seconds;
        self.current_pos.y += self.velocity.y * seconds;
        if self.current_pos.y>=self.target_pos.y {
            self.current_pos = self.target_pos;
            self.velocity = Vector2{x:0.0,y:0.0};
            self.stopped = true;
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        graphics::draw(
            ctx, 
            match self.colour {
                Colour::Green => &assets.green,
                Colour::Red => &assets.red,
            }, 
            graphics::DrawParam::default().dest(self.current_pos))
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }
}