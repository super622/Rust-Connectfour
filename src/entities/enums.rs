#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Colour {
    Green,
    Red,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BoardCell {
    Piece(Colour),
    Empty,
}