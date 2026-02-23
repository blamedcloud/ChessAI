use crate::chess_game::chess_piece::PieceName;
use crate::chess_game::chess_square::SquareID;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChessMove {
    Move(SquareID, SquareID),
    Capture(SquareID, SquareID),
    EnPassant(SquareID, SquareID),
    ShortCastle,
    LongCastle,
    Promotion(SquareID, PieceName),
    CapturePromotion(SquareID, SquareID, PieceName),
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Annotation {
    None,
    Check,
    CheckMate,
    Draw,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AnnotatedMove {
    pub chess_move: ChessMove,
    pub annotation: Annotation,
}

impl AnnotatedMove {
    pub fn new(chess_move: ChessMove, annotation: Annotation) -> Self {
        Self { chess_move, annotation }
    }
}

pub struct MoveList {
    moves: Vec<AnnotatedMove>,
}

impl MoveList {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn add_move(&mut self, ann_move: AnnotatedMove) {
        self.moves.push(ann_move);
    }

    pub fn has_move(&self, ann_move: AnnotatedMove) -> bool {
        self.moves.contains(&ann_move)
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }
}
