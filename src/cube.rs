//! Thin wrapper around `cubesim`'s facelet representation: the concrete cube
//! type this project works with, plus scramble generation and WCA-notation
//! formatting, neither of which `cubesim` provides out of the box.

use cubesim::{Cube, FaceletCube, Move, MoveVariant};
use rand::RngExt;

/// The cube representation used throughout this project. `FaceletCube` is
/// `cubesim`'s array-backed implementation; we only ever need sticker state
/// (not 3D geometry), so there's no reason to reach for `GeoCube`.
pub type CubeState = FaceletCube;

/// Builds a solved 3x3x3 cube.
pub fn solved() -> CubeState {
    CubeState::new(3)
}

/// The 18 face turns a 3x3x3 WCA scramble is built from. `cubesim::all_moves`
/// also yields wide moves and whole-cube rotations, which only make sense
/// for bigger cubes (or as notation shorthand), so we enumerate the basic
/// turns ourselves instead.
fn face_turns() -> [Move; 18] {
    use MoveVariant::{Double, Inverse, Standard};
    [
        Move::U(Standard), Move::U(Double), Move::U(Inverse),
        Move::L(Standard), Move::L(Double), Move::L(Inverse),
        Move::F(Standard), Move::F(Double), Move::F(Inverse),
        Move::R(Standard), Move::R(Double), Move::R(Inverse),
        Move::B(Standard), Move::B(Double), Move::B(Inverse),
        Move::D(Standard), Move::D(Double), Move::D(Inverse),
    ]
}

/// Generates a random sequence of `length` face turns for a 3x3x3 cube,
/// never turning the same face twice in a row.
pub fn random_scramble(length: usize) -> Vec<Move> {
    let moves = face_turns();
    let mut rng = rand::rng();
    let mut scramble: Vec<Move> = Vec::with_capacity(length);

    while scramble.len() < length {
        let candidate = moves[rng.random_range(0..moves.len())];
        if scramble.last().is_some_and(|&last| face_of(last) == face_of(candidate)) {
            continue;
        }
        scramble.push(candidate);
    }

    scramble
}

/// Applies a scramble to a solved cube.
pub fn scrambled(moves: &[Move]) -> CubeState {
    solved().apply_moves(moves)
}

/// Formats a sequence of moves as a WCA-notation scramble string, e.g.
/// `"R U' F2 L"`.
pub fn to_wca_string(moves: &[Move]) -> String {
    moves.iter().copied().map(notation).collect::<Vec<_>>().join(" ")
}

/// Identifies which face a move turns, ignoring its variant (and, for wide
/// moves, its layer count), so consecutive scramble moves can avoid
/// repeating a face.
fn face_of(mv: Move) -> char {
    match mv {
        Move::U(_) | Move::Uw(..) => 'U',
        Move::L(_) | Move::Lw(..) => 'L',
        Move::F(_) | Move::Fw(..) => 'F',
        Move::R(_) | Move::Rw(..) => 'R',
        Move::B(_) | Move::Bw(..) => 'B',
        Move::D(_) | Move::Dw(..) => 'D',
        Move::X(_) => 'x',
        Move::Y(_) => 'y',
        Move::Z(_) => 'z',
    }
}

fn notation(mv: Move) -> String {
    match mv {
        Move::U(v) => format!("U{}", suffix(v)),
        Move::L(v) => format!("L{}", suffix(v)),
        Move::F(v) => format!("F{}", suffix(v)),
        Move::R(v) => format!("R{}", suffix(v)),
        Move::B(v) => format!("B{}", suffix(v)),
        Move::D(v) => format!("D{}", suffix(v)),
        Move::Uw(n, v) => format!("{n}Uw{}", suffix(v)),
        Move::Lw(n, v) => format!("{n}Lw{}", suffix(v)),
        Move::Fw(n, v) => format!("{n}Fw{}", suffix(v)),
        Move::Rw(n, v) => format!("{n}Rw{}", suffix(v)),
        Move::Bw(n, v) => format!("{n}Bw{}", suffix(v)),
        Move::Dw(n, v) => format!("{n}Dw{}", suffix(v)),
        Move::X(v) => format!("x{}", suffix(v)),
        Move::Y(v) => format!("y{}", suffix(v)),
        Move::Z(v) => format!("z{}", suffix(v)),
    }
}

fn suffix(variant: MoveVariant) -> &'static str {
    match variant {
        MoveVariant::Standard => "",
        MoveVariant::Double => "2",
        MoveVariant::Inverse => "'",
    }
}
