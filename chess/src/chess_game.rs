use crate::chess_game::chess_board::ChessBoard;
use crate::chess_game::chess_move::{AnnotatedMove, Annotation, ChessMove, MoveList};
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
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ChessGameState {
    board: ChessBoard,
    active_player: Player,
    result: Option<GameResult>,
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

    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    pub fn active_player(&self) -> Player {
        self.active_player
    }

    pub fn result(&self) -> Option<GameResult> {
        self.result
    }

    pub fn turn(&self) -> usize {
        self.turn_num
    }

    pub fn get_fen(&self) -> String {
        let mut fen = String::new();
        for r in (0..8).rev() {
            fen += self.get_rank_fen(r.into()).as_str();
            if r != 0 {
                fen += "/";
            }
        }
        match self.active_player {
            Player::White => fen += " w ",
            Player::Black => fen += " b ",
        };
        fen += self.get_castling_fen().as_str();
        fen += " ";
        match self.ep_square {
            None => fen += "-",
            Some(ep_square) => fen += &ep_square.to_str(),
        }
        fen += " ";
        fen += self.draw_clock.to_string().as_str();
        fen += " ";
        fen += self.turn_num.to_string().as_str();
        fen
    }

    fn get_rank_fen(&self, rank: Rank) -> String {
        let mut rank_fen = String::new();
        let mut empty_sq = 0;
        for f in 0..8 {
            let sq = self.board.square_by_id(SquareID(f.into(), rank));
            if let Some(piece) = sq.get_piece() {
                if empty_sq > 0 {
                    rank_fen += empty_sq.to_string().as_str();
                    empty_sq = 0;
                }
                rank_fen += piece.to_string().as_str();
            } else {
                empty_sq += 1;
            }
        }
        if empty_sq > 0 {
            rank_fen += empty_sq.to_string().as_str();
        }
        rank_fen
    }

    fn get_castling_fen(&self) -> String {
        let mut castling_fen = String::new();
        let w_king = self.castling_valid(SquareID(File::E, Rank::One), PieceName::King);
        let wk_rook = self.castling_valid(SquareID(File::H, Rank::One), PieceName::Rook);
        if w_king && wk_rook {
            castling_fen += "K";
        }
        let wq_rook = self.castling_valid(SquareID(File::A, Rank::One), PieceName::Rook);
        if w_king && wq_rook {
            castling_fen += "Q";
        }

        let b_king = self.castling_valid(SquareID(File::E, Rank::Eight), PieceName::King);
        let bk_rook = self.castling_valid(SquareID(File::H, Rank::Eight), PieceName::Rook);
        if b_king && bk_rook {
            castling_fen += "k";
        }
        let bq_rook = self.castling_valid(SquareID(File::A, Rank::Eight), PieceName::Rook);
        if b_king && bq_rook {
            castling_fen += "q";
        }
        castling_fen
    }

    fn castling_valid(&self, id: SquareID, name: PieceName) -> bool {
        self.board.square_by_id(id).get_piece().is_some_and(|p| p.get_name() == name && p.not_moved())
    }

    pub fn make_move(&mut self, annotated_move: AnnotatedMove) {
        self.ep_square = None;
        match annotated_move.chess_move {
            ChessMove::Move(id, target) => {
                if self.board.square_by_id(id).get_piece().is_some_and(|p| p.get_name() == PieceName::Pawn) {
                    self.draw_clock = 0;
                    // handle ep square
                    let offset = id.calc_offset(target);
                    if offset.file() == 0 && offset.rank().abs() == 2 {
                        let ep_offset = SquareOffset(0, offset.rank() / 2);
                        let ep_sq = id.add_offset(ep_offset).unwrap();
                        self.ep_square = Some(ep_sq);
                    }
                } else {
                    self.draw_clock += 1;
                }
            },
            ChessMove::Capture(_, _) => self.draw_clock = 0,
            ChessMove::EnPassant(_, _) => self.draw_clock = 0,
            ChessMove::ShortCastle => self.draw_clock += 1,
            ChessMove::LongCastle => self.draw_clock += 1,
            ChessMove::Promotion(_, _) => self.draw_clock = 0,
            ChessMove::CapturePromotion(_, _, _) => self.draw_clock = 0,
        }

        match annotated_move.annotation {
            Annotation::CheckMate => {
                match self.active_player {
                    Player::White => self.result = Some(GameResult::WhiteWin),
                    Player::Black => self.result = Some(GameResult::BlackWin),
                };
            },
            Annotation::Draw => self.result = Some(GameResult::Draw),
            _ => {},
        }
        if self.active_player == Player::Black {
            self.turn_num += 1;
        }

        if self.result == None && self.draw_clock >= 50 {
            self.result = Some(GameResult::Draw);
        }

        self.board.make_move(annotated_move.chess_move, self.active_player);
        self.active_player = self.active_player.opponent();
    }

    pub fn get_legal_moves(&self) -> MoveList {
        let mut move_list = MoveList::new();
        let opponent = self.active_player.opponent();
        let all_moves = self.get_all_moves();
        for m in all_moves {
            let my_copy = {
                let mut my_copy = self.clone();
                my_copy.make_move(AnnotatedMove::new(m, Annotation::None));
                my_copy
            };
            let king_sq = my_copy.board.get_king_sq(self.active_player);
            if king_sq.not_seen_by(opponent) {
                // move is legal
                let is_check = my_copy.board.get_king_sq(opponent).is_seen_by(self.active_player);
                let has_legal_move = my_copy.has_legal_moves();
                let annotation = match (is_check, has_legal_move) {
                    (true, true) => Annotation::Check,
                    (true, false) => Annotation::CheckMate,
                    (false, true) => Annotation::None,
                    (false, false) => Annotation::Draw,
                };
                move_list.add_move(AnnotatedMove::new(m, annotation));
            }
        }
        move_list
    }

    fn has_legal_moves(&self) -> bool {
        let opponent = self.active_player.opponent();
        let all_moves = self.get_all_moves();
        for m in all_moves {
            let my_copy = {
                let mut my_copy = self.clone();
                my_copy.make_move(AnnotatedMove::new(m, Annotation::None));
                my_copy
            };
            let king_sq = my_copy.board.get_king_sq(self.active_player);
            if king_sq.not_seen_by(opponent) {
                return true;
            }
        }
        false
    }

    fn get_all_moves(&self) -> Vec<ChessMove> {
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
        let offsets = PieceName::knight_offsets();
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

    fn add_los_moves<F>(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>, f: F)
    where
        F: Fn(isize) -> SquareOffset
    {
        let id = sq.get_id();
        for i in 1..8 {
            let offset = f(i);
            if let Some(target) = id.add_offset(offset) {
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

    fn add_bishop_moves(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>) {
        self.add_los_moves(sq, moves, |i| SquareOffset(-i, -i));
        self.add_los_moves(sq, moves, |i| SquareOffset(-i, i));
        self.add_los_moves(sq, moves, |i| SquareOffset(i, -i));
        self.add_los_moves(sq, moves, |i| SquareOffset(i, i));
    }

    fn add_rook_moves(&self, sq: &ChessSquare, moves: &mut Vec<ChessMove>) {
        self.add_los_moves(sq, moves, |i| SquareOffset(-i, 0));
        self.add_los_moves(sq, moves, |i| SquareOffset(i, 0));
        self.add_los_moves(sq, moves, |i| SquareOffset(0, -i));
        self.add_los_moves(sq, moves, |i| SquareOffset(0, i));
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
        let offsets = PieceName::king_offsets();
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
    use crate::chess_game::chess_move::{AnnotatedMove, Annotation, ChessMove};
    use crate::chess_game::chess_square::{File, Rank, SquareID};
    use crate::chess_game::{ChessGameState, GameResult, Player};

    fn show() -> bool {
        true
    }

    #[test]
    fn test_fen() {
        let mut game = ChessGameState::new();
        let fen = game.get_fen();
        assert_eq!(fen, String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
        // pawn to e4
        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::E, Rank::Two), SquareID(File::E, Rank::Four)), Annotation::None));
        let fen = game.get_fen();
        assert_eq!(fen, String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"));
        // pawn to c5
        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::C, Rank::Seven), SquareID(File::C, Rank::Five)), Annotation::None));
        let fen = game.get_fen();
        assert_eq!(fen, String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"));
        // Nf3
        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::G, Rank::One), SquareID(File::F, Rank::Three)), Annotation::None));
        let fen = game.get_fen();
        assert_eq!(fen, String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"));

    }

    #[test]
    fn initial_moves() {
        let game = ChessGameState::new();
        let moves = game.get_legal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn basic_opening() {
        let mut game = ChessGameState::new();
        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::E, Rank::Two), SquareID(File::E, Rank::Four)), Annotation::None));
        assert_eq!(game.active_player, Player::Black);
        assert_eq!(game.turn(), 1);
        let moves = game.get_legal_moves();
        assert_eq!(moves.len(), 20);

        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::E, Rank::Seven), SquareID(File::E, Rank::Five)), Annotation::None));
        assert_eq!(game.active_player, Player::White);
        assert_eq!(game.turn(), 2);
        let moves = game.get_legal_moves();
        assert_eq!(moves.len(), 29);

        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::G, Rank::One), SquareID(File::F, Rank::Three)), Annotation::None));
        assert_eq!(game.active_player, Player::Black);
        assert_eq!(game.turn(), 2);
        let moves = game.get_legal_moves();
        assert_eq!(moves.len(), 29);
        assert_eq!(game.board().square_by_id(SquareID(File::E, Rank::Five)).get_seen(), [1, 0]);

        game.make_move(AnnotatedMove::new(ChessMove::Move(SquareID(File::B, Rank::Eight), SquareID(File::C, Rank::Six)), Annotation::None));
        assert_eq!(game.active_player, Player::White);
        assert_eq!(game.turn(), 3);
        let moves = game.get_legal_moves();
        assert_eq!(moves.len(), 27);
        assert_eq!(game.board().square_by_id(SquareID(File::E, Rank::Five)).get_seen(), [1, 1]);

        if show() {
            println!("{}", game.board);
        }
    }

    #[test]
    fn scholars_mate() {
        let mut game = ChessGameState::new();
        let moves = game.get_legal_moves();
        let e4 = AnnotatedMove::new(ChessMove::Move(SquareID(File::E, Rank::Two), SquareID(File::E, Rank::Four)), Annotation::None);
        assert!(moves.has_move(e4));
        game.make_move(e4);

        let moves = game.get_legal_moves();
        let e5 = AnnotatedMove::new(ChessMove::Move(SquareID(File::E, Rank::Seven), SquareID(File::E, Rank::Five)), Annotation::None);
        assert!(moves.has_move(e5));
        game.make_move(e5);

        let moves = game.get_legal_moves();
        let bc4 = AnnotatedMove::new(ChessMove::Move(SquareID(File::F, Rank::One), SquareID(File::C, Rank::Four)), Annotation::None);
        assert!(moves.has_move(bc4));
        game.make_move(bc4);

        let moves = game.get_legal_moves();
        let bc5 = AnnotatedMove::new(ChessMove::Move(SquareID(File::F, Rank::Eight), SquareID(File::C, Rank::Five)), Annotation::None);
        assert!(moves.has_move(bc5));
        game.make_move(bc5);

        let moves = game.get_legal_moves();
        let qf3 = AnnotatedMove::new(ChessMove::Move(SquareID(File::D, Rank::One), SquareID(File::F, Rank::Three)), Annotation::None);
        assert!(moves.has_move(qf3));
        game.make_move(qf3);

        let moves = game.get_legal_moves();
        let nc6 = AnnotatedMove::new(ChessMove::Move(SquareID(File::B, Rank::Eight), SquareID(File::C, Rank::Six)), Annotation::None);
        assert!(moves.has_move(nc6));
        game.make_move(nc6);

        let moves = game.get_legal_moves();
        let bf7 = AnnotatedMove::new(ChessMove::Capture(SquareID(File::C, Rank::Four), SquareID(File::F, Rank::Seven)), Annotation::Check);
        assert!(moves.has_move(bf7));
        let qf7 = AnnotatedMove::new(ChessMove::Capture(SquareID(File::F, Rank::Three), SquareID(File::F, Rank::Seven)), Annotation::CheckMate);
        assert!(moves.has_move(qf7));
        game.make_move(qf7);

        assert!(game.result().is_some_and(|r| r == GameResult::WhiteWin));
        assert_eq!(game.get_fen(), "r1bqk1nr/pppp1Qpp/2n5/2b1p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4");

        if show() {
            println!("{}", game.board);
        }
    }

    #[test]
    fn mate_in_2() {
        let mut game = ChessGameState::new();

        let moves = game.get_legal_moves();
        let f3 = AnnotatedMove::new(ChessMove::Move(SquareID(File::F, Rank::Two), SquareID(File::F, Rank::Three)), Annotation::None);
        assert!(moves.has_move(f3));
        game.make_move(f3);

        let moves = game.get_legal_moves();
        let e5 = AnnotatedMove::new(ChessMove::Move(SquareID(File::E, Rank::Seven), SquareID(File::E, Rank::Five)), Annotation::None);
        assert!(moves.has_move(e5));
        game.make_move(e5);

        let moves = game.get_legal_moves();
        let g4 = AnnotatedMove::new(ChessMove::Move(SquareID(File::G, Rank::Two), SquareID(File::G, Rank::Four)), Annotation::None);
        assert!(moves.has_move(g4));
        game.make_move(g4);

        let moves = game.get_legal_moves();
        let qh4 = AnnotatedMove::new(ChessMove::Move(SquareID(File::D, Rank::Eight), SquareID(File::H, Rank::Four)), Annotation::CheckMate);
        assert!(moves.has_move(qh4));
        game.make_move(qh4);

        assert!(game.result().is_some_and(|r| r == GameResult::BlackWin));
        assert_eq!(game.get_fen(), "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3");

        if show() {
            println!("{}", game.board);
        }
    }
}
