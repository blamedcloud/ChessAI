use crate::chess_game::chess_board::ChessBoard;
use crate::chess_game::chess_move::ChessMove;
use crate::chess_game::chess_piece::{ChessPiece, PieceName};
use crate::chess_game::chess_square::{ChessSquare, File, Rank, SquareID, SquareOffset};

pub mod chess_square;
pub mod chess_piece;
pub mod chess_move;
pub mod chess_board;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Result {
    WhiteWin,
    BlackWin,
    Draw,
}

pub struct ChessGameState {
    board: ChessBoard,
    active_player: Player,
    result: Option<Result>,
    ep_square: Option<SquareID>,
    draw_clock: usize,
    turn_num: usize,
}

impl ChessGameState {
    pub fn new() -> Self {
        Self {
            board: ChessBoard::new(),
            active_player: Player::White,
            result: None,
            ep_square: None,
            draw_clock: 0,
            turn_num: 1,
        }
    }

    pub fn get_legal_moves(&self) -> Vec<ChessMove> {
        // need to account for check/checkmate
        self.get_all_moves()
    }

    fn get_all_moves(&self) -> Vec<ChessMove> {
        if self.result.is_some() {
            return Vec::new();
        }
        let mut moves = Vec::new();

        for square in self.board.iter() {
            if square.get_piece().is_some_and(|p| p.get_owner() == self.active_player) {
                let piece = square.get_piece().unwrap();
                match piece.get_name() {
                    PieceName::Pawn => self.add_pawn_moves(square, piece, &mut moves),
                    PieceName::Knight => self.add_knight_moves(square, &mut moves),
                    PieceName::Bishop => self.add_bishop_moves(square, &mut moves),
                    PieceName::Rook => self.add_rook_moves(square, &mut moves),
                    PieceName::Queen => self.add_queen_moves(square, &mut moves),
                    PieceName::King => self.add_king_moves(square, piece, &mut moves),
                }
            }
        }
        moves
    }

    fn add_pawn_moves(&self, sq: &ChessSquare, piece: ChessPiece, moves: &mut Vec<ChessMove>) {
        let id = sq.get_id();
        let promotion_rank = match self.active_player {
            Player::White => Rank::Eight,
            Player::Black => Rank::One,
        };
        let promote_to = [PieceName::Knight, PieceName::Bishop, PieceName::Rook, PieceName::Queen];
        //push
        let push_offset = match self.active_player {
            Player::White => SquareOffset(0, 1),
            Player::Black => SquareOffset(0, -1),
        };
        let push_sq = id.add_offset(push_offset).unwrap();
        if self.board.square_by_id(push_sq).get_piece().is_none() {
            if push_sq.rank() == promotion_rank {
                for name in promote_to.iter() {
                    moves.push(ChessMove::Promotion(push_sq, *name));
                }
            } else {
                moves.push(ChessMove::Move(id, push_sq));
                if piece.not_moved() {
                    let double_sq = push_sq.add_offset(push_offset).unwrap();
                    if self.board.square_by_id(double_sq).get_piece().is_none() {
                        moves.push(ChessMove::Move(id, double_sq));
                    }
                }
            }
        }
        // captures
        let capture_offsets = [SquareOffset(-1, 0), SquareOffset(1, 0)];
        for offset in capture_offsets.iter() {
            let capture = push_sq.add_offset(*offset);
            if let Some(target_id) = capture {
                let target_sq = self.board.square_by_id(target_id);
                if target_sq.get_piece().is_some_and(|p| p.get_owner() == self.active_player.opponent()) {
                    if target_id.rank() == promotion_rank {
                        for name in promote_to.iter() {
                            moves.push(ChessMove::CapturePromotion(id, target_id, *name));
                        }
                    } else {
                        moves.push(ChessMove::Capture(id, target_id));
                    }
                } else if target_sq.get_piece().is_none() && self.ep_square.is_some_and(|ep_sq| ep_sq == target_id) {
                    moves.push(ChessMove::EnPassant(id, target_id));
                }
            }
        }
    }

