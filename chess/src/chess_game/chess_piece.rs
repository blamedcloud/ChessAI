use crate::chess_game::Player;

pub struct ChessPiece {
    owner: Player,
    name: PieceName,
    _moved: bool,
}

impl ChessPiece {
    pub fn new(owner: Player, name: PieceName, moved: bool) -> ChessPiece {
        Self { owner, name, _moved: moved }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
