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
