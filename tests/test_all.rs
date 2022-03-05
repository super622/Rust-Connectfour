use ggez::mint::Point2;

use connectfour::entities::falling_piece::FallingPiece;
use connectfour::entities::board::Board;
use connectfour::entities::enums::{BoardCell,Colour};

#[test]
fn test_falling_piece() {
    let mut piece = FallingPiece::new(
        Point2{x:0.0,y:0.0}, 
        Point2{x:0.0,y:100.0}, 
        Point2{x:0,y:0}, 
        Colour::Red,
    );

    assert!(!piece.is_stopped());

    piece.update(0.1);
    assert_eq!(Point2{x:0.0,y:piece.velocity.y*0.1},piece.current_pos);
    assert!(!piece.is_stopped());

    piece.update(0.01);
    assert_eq!(Point2{x:0.0,y:piece.velocity.y*0.11},piece.current_pos);
    assert!(!piece.is_stopped());

    piece.update(1.0);
    assert_eq!(piece.target_pos,piece.current_pos);
    assert!(piece.is_stopped());
}

#[test]
fn test_board() {
    let mut board = Board::new(Point2{x:0.0,y:0.0});
    assert_eq!(board.result,None);

    board.board[5][3] = BoardCell::Piece(Colour::Green);
    board.board[5][4] = BoardCell::Piece(Colour::Green);
    board.board[5][5] = BoardCell::Piece(Colour::Green);
    board.board[5][6] = BoardCell::Piece(Colour::Green);
    board.decide_result(Point2{x:3,y:5}, Colour::Green);
    assert_eq!(board.result.unwrap(),BoardCell::Piece(Colour::Green));
    
    board.clear();
    board.board[2][6] = BoardCell::Piece(Colour::Red);
    board.board[3][6] = BoardCell::Piece(Colour::Red);
    board.board[4][6] = BoardCell::Piece(Colour::Red);
    board.board[5][6] = BoardCell::Piece(Colour::Red);
    board.decide_result(Point2{x:6,y:2}, Colour::Red);
    assert_eq!(board.result.unwrap(),BoardCell::Piece(Colour::Red));
    
    board.clear();
    board.board[2][3] = BoardCell::Piece(Colour::Red);
    board.board[3][4] = BoardCell::Piece(Colour::Red);
    board.board[4][5] = BoardCell::Piece(Colour::Red);
    board.board[5][6] = BoardCell::Piece(Colour::Red);
    board.decide_result(Point2{x:3,y:2}, Colour::Red);
    assert_eq!(board.result.unwrap(),BoardCell::Piece(Colour::Red));
    
    board.clear();
    board.board[3][3] = BoardCell::Piece(Colour::Green);
    board.board[2][4] = BoardCell::Piece(Colour::Green);
    board.board[1][5] = BoardCell::Piece(Colour::Green);
    board.board[0][6] = BoardCell::Piece(Colour::Green);
    board.decide_result(Point2{x:6,y:0}, Colour::Green);
    assert_eq!(board.result.unwrap(),BoardCell::Piece(Colour::Green));
    
    board.clear();
    for j in 0..3 {
        for i in 0..2 {
            board.board[i*3][j*2] = BoardCell::Piece(Colour::Green);
            board.board[i*3][j*2+1] = BoardCell::Piece(Colour::Red);
        }
        board.board[2][j*2] = BoardCell::Piece(Colour::Red);
        board.board[3][j*2] = BoardCell::Piece(Colour::Red);
        board.board[2][j*2+1] = BoardCell::Piece(Colour::Green);
        board.board[3][j*2+1] = BoardCell::Piece(Colour::Green);
        board.board[j*2][6] = BoardCell::Piece(Colour::Red);
        board.board[j*2+1][6] = BoardCell::Piece(Colour::Green);
    }
    
    for i in 0..2 {
        for j in 0..3 {
            board.decide_result(Point2{x:j,y:i}, Colour::Green);
            board.decide_result(Point2{x:j,y:i}, Colour::Red);
        }
    }
    assert!(board.result.is_none());

    board.piece_count = 42;
    board.decide_result(Point2{x:0,y:0}, Colour::Green);
    assert_eq!(board.result.unwrap(),BoardCell::Empty);
}