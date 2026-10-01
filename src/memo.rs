//! Cycle-breaking: given a scrambled cube and a buffer piece, work out the
//! sequence of pieces visited -- and therefore the letter pairs to
//! memorize -- to solve that piece type with a buffer-based method (Old
//! Pochmann, M2). See `cycle` for the shared permutation-walking primitives
//! this and `threestyle` are both built on.

use crate::cube::CubeState;
use crate::cycle;
pub use crate::cycle::Piece;
use crate::lettering::Buffers;

/// One step of a memo sequence: the buffer swaps with the piece at this
/// sticker next.
pub struct CycleStep {
    pub letter: char,
}

/// Walks the corner or edge permutation cycles of `cube` starting from the
/// relevant buffer in `buffers`, returning the memo sequence a blindfolded
/// solver would learn.
///
/// Pieces already solved (including the buffer's own, if untouched by the
/// scramble) contribute nothing. Once the buffer's own cycle closes, any
/// remaining unsolved pieces are walked as further cycles, each starting
/// fresh (not through the buffer) the way a solver picks up a "floating"
/// letter pair after closing out the buffer's cycle.
pub fn cycles(cube: &CubeState, piece: Piece, buffers: &Buffers) -> Vec<CycleStep> {
    let buffer_letter = match piece {
        Piece::Corners => buffers.corner,
        Piece::Edges => buffers.edge,
    };

    let mut result = Vec::new();
    cycle::walk_cycles(cube, piece, buffer_letter, |_start, raw| {
        result.extend(raw.iter().map(|&letter| CycleStep { letter }));
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubesim::{sticker_index, Face, FaceletCube};

    fn letters(steps: &[CycleStep]) -> Vec<char> {
        steps.iter().map(|s| s.letter).collect()
    }

    #[test]
    fn solved_cube_has_no_corner_or_edge_cycles() {
        let cube = crate::cube::solved();
        assert!(letters(&cycles(&cube, Piece::Corners, &Buffers::default())).is_empty());
        assert!(letters(&cycles(&cube, Piece::Edges, &Buffers::default())).is_empty());
    }

    /// Twists the buffer corner (UBL) in place -- U1/L1/B3 -- leaving
    /// everything else solved. A solver facing just this writes down the
    /// pair made of the buffer corner's *other* two letters.
    #[test]
    fn buffer_corner_twisted_in_place_pairs_its_other_two_letters() {
        let mut state = cubesim::solved_state(3);
        let u1 = sticker_index(3, Face::U, 1) as usize;
        let l1 = sticker_index(3, Face::L, 1) as usize;
        let b3 = sticker_index(3, Face::B, 3) as usize;
        state[u1] = Face::L;
        state[l1] = Face::B;
        state[b3] = Face::U;
        let cube = FaceletCube::from(state);

        let result = letters(&cycles(&cube, Piece::Corners, &Buffers::default()));
        assert_eq!(result, vec!['E', 'R']);
    }

    /// Flips the buffer edge (UB) in place -- U2/B2 -- leaving everything
    /// else solved. A solver facing just this writes down the buffer edge's
    /// other letter alone.
    #[test]
    fn buffer_edge_flipped_in_place_names_its_other_letter() {
        let mut state = cubesim::solved_state(3);
        let u2 = sticker_index(3, Face::U, 2) as usize;
        let b2 = sticker_index(3, Face::B, 2) as usize;
        state[u2] = Face::B;
        state[b2] = Face::U;
        let cube = FaceletCube::from(state);

        let result = letters(&cycles(&cube, Piece::Edges, &Buffers::default()));
        assert_eq!(result, vec!['Q']);
    }

    /// Across many random scrambles, every returned letter should be a
    /// distinct `'A'..='X'` piece letter other than the buffer itself (the
    /// buffer never needs to name itself). `next_letter` is a permutation
    /// of all 24 letters of a piece type (a sticker-level encoding of
    /// permutation *and* orientation, standard for Speffz cycle notation),
    /// so it decomposes into disjoint cycles covering all 24 letters: no
    /// letter -- not even a piece's second letter -- can appear twice, and
    /// at most 23 (every letter but the buffer's) can appear at all.
    #[test]
    fn cycles_never_repeat_or_include_the_buffer_letter() {
        for seed in 0..200 {
            let scramble = crate::cube::random_scramble(10 + seed % 15);
            let cube = crate::cube::scrambled(&scramble);

            for piece in [Piece::Corners, Piece::Edges] {
                let result = letters(&cycles(&cube, piece, &Buffers::default()));
                assert!(result.len() <= 23, "{:?}", result);
                assert!(!result.contains(&'A'), "{:?}", result);
                let mut sorted = result.clone();
                sorted.sort_unstable();
                sorted.dedup();
                assert_eq!(sorted.len(), result.len(), "{:?}", result);
                assert!(result.iter().all(|c| ('A'..='X').contains(c)));
            }
        }
    }
}
