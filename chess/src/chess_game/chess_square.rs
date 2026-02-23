use std::fmt::{Display, Formatter};
use crate::chess_game::chess_piece::{ChessPiece, PieceName};
use crate::chess_game::Player;

#[derive(Debug, Copy, Clone)]
pub struct ChessSquare {
    id: SquareID,
    color: SquareColor,
    piece: Option<ChessPiece>,
    seen_by: [u8; 2], // how many of each players pieces see this square
}

impl ChessSquare {
    // returns the state of the i-th square at the start of a chess game.
    // starts from a1 (0), b1 (1), ..., up to h8 (63)
    pub fn initial(i: usize) -> Self {
        let id: SquareID = i.into();
        let color: SquareColor = id.into();
        let piece = match id {
            SquareID(file, Rank::One) => match file {
                File::A | File::H => Some(ChessPiece::new(Player::White, PieceName::Rook, false)),
                File::B | File::G => Some(ChessPiece::new(Player::White, PieceName::Knight, false)),
                File::C | File::F => Some(ChessPiece::new(Player::White, PieceName::Bishop, false)),
                File::D => Some(ChessPiece::new(Player::White, PieceName::Queen, false)),
                File::E => Some(ChessPiece::new(Player::White, PieceName::King, false)),
            },
            SquareID(_, Rank::Two) => Some(ChessPiece::new(Player::White, PieceName::Pawn, false)),
            SquareID(_, Rank::Seven) => Some(ChessPiece::new(Player::Black, PieceName::Pawn, false)),
            SquareID(file, Rank::Eight) => match file {
                File::A | File::H => Some(ChessPiece::new(Player::Black, PieceName::Rook, false)),
                File::B | File::G => Some(ChessPiece::new(Player::Black, PieceName::Knight, false)),
                File::C | File::F => Some(ChessPiece::new(Player::Black, PieceName::Bishop, false)),
                File::D => Some(ChessPiece::new(Player::Black, PieceName::Queen, false)),
                File::E => Some(ChessPiece::new(Player::Black, PieceName::King, false)),
            },
            _ => None,
        };
        let seen_by = match id {
            SquareID(file, Rank::One) => match file {
                File::A | File::H => [0, 0],
                _ => [1, 0],
            },
            SquareID(file, Rank::Two) => match file {
                File::D | File::E => [4, 0],
                _ => [1, 0],
            },
            SquareID(file, Rank::Three) => match file {
                File::C | File::F => [3, 0],
                _ => [2, 0]
            },
            SquareID(file, Rank::Eight) => match file {
                File::A | File::H => [0, 0],
                _ => [0, 1],
            },
            SquareID(file, Rank::Seven) => match file {
                File::D | File::E => [0, 4],
                _ => [0, 1],
            },
            SquareID(file, Rank::Six) => match file {
                File::C | File::F => [0, 3],
                _ => [0, 2]
            },
            _ => [0, 0]
        };
        Self { id, color, piece, seen_by }
    }

    pub fn new(id: SquareID, piece: Option<ChessPiece>, seen: [u8; 2]) -> Self {
        let color = id.into();
        Self { id, color, piece , seen_by: seen }
    }

    pub fn get_id(&self) -> SquareID {
        self.id
    }

    pub fn get_color(&self) -> SquareColor {
        self.color
    }

    pub fn get_piece(&self) -> Option<ChessPiece> {
        self.piece
    }

    pub fn clear_piece(&mut self) {
        self.piece = None;
    }

    pub fn set_piece(&mut self, piece: ChessPiece) {
        self.piece = Some(piece);
    }

    pub fn get_seen(&self) -> [u8; 2] {
        self.seen_by
    }

    pub fn is_seen_by(&self, player: Player) -> bool {
        match player {
            Player::White => self.seen_by[0] > 0,
            Player::Black => self.seen_by[1] > 0,
        }
    }

    pub fn not_seen_by(&self, player: Player) -> bool {
        match player {
            Player::White => self.seen_by[0] == 0,
            Player::Black => self.seen_by[1] == 0,
        }
    }

    pub fn clear_seen(&mut self) {
        self.seen_by = [0, 0];
    }

    pub fn add_seen(&mut self, seen: [u8; 2]) {
        self.seen_by = [self.seen_by[0] + seen[0], self.seen_by[1] + seen[1]];
    }
}

impl Display for ChessSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.piece.is_some() {
            self.piece.unwrap().fmt(f)?;
        } else {
            match self.color {
                SquareColor::Light => write!(f, " ")?,
                SquareColor::Dark => write!(f, "_")?,
            };
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SquareID(pub File, pub Rank);

impl SquareID {
    pub fn file(&self) -> File { self.0 }
    pub fn rank(&self) -> Rank { self.1 }

    pub fn add_offset(&self, offset: SquareOffset) -> Option<SquareID> {
        let fu: usize = self.0.into();
        let ru: usize = self.1.into();
        let fi: isize = fu as isize;
        let ri: isize = ru as isize;
        let new_f = fi + offset.0;
        let new_r = ri + offset.1;
        if new_f >= 0 && new_f < 8 && new_r >= 0 && new_r < 8 {
            Some(SquareID((new_f as usize).into(), (new_r as usize).into()))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SquareOffset(pub isize, pub isize);


impl From<usize> for SquareID {
    fn from(value: usize) -> Self {
        // values 64 and bigger get translated via mod into usable ones
        let v = value % 64;
        let file = v % 8;
        let rank = v / 8;
        SquareID(file.into(), rank.into())
    }
}

impl From<SquareID> for usize {
    fn from(value: SquareID) -> usize {
        let file: usize = value.0.into();
        let rank: usize = value.1.into();
        rank * 8 + file
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        let v = value % 8;
        match v {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            6 => Self::Seven,
            7 => Self::Eight,
            _ => unreachable!(),
        }
    }
}

impl From<Rank> for usize {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<usize> for File {
    fn from(value: usize) -> Self {
        let v = value % 8;
        match v {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => unreachable!(),
        }
    }
}

impl From<File> for usize {
    fn from(file: File) -> Self {
        match file {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SquareColor {
    Dark,
    Light,
}

impl From<SquareID> for SquareColor {
    fn from(value: SquareID) -> Self {
        let r: usize = value.rank().into();
        let f: usize = value.file().into();
        match (r+f) % 2 {
            0 => Self::Dark,
            1 => Self::Light,
            _ => unreachable!(),
        }
    }
}
