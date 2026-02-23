use std::fmt::{Display, Formatter};
use std::slice::Iter;
use crate::chess_game::chess_move::ChessMove;
use crate::chess_game::chess_piece::{ChessPiece, PieceName};
use crate::chess_game::chess_square::{ChessSquare, File, Rank, SquareID, SquareOffset};
use crate::chess_game::Player;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

    pub fn get_king_sq(&self, player: Player) -> &ChessSquare {
        //TODO: cache this value
        for i in 0..64 {
            if self.board[i].get_piece().is_some_and(|p| p.get_name() == PieceName::King && p.get_owner() == player) {
                return &self.board[i];
            }
        }
        panic!("No king square found");
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
        self.calc_seen();
    }

    fn clear_seen(&mut self) {
        for sq in self.board.iter_mut() {
            sq.clear_seen();
        }
    }

    fn calc_seen(&mut self) {
        self.clear_seen();
        for index in 0..64 {
            let sq = self.board[index];
            if let Some(piece) = sq.get_piece() {
                let id = sq.get_id();
                let player = piece.get_owner();
                match piece.get_name() {
                    PieceName::Pawn => self.pawn_seen(id, player),
                    PieceName::Knight => self.knight_seen(id, player),
                    PieceName::Bishop => self.bishop_seen(id, player),
                    PieceName::Rook => self.rook_seen(id, player),
                    PieceName::Queen => self.queen_seen(id, player),
                    PieceName::King => self.king_seen(id, player),
                }
            }
        }
    }

    fn pawn_seen(&mut self, id: SquareID, player: Player) {
        let forward_offset = match player {
            Player::White => SquareOffset(0, 1),
            Player::Black => SquareOffset(0, -1),
        };
        let offsets = [SquareOffset(-1, 0) + forward_offset, SquareOffset(1, 0) + forward_offset];
        for offset in &offsets {
            if let Some(target) = id.add_offset(*offset) {
                self.square_by_id_mut(target).add_seen_by(player, 1);
            }
        }
    }

    fn knight_seen(&mut self, id: SquareID, player: Player) {
        let offsets = PieceName::knight_offsets();
        for offset in offsets {
            if let Some(target) = id.add_offset(offset) {
                self.square_by_id_mut(target).add_seen_by(player, 1);
            }
        }
    }

    fn los_seen<F>(&mut self, id: SquareID, player: Player, f: F)
    where
        F: Fn(isize) -> SquareOffset
    {
        for i in 1..8 {
            let offset = f(i);
            if let Some(target) = id.add_offset(offset) {
                let sq = self.square_by_id_mut(target);
                sq.add_seen_by(player, 1);
                if sq.get_piece().is_some() {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn bishop_seen(&mut self, id: SquareID, player: Player) {
        self.los_seen(id, player, |i| SquareOffset(-i, -i));
        self.los_seen(id, player, |i| SquareOffset(-i, i));
        self.los_seen(id, player, |i| SquareOffset(i, -i));
        self.los_seen(id, player, |i| SquareOffset(i, i));
    }

    fn rook_seen(&mut self, id: SquareID, player: Player) {
        self.los_seen(id, player, |i| SquareOffset(-i, 0));
        self.los_seen(id, player, |i| SquareOffset(i, 0));
        self.los_seen(id, player, |i| SquareOffset(0, -i));
        self.los_seen(id, player, |i| SquareOffset(0, i));
    }

    fn queen_seen(&mut self, id: SquareID, player: Player) {
        // a queen can move like a bishop or rook
        self.bishop_seen(id, player);
        self.rook_seen(id, player);
    }

    fn king_seen(&mut self, id: SquareID, player: Player) {
        let offsets = PieceName::king_offsets();
        for offset in offsets {
            if let Some(target) = id.add_offset(offset) {
                self.square_by_id_mut(target).add_seen_by(player, 1);
            }
        }
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
    use crate::chess_game::chess_move::ChessMove;
    use crate::chess_game::chess_piece::PieceName;
    use crate::chess_game::chess_square::{File, Rank, SquareColor, SquareID};
    use crate::chess_game::Player;

    fn show() -> bool {
        false
    }

    #[test]
    fn test_new_board() {
        let board = ChessBoard::new();
        if show() {
            println!("board: \n{}", board);
        }

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

    #[test]
    fn test_initial_seen() {
        let start = ChessBoard::new();

        let mut board = ChessBoard::new();
        let n_sq = SquareID(File::B, Rank::One);
        let target = SquareID(File::C, Rank::Three);
        board.make_move(ChessMove::Move(n_sq, target), Player::White);

        if show() {
            println!("board: \n{}", board);
        }
        assert_ne!(start, board);

        board.make_move(ChessMove::Move(target, n_sq), Player::White);

        assert_eq!(start, board);
    }
}
