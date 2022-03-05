use ggez::mint::Point2;
use ggez::Context;
use ggez::graphics;
use ggez::GameResult;

use std::cmp::min;

use crate::assets::Assets;
use crate::entities::enums::{BoardCell,Colour};
use crate::entities::falling_piece::FallingPiece;

pub struct Board{
    pub board: [[BoardCell; 7]; 6],
    pub board_offset: Point2<f32>,
    pub piece_count: usize,
    pub result: Option<BoardCell>,
}

impl Board {
    pub fn new(board_offset: Point2<f32>) -> Board {
        let board = [[BoardCell::Empty; 7]; 6];
        Board {
            board, 
            board_offset, 
            piece_count: 0, 
            result: None
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.board[i][j] = BoardCell::Empty;
            }
        }
        self.piece_count = 0;
        self.result = None;
    }

    pub fn update(&mut self, option_piece: &Option<FallingPiece>) -> Option<()> {
        if option_piece.is_none(){
            return None
        }
        let piece = option_piece.as_ref().unwrap();
        if !piece.is_stopped() {
            return None;
        }
        self.board[piece.target_coords.y][piece.target_coords.x] 
            = BoardCell::Piece(piece.colour);
        self.piece_count += 1;
        self.decide_result(piece.target_coords, piece.colour);
        return Some(());
    }

    pub fn draw_back(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let draw_params = graphics::DrawParam::default()
            .dest(self.board_offset);
        graphics::draw(ctx, &assets.back, draw_params)?;
        Ok(())
    }

    pub fn draw_front(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                let piece_params = graphics::DrawParam::default()
                .dest(Point2 {
                    x:self.board_offset.x+40.0+(j as f32)*96.0,
                    y:self.board_offset.y+40.0+(i as f32)*96.0
                });
                match self.board[i][j] {
                    BoardCell::Piece(Colour::Green)
                        => graphics::draw(ctx, &assets.green, piece_params)?,
                    BoardCell::Piece(Colour::Red)
                        => graphics::draw(ctx, &assets.red, piece_params)?,
                    _ => (),
                }
            }
        }
        let draw_params = graphics::DrawParam::default()
            .dest(self.board_offset);
        graphics::draw(ctx, &assets.front, draw_params)?;

        Ok(())
    }

    pub fn find_position(&self, mouse: Point2<f32>) -> Option<Point2<usize>> {
        if mouse.x < self.board_offset.x + 32.0 
            || mouse.x > self.board_offset.x - 32.0 + 736.0 
            || mouse.y < self.board_offset.y 
            || mouse.y > self.board_offset.y + 688.0 {
                return None;
            }

        let column: usize = min(((mouse.x - self.board_offset.x) as usize - 32)/96,6);
        let mut row = 0;
        while row < self.board.len() && self.board[row][column] == BoardCell::Empty {
            row +=1;
        }
        if row>0 {
            return Some(Point2{x:column,y:row-1});
        }
        None
    }

    pub fn decide_result(&mut self, coords: Point2<usize>, colour: Colour) {
        let row = coords.y;
        let col = coords.x;

        //check row
        let mut count = 0; let mut max_count = 0;
        for i in 0..self.board[row].len() {
            if self.board[row][i] == BoardCell::Piece(colour) {count += 1;}
            if self.board[row][i] != BoardCell::Piece(colour) 
                || i == self.board[row].len()-1 {
                if count >= max_count {max_count = count; count = 0;}}
        }
        if max_count >= 4 {self.result = Some(BoardCell::Piece(colour)); return ();}

        //check column
        count = 0; max_count = 0;
        for i in 0..self.board.len() {
            if self.board[i][col] == BoardCell::Piece(colour) {count += 1;}
            if self.board[i][col] != BoardCell::Piece(colour)
                || i == self.board.len()-1 {
                if count >= max_count {max_count = count; count = 0;}}
        }
        if max_count >= 4 {self.result = Some(BoardCell::Piece(colour)); return ();}

        //check main diagonal
        count = 0; max_count = 0;
        let mut i = row - min(row,col);
        let mut j = col - min(row,col);
        while i < self.board.len() && j < self.board[row].len() {
            if self.board[i][j] == BoardCell::Piece(colour) {count += 1;}
            if self.board[i][j] != BoardCell::Piece(colour)
                || i == self.board.len() - 1 || j == self.board[row].len() - 1 {
                if count >= max_count {max_count = count; count = 0;}}
            i+=1; j+=1;
        }
        if max_count >= 4 {self.result = Some(BoardCell::Piece(colour)); return ();}
        
        //check secondary diagonal
        count = 0; max_count = 0;
        j = min(row+col,self.board[row].len()-1);
        let mut j = j as isize;
        i = row+col-(j as usize);
        while i < self.board.len() && j >= 0 {
            if self.board[i as usize][j as usize] == BoardCell::Piece(colour) {count += 1;}
            if self.board[i as usize][j as usize] != BoardCell::Piece(colour)
            || i == self.board.len() - 1 || j == 0 {
            if count >= max_count {max_count = count; count = 0;}}
            i+=1; j-=1;
        }
        if max_count >= 4 {self.result = Some(BoardCell::Piece(colour)); return ();}

        if self.piece_count == self.board.len()*self.board[row].len() {
            self.result = Some(BoardCell::Empty);
        }
    }
}