    fn add_knight_moves(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>) {
        let id = sq.get_id();
        let offsets = [SquareOffset(-2,-1), SquareOffset(-2,1), SquareOffset(-1,-2), SquareOffset(-1,2), SquareOffset(1,-2), SquareOffset(1, 2), SquareOffset(2,-1), SquareOffset(2, 1)];
        for offset in offsets.into_iter() {
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                    moves.push(ChessMove::Capture(id, target));
                }
            }
        }
    }

    fn add_bishop_moves(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>) {
        let id = sq.get_id();
        // left-down (-file, -rank)
        for i in 1..8 {
            let offset = SquareOffset(-i, -i);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
        // left-up (-file, +rank)
        for i in 1..8 {
            let offset = SquareOffset(-i, i);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
        // right-down (+file, -rank)
        for i in 1..8 {
            let offset = SquareOffset(i, -i);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
        // right-up (+file, +rank)
        for i in 1..8 {
            let offset = SquareOffset(i, i);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn add_rook_moves(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>) {
        let id = sq.get_id();
        // left (-file)
        for i in 1..8 {
            let offset = SquareOffset(-i, 0);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
        // right (+file)
        for i in 1..8 {
            let offset = SquareOffset(i, 0);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
        // down (-rank)
        for i in 1..8 {
            let offset = SquareOffset(0, -i);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
        // up (+rank)
        for i in 1..8 {
            let offset = SquareOffset(0, i);
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.get_piece().is_none() {
                    moves.push(ChessMove::Move(id, target));
                } else {
                    if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn add_queen_moves(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>) {
        // a queen can move like a bishop or rook
        self.add_bishop_moves(sq, moves);
        self.add_rook_moves(sq, moves);
    }

    fn add_king_moves(&self, sq: &ChessSquare, piece: ChessPiece, moves: &mut Vec<ChessMove>) {
        let id = sq.get_id();
        let opponent = self.active_player.opponent();
        // standard moves
        let offsets = [SquareOffset(-1,-1), SquareOffset(-1,0), SquareOffset(-1,1), SquareOffset(0,-1), SquareOffset(0,1), SquareOffset(1, -1), SquareOffset(1,0), SquareOffset(1, 1)];
        for offset in offsets.into_iter() {
            let offset_sq = id.add_offset(offset);
            if let Some(target) = offset_sq {
                let target_sq = self.board.square_by_id(target);
                if target_sq.not_seen_by(opponent) {
                    if target_sq.get_piece().is_none() {
                        moves.push(ChessMove::Move(id, target));
                    } else if target_sq.get_piece().unwrap().get_owner() != self.active_player {
                        moves.push(ChessMove::Capture(id, target));
                    }
                }
            }
        }
        // castling
        if piece.not_moved() && sq.not_seen_by(opponent) {
            let rank = id.rank();
            //short-castle
            let rook = self.board.square_by_id(SquareID(File::H, rank));
            if rook.get_piece().is_some_and(|p| p.get_name() == PieceName::Rook && p.not_moved()) {
                let b_sq = self.board.square_by_id(SquareID(File::F, rank));
                if b_sq.get_piece().is_none() && b_sq.not_seen_by(opponent) {
                    let n_sq = self.board.square_by_id(SquareID(File::G, rank));
                    if n_sq.get_piece().is_none() && n_sq.not_seen_by(opponent) {
                        moves.push(ChessMove::ShortCastle);
                    }
                }
            }
            //long-castle
            let rook = self.board.square_by_id(SquareID(File::A, rank));
            if rook.get_piece().is_some_and(|p| p.get_name() == PieceName::Rook && p.not_moved()) {
                let q_sq = self.board.square_by_id(SquareID(File::D, rank));
                if q_sq.get_piece().is_none() && q_sq.not_seen_by(opponent) {
                    let b_sq = self.board.square_by_id(SquareID(File::C, rank));
                    if b_sq.get_piece().is_none() && b_sq.not_seen_by(opponent) {
                        let n_sq = self.board.square_by_id(SquareID(File::B, rank));
                        if n_sq.get_piece().is_none() {
                            moves.push(ChessMove::LongCastle);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chess_game::ChessGameState;

    #[test]
    fn initial_moves() {
        let game = ChessGameState::new();
        let moves = game.get_legal_moves();
        assert_eq!(moves.len(), 20);
    }
}