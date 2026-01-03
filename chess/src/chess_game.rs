use crate::chess_game::chess_square::{ChessSquare, SquareID};

pub mod chess_square;
pub mod chess_piece;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Result {
    WhiteWin,
    BlackWin,
    Draw,
}

pub struct ChessGame {
    board: [ChessSquare; 64],
    active_player: Player,
    result: Option<Result>,
    ep_square: Option<SquareID>,
    draw_clock: usize,
    turn_num: usize,
}

impl ChessGame {
    pub fn new() -> Self {
        let board : [ChessSquare; 64] = std::array::from_fn(|i| ChessSquare::initial(i));
        Self {
            board,
            active_player: Player::White,
            result: None,
            ep_square: None,
            draw_clock: 0,
            turn_num: 1,
        }
    }
}
