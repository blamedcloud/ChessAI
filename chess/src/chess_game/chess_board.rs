use std::fmt::{Display, Formatter};
use std::slice::Iter;
use crate::chess_game::chess_move::ChessMove;
use crate::chess_game::chess_piece::ChessPiece;
use crate::chess_game::chess_square::{ChessSquare, File, Rank, SquareID};
use crate::chess_game::Player;

pub struct ChessBoard {
    board: [ChessSquare; 64],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        Self {
            board: std::array::from_fn(|i| ChessSquare::initial(i)),
        }
    }

    pub fn square_by_id(&self, id: SquareID) -> &ChessSquare {
        let index: usize = id.into();
        &self.board[index]
    }

    fn square_by_id_mut(&mut self, id: SquareID) -> &mut ChessSquare {
        let index: usize = id.into();
        &mut self.board[index]
    }

    pub fn iter(&'_ self) -> Iter<'_, ChessSquare> {
        self.board.iter()
    }

    pub fn make_move(&mut self, chess_move: ChessMove, player: Player) {
        match chess_move {
            ChessMove::Move(id, target_id) | ChessMove::Capture(id, target_id)=> {
                let sq = self.square_by_id_mut(id);
                let mut piece = sq.get_piece().unwrap();
                sq.clear_piece();
                let target_sq = self.square_by_id_mut(target_id);
                piece.set_moved(true);
                target_sq.set_piece(piece);
            },
            ChessMove::EnPassant(id, target_id) => {
                let sq = self.square_by_id_mut(id);
                let piece = sq.get_piece().unwrap();
                sq.clear_piece();
                let target_sq = self.square_by_id_mut(target_id);
                target_sq.set_piece(piece);
                let ep_id = SquareID(target_id.file(), id.rank());
                let ep_sq = self.square_by_id_mut(ep_id);
                ep_sq.clear_piece();
            },
            ChessMove::ShortCastle => {
                let rank = match player {
                    Player::White => Rank::One,
                    Player::Black => Rank::Eight,
                };
                let king_id = SquareID(File::E, rank);
                let king_sq = self.square_by_id_mut(king_id);
                let mut king = king_sq.get_piece().unwrap();
                king.set_moved(true);
                king_sq.clear_piece();
                let new_king_id = SquareID(File::G, rank);
                let new_king_sq = self.square_by_id_mut(new_king_id);
                new_king_sq.set_piece(king);

                let rook_id = SquareID(File::H, rank);
                let rook_sq = self.square_by_id_mut(rook_id);
                let mut rook = rook_sq.get_piece().unwrap();
                rook.set_moved(true);
                rook_sq.clear_piece();
                let new_rook_id = SquareID(File::F, rank);
                let new_rook_sq = self.square_by_id_mut(new_rook_id);
                new_rook_sq.set_piece(rook);
            },
            ChessMove::LongCastle => {
                let rank = match player {
                    Player::White => Rank::One,
                    Player::Black => Rank::Eight,
                };
                let king_id = SquareID(File::E, rank);
                let king_sq = self.square_by_id_mut(king_id);
                let mut king = king_sq.get_piece().unwrap();
                king.set_moved(true);
                king_sq.clear_piece();
                let new_king_id = SquareID(File::C, rank);
                let new_king_sq = self.square_by_id_mut(new_king_id);
                new_king_sq.set_piece(king);

                let rook_id = SquareID(File::A, rank);
                let rook_sq = self.square_by_id_mut(rook_id);
                let mut rook = rook_sq.get_piece().unwrap();
                rook.set_moved(true);
                rook_sq.clear_piece();
                let new_rook_id = SquareID(File::D, rank);
                let new_rook_sq = self.square_by_id_mut(new_rook_id);
                new_rook_sq.set_piece(rook);
            },
            ChessMove::Promotion(target_id, piece_name) => {
                let id = match player {
                    Player::White => SquareID(target_id.file(), Rank::Seven),
                    Player::Black => SquareID(target_id.file(), Rank::Two),
                };
                let sq = self.square_by_id_mut(id);
                sq.clear_piece();
                let target_sq = self.square_by_id_mut(target_id);
                target_sq.set_piece(ChessPiece::new(player, piece_name, true));
            },
            ChessMove::CapturePromotion(id, target_id, piece_name) => {
                let sq = self.square_by_id_mut(id);
                sq.clear_piece();
                let target_sq = self.square_by_id_mut(target_id);
                target_sq.set_piece(ChessPiece::new(player, piece_name, true));
            }
        }
        self.clear_seen();
        self.calc_seen();
    }

    fn clear_seen(&mut self) {
        for sq in self.board.iter_mut() {
            sq.clear_seen();
        }
    }

    fn calc_seen(&mut self) {
        todo!()
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in (0..=7).rev() {
            for file in 0..=7 {
                let id = SquareID(file.into(), rank.into());
                let index: usize = id.into();
                let square = &self.board[index];
                square.fmt(f)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::chess_game::chess_board::ChessBoard;
    use crate::chess_game::chess_piece::PieceName;
    use crate::chess_game::chess_square::{File, Rank, SquareColor, SquareID};
    use crate::chess_game::Player;

    #[test]
    fn test_new_board() {
        let board = ChessBoard::new();
        println!("board: \n{}", board);

        let a1 = &board.board[0];
        assert_eq!(a1.get_id(), SquareID(File::A, Rank::One));
        assert_eq!(a1.get_color(), SquareColor::Dark);
        assert_eq!(a1.get_seen(), [0, 0]);
        assert!(a1.get_piece().is_some());
        let a1_piece = a1.get_piece().unwrap();
        assert_eq!(a1_piece.get_owner(), Player::White);
        assert_eq!(a1_piece.get_name(), PieceName::Rook);

        let f2 = &board.board[13];
        assert_eq!(f2.get_id(), SquareID(File::F, Rank::Two));
        assert_eq!(f2.get_color(), SquareColor::Dark);
        assert_eq!(f2.get_seen(), [1, 0]);
        assert!(f2.get_piece().is_some());
        let f2_piece = f2.get_piece().unwrap();
        assert_eq!(f2_piece.get_owner(), Player::White);
        assert_eq!(f2_piece.get_name(), PieceName::Pawn);

        let c6 = &board.board[42];
        assert_eq!(c6.get_id(), SquareID(File::C, Rank::Six));
        assert_eq!(c6.get_seen(), [0, 3]);
        assert!(c6.get_piece().is_none());

        let h8 = &board.board[63];
        assert_eq!(h8.get_id(), SquareID(File::H, Rank::Eight));
        assert_eq!(h8.get_color(), SquareColor::Dark);
        assert_eq!(h8.get_seen(), [0, 0]);
        assert!(h8.get_piece().is_some());
        let h8_piece = h8.get_piece().unwrap();
        assert_eq!(h8_piece.get_owner(), Player::Black);
        assert_eq!(h8_piece.get_name(), PieceName::Rook);
    }
}