use std::fmt::{Display, Formatter};
use crate::chess_game::Player;


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
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.owner {
            Player::White => match self.name {
                PieceName::Pawn => write!(f, "P"),
                PieceName::Knight => write!(f, "N"),
                PieceName::Bishop => write!(f, "B"),
                PieceName::Rook => write!(f, "R"),
                PieceName::Queen => write!(f, "Q"),
                PieceName::King => write!(f, "K"),
            },
            Player::Black => match self.name {
                PieceName::Pawn => write!(f, "p"),
                PieceName::Knight => write!(f, "n"),
                PieceName::Bishop => write!(f, "b"),
                PieceName::Rook => write!(f, "r"),
                PieceName::Queen => write!(f, "q"),
                PieceName::King => write!(f, "k"),
            }
        }
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
