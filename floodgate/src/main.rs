use csa::Action;
use std::collections::HashMap;
use std::{env, fs, io, path, process};
use yasai::{Color, File, Move, Piece, PieceType, Position, Rank, Square};

#[derive(Debug, PartialEq, Eq)]
struct FinalPosition {
    board: Vec<String>,
    hands: [Vec<String>; 2],
    color: Color,
}

impl From<String> for FinalPosition {
    fn from(s: String) -> Self {
        let lines = s.split('\n').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let mut hands = [Vec::new(), Vec::new()];
        for &line in &lines {
            if line.starts_with("P+") {
                for i in 0..(line.len() - 2) / 4 {
                    hands[0].push(line[i * 4 + 4..i * 4 + 6].to_string());
                }
            }
            if line.starts_with("P-") {
                for i in 0..(line.len() - 2) / 4 {
                    hands[1].push(line[i * 4 + 4..i * 4 + 6].to_string());
                }
            }
        }
        hands.iter_mut().for_each(|v| v.sort());
        let color = match lines[lines.len() - 1] {
            "+" => Color::Black,
            "-" => Color::White,
            _ => unreachable!(),
        };
        Self {
            board: lines[..9].iter().map(|s| s.to_string()).collect(),
            hands,
            color,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args = env::args().collect::<Vec<_>>();
    if let Some(dir) = args.get(1) {
        for e in fs::read_dir(dir)? {
            load(e?.path().as_path())?;
        }
    } else {
        println!("usage: {} <dir>", args[0]);
        process::exit(1);
    }
    Ok(())
}

fn load(path: &path::Path) -> Result<(), io::Error> {
    println!("{}", path.display());
    let s = fs::read_to_string(path)?;
    if let Some(fp) = parse_result(&s) {
        match csa::parse_csa(&s) {
            Ok(record) => {
                let mut pos = Position::default();
                let mut hm = HashMap::new();
                hm.insert(pos.key(), 1);
                let mut moves = Vec::new();
                for (i, mr) in record.moves.iter().enumerate() {
                    if let Some(m) = action2move(&mr.action, &pos) {
                        pos.do_move(m);
                        moves.push(m);
                        println!("{:3} {m} => {:016x}", i + 1, pos.key());
                        *hm.entry(pos.key()).or_default() += 1;
                        if hm[&pos.key()] >= 4 {
                            if let Some(m) = moves.pop() {
                                pos.undo_move(m);
                                break;
                            }
                        }
                    }
                }
                assert_eq!(fp, FinalPosition::from(format!("{pos}")), "{s}");
            }
            Err(e) => {
                panic!("failed to parse csa {}: {}", path.display(), e);
            }
        }
    }
    Ok(())
}

fn parse_result(s: &str) -> Option<FinalPosition> {
    let lines = s.split('\n').collect::<Vec<_>>();
    if let Some(i) = lines.iter().position(|line| line.starts_with("'P1")) {
        if let Some(v) = lines[i..]
            .split_inclusive(|&s| s == "'+" || s == "'-")
            .next()
        {
            let s = v.iter().map(|s| &s[1..]).collect::<Vec<_>>().join("\n");
            return Some(s.into());
        }
    }
    None
}

fn action2move(action: &Action, pos: &Position) -> Option<Move> {
    if let Action::Move(c, from, to, pt) = action {
        let c = c2c(c);
        let to = sq2sq(to);
        let pt = pt2pt(pt);
        let mut piece = Piece::from_cp(c, pt);
        if *from != csa::Square::new(0, 0) {
            let from = sq2sq(from);
            let is_promotion = pos.piece_on(from) != Some(piece);
            if is_promotion {
                piece = piece.demoted();
            }
            Some(Move::new_normal(from, to, is_promotion, piece))
        } else {
            Some(Move::new_drop(to, piece))
        }
    } else {
        None
    }
}

fn c2c(c: &csa::Color) -> Color {
    match c {
        csa::Color::Black => Color::Black,
        csa::Color::White => Color::White,
    }
}

fn sq2sq(sq: &csa::Square) -> Square {
    Square::new(
        File::ALL[sq.file as usize - 1],
        Rank::ALL[sq.rank as usize - 1],
    )
}

fn pt2pt(pt: &csa::PieceType) -> PieceType {
    match pt {
        csa::PieceType::Pawn => PieceType::FU,
        csa::PieceType::Lance => PieceType::KY,
        csa::PieceType::Knight => PieceType::KE,
        csa::PieceType::Silver => PieceType::GI,
        csa::PieceType::Gold => PieceType::KI,
        csa::PieceType::Bishop => PieceType::KA,
        csa::PieceType::Rook => PieceType::HI,
        csa::PieceType::King => PieceType::OU,
        csa::PieceType::ProPawn => PieceType::TO,
        csa::PieceType::ProLance => PieceType::NY,
        csa::PieceType::ProKnight => PieceType::NK,
        csa::PieceType::ProSilver => PieceType::NG,
        csa::PieceType::Horse => PieceType::UM,
        csa::PieceType::Dragon => PieceType::RY,
        csa::PieceType::All => unreachable!(),
    }
}
