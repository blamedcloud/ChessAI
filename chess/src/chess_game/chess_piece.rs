use std::fmt::{Display, Formatter};
use crate::chess_game::chess_square::SquareOffset;
use crate::chess_game::Player;


// doesn't auto-derive PartialEq or Eq because we don't care about _moved
#[derive(Debug, Copy, Clone)]
pub struct ChessPiece {
    owner: Player,
    name: PieceName,
    _moved: bool,
}

impl ChessPiece {
    pub fn new(owner: Player, name: PieceName, moved: bool) -> ChessPiece {
        Self { owner, name, _moved: moved }
    }

    pub fn get_owner(&self) -> Player {
        self.owner
    }

    pub fn get_name(&self) -> PieceName {
        self.name
    }

    pub fn has_moved(&self) -> bool {
        self._moved
    }

    pub fn not_moved(&self) -> bool {
        !self._moved
    }

    pub fn set_moved(&mut self, moved: bool) {
        self._moved = moved;
    }

    pub fn to_string(&self) -> String {
        match self.owner {
            Player::White => match self.name {
                PieceName::Pawn => "P",
                PieceName::Knight => "N",
                PieceName::Bishop => "B",
                PieceName::Rook => "R",
                PieceName::Queen => "Q",
                PieceName::King => "K",
            },
            Player::Black => match self.name {
                PieceName::Pawn => "p",
                PieceName::Knight => "n",
                PieceName::Bishop => "b",
                PieceName::Rook => "r",
                PieceName::Queen => "q",
                PieceName::King => "k",
            }
        }.to_string()
    }
}

impl PartialEq for ChessPiece {
    fn eq(&self, other: &ChessPiece) -> bool {
        // do not compare _moved
        self.owner == other.owner && self.name == other.name
    }
}

impl Eq for ChessPiece {}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
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

impl PieceName {
    pub fn knight_offsets() -> [SquareOffset; 8] {
        [SquareOffset(-2,-1), SquareOffset(-2,1), SquareOffset(-1,-2), SquareOffset(-1,2), SquareOffset(1,-2), SquareOffset(1, 2), SquareOffset(2,-1), SquareOffset(2, 1)]
    }

    pub fn king_offsets() -> [SquareOffset; 8] {
        [SquareOffset(-1,-1), SquareOffset(-1,0), SquareOffset(-1,1), SquareOffset(0,-1), SquareOffset(0,1), SquareOffset(1, -1), SquareOffset(1,0), SquareOffset(1, 1)]
    }
}
