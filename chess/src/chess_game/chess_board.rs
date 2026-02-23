use std::fmt::{Display, Formatter};
use std::slice::Iter;
use crate::chess_game::chess_square::{ChessSquare, SquareID};

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

    pub fn iter(&'_ self) -> Iter<'_, ChessSquare> {
        self.board.iter()
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