use crate::chess_game::chess_piece::{ChessPiece, PieceName};
use crate::chess_game::chess_square::SquareID;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChessMove {
    Move(SquareID, SquareID),
    Capture(SquareID, SquareID),
    ShortCastle,
    LongCastle,
    Promotion(SquareID, PieceName),
    CapturePromotion(SquareID, SquareID, PieceName),
}

