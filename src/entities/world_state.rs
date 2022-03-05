use ggez::mint::Point2;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::input::{self, mouse};
use ggez::timer;
use ggez::event::{self, EventHandler};
use ggez::audio::SoundSource;

use crate::assets::Assets;
use crate::entities::enums::{BoardCell,Colour};
use crate::entities::board::Board;
use crate::entities::falling_piece::FallingPiece;

pub struct WorldState {
    assets: Assets,
    game_board: Board,
    current_player: Colour,
    first_player: Colour,
    falling_piece: Option<FallingPiece>,
    first_name: String,
    second_name: String,
}

impl WorldState {    
    pub fn new(ctx: &mut Context, board_offset: Point2<f32>, colour: Colour, 
        first_name: String, second_name: String) -> GameResult<WorldState> {
        Ok(
            WorldState {
                game_board: Board::new(board_offset),
                assets:Assets::new(ctx)?,
                current_player:colour,
                first_player: colour,
                falling_piece: None,
                first_name,
                second_name,
            }
        )
    }

    pub fn reset(&mut self) {
        self.game_board.clear();
        self.current_player = self.first_player;
        self.falling_piece = None;
    }
}

impl EventHandler for WorldState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_board.result.is_some() {
            return Ok(());
        }
        const TARGET_FPS: u32 = 60;

        while timer::check_update_time(ctx, TARGET_FPS) {
            let seconds = 1.0 / (TARGET_FPS as f32);

            if mouse::button_pressed(ctx, mouse::MouseButton::Left) 
                && self.falling_piece.is_none() {
                let mouse_position = mouse::position(ctx);
                let cell = self.game_board.find_position(mouse_position);
                //println!("{:?}",cell);
                if let Some(coords) = cell {
                    self.falling_piece = Some(FallingPiece::new(
                        Point2 {
                            x:self.game_board.board_offset.x+40.0+(coords.x as f32)*96.0,
                            y:self.game_board.board_offset.y-96.0},
                        Point2 {
                            x:self.game_board.board_offset.x+40.0+(coords.x as f32)*96.0,
                            y:self.game_board.board_offset.y+40.0+(coords.y as f32)*96.0}, 
                        coords,
                            self.current_player,
                        )
                    )
                }
            }

            if let Some(piece) = self.falling_piece.as_mut() {
                piece.update(seconds);
            }

            if let Some(_) = self.game_board.update(&self.falling_piece) {
                self.falling_piece = None;
                let _ = self.assets.fall_sound.play(ctx);
                match self.current_player {
                    Colour::Green => self.current_player = Colour::Red,
                    Colour::Red => self.current_player = Colour::Green
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = ggez::graphics::Color::from_rgb(80, 80, 80);
        graphics::clear(ctx, background_color);

        self.game_board.draw_back(ctx, &self.assets)?;

        if let Some(_) = self.falling_piece {
            self.falling_piece.as_ref().unwrap().draw(ctx,&self.assets)?;
        }
        else if self.game_board.result.is_none() {
            let mouse_position = mouse::position(ctx);
            let cell = self.game_board.find_position(mouse_position);
            if let Some(coords) = cell {
                let end = Point2 {
                    x:self.game_board.board_offset.x+40.0+(coords.x as f32)*96.0,
                    y:self.game_board.board_offset.y+40.0+(coords.y as f32)*96.0
                };
                graphics::draw(
                    ctx,
                    match self.current_player {
                        Colour::Green => &self.assets.green_alternative,
                        Colour::Red => &self.assets.red_alternative,
                    },
                    graphics::DrawParam::default().dest(end))?;
            }
        }

        self.game_board.draw_front(ctx, &self.assets)?;

        if self.game_board.result.is_some() {
            let winner: String = match self.game_board.result.unwrap() {
                BoardCell::Empty => "Draw".to_string(),
                BoardCell::Piece(c) => 
                    if c == self.first_player {self.first_name.clone() + " won!"}
                    else {self.second_name.clone() + " won!"}
            };
            let mut text = graphics::Text::new(winner);
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
            text.set_font(font, graphics::PxScale::from(60.0));

            graphics::draw(ctx, &text, graphics::DrawParam::default()
                .dest(Point2 {
                    x:self.game_board.board_offset.x,
                    y:self.game_board.board_offset.y + 688.0 - 60.0
                }
            ))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode,
        _keymod: input::keyboard::KeyMods, _repeat: bool) {
            match keycode {
            event::KeyCode::R => { self.reset(); },
            _ => (), 
        }
    }
}