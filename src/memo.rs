//! Cycle-breaking: given a scrambled cube and a buffer piece, work out the
//! sequence of pieces visited -- and therefore the letter pairs to
//! memorize -- to solve that piece type with a buffer-based method (Old
//! Pochmann, M2, 3-style, ...).

use crate::cube::CubeState;
use crate::lettering::Buffers;

/// Piece type being solved, which determines which stickers matter and how
/// cycles are broken.
#[derive(Clone, Copy)]
pub enum Piece {
    Corners,
    Edges,
}

/// One step of a memo sequence: the buffer swaps with the piece at this
/// sticker next.
#[allow(dead_code)]
pub struct CycleStep {
    pub letter: char,
}

/// Walks the corner or edge permutation cycles of `cube` starting from the
/// relevant buffer in `buffers`, returning the memo sequence a blindfolded
/// solver would learn.
#[allow(unused_variables)]
pub fn cycles(cube: &CubeState, piece: Piece, buffers: &Buffers) -> Vec<CycleStep> {
    todo!("walk piece permutation cycles from the buffer using cubesim::sticker_index")
}
