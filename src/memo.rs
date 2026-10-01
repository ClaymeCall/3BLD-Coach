//! Cycle-breaking: given a scrambled cube and a buffer piece, work out the
//! sequence of pieces visited -- and therefore the letter pairs to
//! memorize -- to solve that piece type with a buffer-based method (Old
//! Pochmann, M2, 3-style, ...).
//!
//! The core move is [`next_letter`]: given a sticker letter `t`, read the
//! colors currently sitting on `t`'s physical piece, find the (unique)
//! other physical piece on the cube with that same set of colors, and
//! return the letter of *that* piece's sticker whose home color matches the
//! color actually sitting at `t`. Starting from the buffer and repeatedly
//! applying this traces exactly the letter sequence a blindfolded solver
//! would memorize: each step names where the piece currently occupying the
//! previous letter's spot actually belongs.

use std::collections::HashSet;

use cubesim::{sticker_index, Cube, Face};

use crate::cube::CubeState;
use crate::lettering::{self, Buffers};

/// Piece type being solved, which determines which stickers matter and how
/// cycles are broken.
#[derive(Clone, Copy)]
pub enum Piece {
    Corners,
    Edges,
}

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
    let state = cube.state();
    let slots = distinct_pieces(piece);

    let slot_index = |letter: char| -> usize {
        slots
            .iter()
            .position(|group| group.contains(&letter))
            .expect("every 'A'..='X' letter belongs to exactly one piece")
    };

    let color_at = |letter: char| -> Face {
        let (face, idx) = sticker(piece, letter);
        state[sticker_index(3, face, idx) as usize]
    };

    let home_face = |letter: char| -> Face { sticker(piece, letter).0 };

    let next_letter = |t: char| -> char {
        let slot = &slots[slot_index(t)];
        let colors: HashSet<Face> = slot.iter().map(|&l| color_at(l)).collect();
        let target = slots
            .iter()
            .find(|group| group.iter().map(|&l| home_face(l)).collect::<HashSet<_>>() == colors)
            .expect("a piece's current colors always match some physical piece on a valid cube");
        let c = color_at(t);
        *target
            .iter()
            .find(|&&l| home_face(l) == c)
            .expect("a piece's colors always match one of its own stickers")
    };

    let buffer_letter = match piece {
        Piece::Corners => buffers.corner,
        Piece::Edges => buffers.edge,
    };

    let mut visited = vec![false; slots.len()];
    let mut result = Vec::new();
    let mut entry = Some(buffer_letter);

    while let Some(start) = entry {
        visited[slot_index(start)] = true;
        let mut t = start;
        loop {
            let next = next_letter(t);
            if next == start {
                break;
            }
            result.push(CycleStep { letter: next });
            visited[slot_index(next)] = true;
            t = next;
        }

        entry = ('A'..='X')
            .filter(|&l| !visited[slot_index(l)])
            .find(|&l| next_letter(l) != l);
    }

    result
}

fn sticker(piece: Piece, letter: char) -> (Face, i32) {
    match piece {
        Piece::Corners => lettering::corner_sticker(letter),
        Piece::Edges => lettering::edge_sticker(letter),
    }
}

/// The distinct physical pieces of `piece`'s type, as groups of their
/// letters.
fn distinct_pieces(piece: Piece) -> Vec<Vec<char>> {
    let mut slots: Vec<Vec<char>> = Vec::new();
    for letter in 'A'..='X' {
        let group: Vec<char> = match piece {
            Piece::Corners => lettering::corner_piece(letter).to_vec(),
            Piece::Edges => lettering::edge_piece(letter).to_vec(),
        };
        if !slots.contains(&group) {
            slots.push(group);
        }
    }
    slots
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubesim::FaceletCube;

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
