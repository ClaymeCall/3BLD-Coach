//! 3-style memo generation: a freestyle, commutator-based alternative to
//! `memo`'s buffer-routed Old Pochmann/M2 cycle-breaking.
//!
//! Unlike `memo::cycles`, which always walks a piece type's permutation
//! starting from (and, for floating cycles, restarting fresh relative to)
//! the fixed buffer, [`generate`] classifies *every* disjoint cycle --
//! whether or not it touches the buffer -- into typed [`Target`]s, and
//! distinguishes ordinary swap-type commutator targets from pure
//! orientation-only ones (a corner twisted, or an edge flipped, in place
//! with nothing else scrambled) and from lone parity-style transpositions
//! (two different pieces swapped, with no third piece to chain through),
//! since real 3-style technique uses a different algorithm family for each
//! (confirmed against BLDDB, a 3BLD algorithm database that keeps "corner
//! twist"/"edge flip" as categories distinct from ordinary "3-style
//! corner"/"3-style edge" swaps).
//!
//! A "freestyle" target naming two non-buffer letters is a modeling
//! convenience -- an abstract permutation step -- not a claim that it's
//! always one single minimal real-world algorithm; see [`validate`] for how
//! correctness checking embraces this.

use std::collections::HashMap;

use crate::cube::CubeState;
use crate::cycle::{self, Piece};
use crate::lettering::{self, Buffers};

/// One target a 3-style solver memorizes for one piece type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Ordinary commutator target: whatever's at `from` goes to `to` (and
    /// the cycle's anchor -- the buffer, or a floating cycle's own restart
    /// letter -- inherits what was at `to`). Corners only.
    CornerSwap { from: char, to: char },
    /// A non-buffer corner twisted in place, everything else about it
    /// solved: `letters` are its other two Speffz letters, in walk order.
    CornerTwist { letters: [char; 2] },
    /// A lone leftover corner after pairing up an odd cycle's letters two at
    /// a time: this corner simply trades places with whichever corner
    /// anchors it (found at solve time), needing a parity-style algorithm
    /// -- real technique usually resolves this together with a matching
    /// [`Target::EdgeParity`], but nothing here requires exactly one of
    /// each.
    CornerParity { letter: char },
    /// Edge analogue of [`Target::CornerSwap`].
    EdgeSwap { from: char, to: char },
    /// A non-buffer edge flipped in place: `letter` is its other Speffz
    /// letter.
    EdgeFlip { letter: char },
    /// Edge analogue of [`Target::CornerParity`].
    EdgeParity { letter: char },
}

/// A full 3-style memo for one scramble: every corner target and every edge
/// target needed to resolve it.
pub struct ThreeStyleMemo {
    pub corners: Vec<Target>,
    pub edges: Vec<Target>,
}

/// Generates a 3-style memo for `cube` relative to `buffers`.
pub fn generate(cube: &CubeState, buffers: &Buffers) -> ThreeStyleMemo {
    ThreeStyleMemo {
        corners: targets_for(cube, Piece::Corners, buffers.corner),
        edges: targets_for(cube, Piece::Edges, buffers.edge),
    }
}

/// Computes one piece type's targets: every disjoint cycle in its
/// permutation, classified and chunked into typed targets.
///
/// A single physical corner swap couples all 3 of its stickers, but
/// `next_letter`'s color-matching only ever follows *one* sticker's chain
/// per step; a full corner-to-corner swap generally spans up to 3
/// independent "parallel tracks" (one per sticker axis), not just the one
/// a single cycle walk finds from a single starting letter (that single
/// track is all Old Pochmann needs, since its fixed buffer algorithm
/// physically carries the other 2 stickers of each involved corner along
/// for free -- but 3-style's freestyle simulation needs every track made
/// explicit to be able to validate an answer). So this keeps starting a
/// fresh single-cycle walk ([`cycle::walk_one_cycle`]) from any letter not
/// yet covered by a previous track -- not just letters whose whole
/// physical *slot* is untouched, which is what `cycle::walk_cycles`'s own
/// floating-cycle continuation checks -- until every non-fixed letter has
/// been accounted for.
fn targets_for(cube: &CubeState, piece: Piece, buffer_letter: char) -> Vec<Target> {
    let mut targets = Vec::new();
    let pc = cycle::PieceCycles::new(cube, piece);
    let mut covered = [false; 24];
    let index = |l: char| (l as u8 - b'A') as usize;

    let mut next_start = Some(buffer_letter);
    while let Some(start) = next_start {
        covered[index(start)] = true;
        cycle::walk_one_cycle(cube, piece, start, |raw| {
            for &l in raw {
                covered[index(l)] = true;
            }
            if is_pure_orientation(piece, start, raw) {
                targets.push(orientation_target(piece, raw));
            } else {
                let mut pairs = raw.chunks_exact(2);
                for pair in &mut pairs {
                    targets.push(swap_target(piece, pair[0], pair[1]));
                }
                if let [widow] = *pairs.remainder() {
                    targets.push(parity_target(piece, widow));
                }
            }
        });
        next_start = ('A'..='X').find(|&l| !covered[index(l)] && pc.next_letter(l) != l);
    }

    targets
}

/// Whether `raw` (the letters visited after `start`) never leaves `start`'s
/// own physical piece -- i.e. this cycle is a pure orientation change of a
/// single piece, not a permutation involving any other piece.
fn is_pure_orientation(piece: Piece, start: char, raw: &[char]) -> bool {
    let group: Vec<char> = match piece {
        Piece::Corners => lettering::corner_piece(start).to_vec(),
        Piece::Edges => lettering::edge_piece(start).to_vec(),
    };
    raw.iter().all(|l| group.contains(l))
}

/// Builds the orientation-only target for a cycle already confirmed pure by
/// [`is_pure_orientation`]: 2 letters for a twisted corner, 1 for a flipped
/// edge.
fn orientation_target(piece: Piece, raw: &[char]) -> Target {
    match piece {
        Piece::Corners => Target::CornerTwist { letters: [raw[0], raw[1]] },
        Piece::Edges => Target::EdgeFlip { letter: raw[0] },
    }
}

fn swap_target(piece: Piece, from: char, to: char) -> Target {
    match piece {
        Piece::Corners => Target::CornerSwap { from, to },
        Piece::Edges => Target::EdgeSwap { from, to },
    }
}

fn parity_target(piece: Piece, letter: char) -> Target {
    match piece {
        Piece::Corners => Target::CornerParity { letter },
        Piece::Edges => Target::EdgeParity { letter },
    }
}

/// Abstract permutation state: slot letter -> the identity (home letter) of
/// whichever sticker currently occupies it. Solved iff `perm[l] == l` for
/// every `l`.
type Perm = HashMap<char, char>;

fn seed_perm(cube: &CubeState, piece: Piece) -> Perm {
    let pc = cycle::PieceCycles::new(cube, piece);
    ('A'..='X').map(|l| (l, pc.next_letter(l))).collect()
}

/// Resolves a 3-element cycle step shared by `CornerSwap`, `CornerTwist`,
/// and `EdgeSwap`: whatever slot currently holds `from`'s identity (the
/// "anchor", found by scanning `perm` -- it's the buffer on a cycle's first
/// step, or a prior step's anchor otherwise) hands off to whatever `to`
/// currently points to, and `from`/`to` are both solved.
///
/// Only valid when a third, distinct slot (the anchor) exists -- i.e. this
/// is never the right primitive for a plain two-element transposition; see
/// [`resolve_pair`] for that.
fn apply_swap(perm: &mut Perm, from: char, to: char) -> Result<(), String> {
    if perm[&from] == from {
        return Err(format!("'{from}' is already solved"));
    }
    let anchor = *perm
        .iter()
        .find(|&(_, &v)| v == from)
        .map(|(k, _)| k)
        .ok_or_else(|| format!("nothing currently maps to '{from}'"))?;
    let next_in_chain = perm[&to];
    perm.insert(from, from);
    perm.insert(to, to);
    perm.insert(anchor, next_in_chain);
    Ok(())
}

/// Resolves a self-contained two-element cycle: whatever slot holds
/// `letter`'s identity is solved, and so is `letter`'s own slot. Used for
/// `EdgeFlip` and for each parity target -- each one is independently
/// self-contained, so any number of them (zero, one, or several) resolve
/// correctly regardless of order or of what the other piece type needs.
fn resolve_pair(perm: &mut Perm, letter: char) -> Result<(), String> {
    if perm[&letter] == letter {
        return Err(format!("'{letter}' is already solved"));
    }
    let anchor = *perm
        .iter()
        .find(|&(_, &v)| v == letter)
        .map(|(k, _)| k)
        .ok_or_else(|| format!("nothing currently maps to '{letter}'"))?;
    perm.insert(letter, letter);
    perm.insert(anchor, anchor);
    Ok(())
}

/// Applies one target to the single `Perm` it belongs to -- every variant
/// resolves via either [`apply_swap`] (a 3-element chain step: ordinary
/// swaps, and twists, since a twist's "3rd slot" is just whichever other
/// physical piece the chain happens to be passing through) or
/// [`resolve_pair`] (a self-contained 2-element closure: flips, and parity
/// targets, which apply identically -- the distinction between them is
/// purely how [`generate`] labels its own output for display, not a
/// difference in simulated effect).
fn apply_to(perm: &mut Perm, target: &Target) -> Result<(), String> {
    match *target {
        Target::CornerSwap { from, to } | Target::EdgeSwap { from, to } => apply_swap(perm, from, to),
        Target::CornerTwist { letters: [a, b] } => apply_swap(perm, a, b),
        Target::CornerParity { letter } | Target::EdgeParity { letter } | Target::EdgeFlip { letter } => {
            resolve_pair(perm, letter)
        }
    }
}

/// Checks whether `answer` (all targets for a single `piece` type) actually
/// resolves `cube`'s scramble back to solved for that piece type, relative
/// to `cube`'s own permutation -- not whether it matches the one
/// decomposition [`generate`] happens to produce. Any semantically valid
/// decomposition is accepted, which is what gives 3-style's freestyle
/// flexibility real meaning here: more than one answer can be correct for
/// the same scramble.
pub fn validate_piece(cube: &CubeState, piece: Piece, answer: &[Target]) -> Result<(), String> {
    let mut perm = seed_perm(cube, piece);
    for target in answer {
        apply_to(&mut perm, target)?;
    }
    if perm.iter().any(|(&slot, &occupant)| slot != occupant) {
        return Err("not fully solved".to_string());
    }
    Ok(())
}

/// Renders targets the way a solver would read them back: one token per
/// target (so mixed 1- and 2-letter targets display naturally), with a
/// parenthesized label on anything that isn't an ordinary swap. This is a
/// display-only convenience -- [`parse_tokens`] accepts the bare letters
/// alone, with no label.
pub fn display(targets: &[Target]) -> String {
    targets.iter().map(display_one).collect::<Vec<_>>().join(" ")
}

fn display_one(target: &Target) -> String {
    match *target {
        Target::CornerSwap { from, to } | Target::EdgeSwap { from, to } => format!("{from}{to}"),
        Target::CornerTwist { letters: [a, b] } => format!("{a}{b} (twist)"),
        Target::EdgeFlip { letter } => format!("{letter} (flip)"),
        Target::CornerParity { letter } | Target::EdgeParity { letter } => format!("{letter} (parity)"),
    }
}

/// Validates a full memo (both piece types) at once; see [`validate_piece`].
#[allow(dead_code)]
pub fn validate(cube: &CubeState, corner_answer: &[Target], edge_answer: &[Target]) -> Result<(), String> {
    validate_piece(cube, Piece::Corners, corner_answer).map_err(|e| format!("corners: {e}"))?;
    validate_piece(cube, Piece::Edges, edge_answer).map_err(|e| format!("edges: {e}"))?;
    Ok(())
}

/// Parses a whitespace-separated memo answer (e.g. `"DI CD"`) into targets
/// for one piece type, the way a solver would type back what they
/// memorized.
///
/// A token's length alone determines its shape: a lone letter is a parity
/// target ([`Target::CornerParity`] for corners; for edges, always parsed
/// as [`Target::EdgeFlip`] rather than [`Target::EdgeParity`] -- the two
/// apply identically during validation, so which variant represents a
/// *typed* lone letter doesn't affect correctness checking, only how
/// [`generate`]'s own output is labeled), and a 2-letter token is an
/// ordinary swap or, for corners, a twist -- real 3-style notation doesn't
/// mark a twist differently from an ordinary swap in writing, only in
/// recognition at execution time, so a 2-letter corner token is classified
/// as [`Target::CornerTwist`] when both letters name the same physical
/// corner, [`Target::CornerSwap`] otherwise.
pub fn parse_tokens(input: &str, piece: Piece) -> Result<Vec<Target>, String> {
    input.split_whitespace().map(|token| parse_token(piece, token)).collect()
}

fn parse_token(piece: Piece, token: &str) -> Result<Target, String> {
    let letters: Vec<char> = token.chars().collect();
    if letters.iter().any(|c| !('A'..='X').contains(c)) {
        return Err(format!("'{token}' is not a Speffz token ('A'..='X' letters only)"));
    }

    match (piece, letters.len()) {
        (Piece::Corners, 1) => Ok(Target::CornerParity { letter: letters[0] }),
        (Piece::Corners, 2) => {
            let (a, b) = (letters[0], letters[1]);
            if lettering::corner_piece(a).contains(&b) {
                Ok(Target::CornerTwist { letters: [a, b] })
            } else {
                Ok(Target::CornerSwap { from: a, to: b })
            }
        }
        (Piece::Corners, _) => Err(format!("'{token}' must be 1 or 2 letters for a corner target")),
        (Piece::Edges, 1) => Ok(Target::EdgeFlip { letter: letters[0] }),
        (Piece::Edges, 2) => Ok(Target::EdgeSwap { from: letters[0], to: letters[1] }),
        (Piece::Edges, _) => Err(format!("'{token}' must be 1 or 2 letters for an edge target")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubesim::{sticker_index, Face, FaceletCube};

    #[test]
    fn solved_cube_has_no_targets() {
        let cube = crate::cube::solved();
        let memo = generate(&cube, &Buffers::default());
        assert!(memo.corners.is_empty());
        assert!(memo.edges.is_empty());
    }

    /// Directly writes a 3-cycle UBR -> UFR -> UFL -> UBR (none of them the
    /// default corner buffer, UBL/'A') by assigning each corner's one or two
    /// non-shared stickers the color of the piece moving into its spot. A
    /// physical 3-corner swap couples all 3 of each corner's stickers, so
    /// this spans 3 independent tracks (one per sticker axis), not just 1 --
    /// `generate` must discover and resolve all of them, not only the one
    /// reachable from a single starting letter.
    #[test]
    fn simple_floating_corner_three_cycle_not_touching_buffer() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::B, 1) as usize] = Face::F; // UBR's B-sticker (letter Q)
        state[sticker_index(3, Face::R, 1) as usize] = Face::L; // UFR's R-sticker (letter M)
        state[sticker_index(3, Face::F, 1) as usize] = Face::B; // UFL's F-sticker (letter I)
        state[sticker_index(3, Face::L, 3) as usize] = Face::R; // UFL's L-sticker (letter Fc)
        let cube = FaceletCube::from(state);

        let memo = generate(&cube, &Buffers::default());
        assert!(memo.edges.is_empty());
        assert_eq!(memo.corners.len(), 3, "{:?}", memo.corners);
        assert!(memo.corners.iter().all(|t| matches!(t, Target::CornerSwap { .. })), "{:?}", memo.corners);
        assert_eq!(validate(&cube, &memo.corners, &memo.edges), Ok(()));
    }

    /// Twists a non-buffer corner (DFR) in place, everything else solved:
    /// classified as `CornerTwist`, not a bare `CornerSwap`, unlike
    /// `memo::cycles` which only emits raw letters.
    #[test]
    fn non_buffer_corner_twisted_in_place_is_classified_as_twist() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::R, 7) as usize] = Face::F; // DFR's R-sticker (letter P)
        state[sticker_index(3, Face::F, 9) as usize] = Face::D; // DFR's F-sticker (letter K)
        state[sticker_index(3, Face::D, 3) as usize] = Face::R; // DFR's D-sticker (letter V)
        let cube = FaceletCube::from(state);

        let memo = generate(&cube, &Buffers::default());
        assert_eq!(memo.corners, vec![Target::CornerTwist { letters: ['V', 'P'] }]);
        assert!(memo.edges.is_empty());
    }

    /// Flips a non-buffer edge (UF) in place, everything else solved:
    /// classified as `EdgeFlip`.
    #[test]
    fn non_buffer_edge_flipped_in_place_is_classified_as_flip() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::U, 8) as usize] = Face::F; // UF's U-sticker (letter C)
        state[sticker_index(3, Face::F, 2) as usize] = Face::U; // UF's F-sticker (letter I)
        let cube = FaceletCube::from(state);

        let memo = generate(&cube, &Buffers::default());
        assert_eq!(memo.edges, vec![Target::EdgeFlip { letter: 'I' }]);
        assert!(memo.corners.is_empty());
    }

    /// Combines the floating 3-cycle (UBR/UFR/UFL) with a separate
    /// non-buffer twisted corner (DFR): both target kinds appear.
    #[test]
    fn scramble_needing_both_a_swap_and_a_twist_target() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::B, 1) as usize] = Face::F;
        state[sticker_index(3, Face::R, 1) as usize] = Face::L;
        state[sticker_index(3, Face::F, 1) as usize] = Face::B;
        state[sticker_index(3, Face::L, 3) as usize] = Face::R;
        state[sticker_index(3, Face::R, 7) as usize] = Face::F;
        state[sticker_index(3, Face::F, 9) as usize] = Face::D;
        state[sticker_index(3, Face::D, 3) as usize] = Face::R;
        let cube = FaceletCube::from(state);

        let memo = generate(&cube, &Buffers::default());
        assert!(memo.corners.contains(&Target::CornerTwist { letters: ['V', 'P'] }), "{:?}", memo.corners);
        assert!(memo.corners.iter().any(|t| matches!(t, Target::CornerSwap { .. })), "{:?}", memo.corners);
        assert_eq!(validate(&cube, &memo.corners, &memo.edges), Ok(()));
    }

    /// Directly swaps two whole corners (UBR<->UFR) and two whole edges
    /// (UF<->UR). Each piece shares one face with its swap partner (corners
    /// share two: U and R), and each shared face contributes its own
    /// odd-length (single-letter) track: every track surfaces as its own
    /// lone parity target, never an ordinary swap.
    #[test]
    fn two_disjoint_transpositions_each_produce_a_parity_target() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::B, 1) as usize] = Face::F; // UBR's B-sticker (letter Q)
        state[sticker_index(3, Face::F, 3) as usize] = Face::B; // UFR's F-sticker (letter J)
        state[sticker_index(3, Face::F, 2) as usize] = Face::R; // UF's F-sticker (letter I)
        state[sticker_index(3, Face::R, 2) as usize] = Face::F; // UR's R-sticker (letter M)
        let cube = FaceletCube::from(state);

        let memo = generate(&cube, &Buffers::default());
        assert!(!memo.corners.is_empty());
        assert!(!memo.edges.is_empty());
        assert!(memo.corners.iter().all(|t| matches!(t, Target::CornerParity { .. })), "{:?}", memo.corners);
        assert!(memo.edges.iter().all(|t| matches!(t, Target::EdgeParity { .. })), "{:?}", memo.edges);
        assert_eq!(validate(&cube, &memo.corners, &memo.edges), Ok(()));
    }

    /// Two *separate* odd corner cycles (and so two independent
    /// `CornerParity` targets) on the same scramble, with edges left with
    /// none at all: a real, legally-scrambled state (this is exactly what a
    /// short random scramble produced during development), proving a piece
    /// type's parity targets don't need to come in cross-type pairs to be
    /// individually valid.
    #[test]
    fn multiple_independent_parity_targets_in_one_piece_type_both_validate() {
        let scramble = [
            cubesim::Move::R(cubesim::MoveVariant::Inverse),
            cubesim::Move::D(cubesim::MoveVariant::Double),
            cubesim::Move::R(cubesim::MoveVariant::Double),
            cubesim::Move::L(cubesim::MoveVariant::Double),
            cubesim::Move::B(cubesim::MoveVariant::Inverse),
            cubesim::Move::D(cubesim::MoveVariant::Inverse),
            cubesim::Move::B(cubesim::MoveVariant::Inverse),
            cubesim::Move::F(cubesim::MoveVariant::Double),
            cubesim::Move::D(cubesim::MoveVariant::Inverse),
            cubesim::Move::R(cubesim::MoveVariant::Inverse),
        ];
        let cube = crate::cube::scrambled(&scramble);

        let memo = generate(&cube, &Buffers::default());
        let corner_parities: Vec<char> = memo
            .corners
            .iter()
            .filter_map(|t| match t {
                Target::CornerParity { letter } => Some(*letter),
                _ => None,
            })
            .collect();
        assert!(corner_parities.len() >= 2, "{:?}", memo.corners);
        assert!(memo.edges.iter().all(|t| !matches!(t, Target::EdgeParity { .. })), "{:?}", memo.edges);
        assert_eq!(validate(&cube, &memo.corners, &memo.edges), Ok(()));
    }

    /// Across many random scrambles, `generate`'s own output always
    /// validates against the scramble it was generated from.
    #[test]
    fn generated_memo_always_validates_against_its_own_scramble() {
        for seed in 0..200 {
            let scramble = crate::cube::random_scramble(10 + seed % 15);
            let cube = crate::cube::scrambled(&scramble);
            let memo = generate(&cube, &Buffers::default());
            assert_eq!(
                validate(&cube, &memo.corners, &memo.edges),
                Ok(()),
                "scramble {:?}",
                crate::cube::to_wca_string(&scramble)
            );
        }
    }

    /// A single R turn produces a 4-corner and a 4-edge physical cycle, each
    /// spanning 3 (corners) or 2 (edges) independent sticker tracks, none
    /// touching the default buffers. Every corner track resolves as a
    /// `CornerSwap` plus a `CornerParity`; hand-building a *different* valid
    /// decomposition of just the first track (swapping the other two
    /// letters first instead, leaving the third as the residual, matching
    /// the general pattern already proven for a single track in
    /// `two_disjoint_transpositions_each_produce_a_parity_target`) and
    /// feeding it to `validate` directly proves correctness-checking is
    /// semantic, not a reimplementation of `generate`'s own walk order.
    #[test]
    fn an_alternative_valid_decomposition_is_also_accepted() {
        use cubesim::{Cube, Move, MoveVariant};
        let cube = crate::cube::solved().apply_moves(&[Move::R(MoveVariant::Standard)]);

        let memo = generate(&cube, &Buffers::default());
        assert_eq!(memo.corners.len(), 6, "{:?}", memo.corners);
        assert!(memo.corners.contains(&Target::CornerSwap { from: 'J', to: 'V' }));
        assert!(memo.corners.contains(&Target::CornerParity { letter: 'T' }));

        let mut alternative_corners = memo.corners.clone();
        let track_one = [
            Target::CornerSwap { from: 'J', to: 'V' },
            Target::CornerParity { letter: 'T' },
        ];
        alternative_corners.retain(|t| !track_one.contains(t));
        alternative_corners.push(Target::CornerSwap { from: 'V', to: 'T' });
        alternative_corners.push(Target::CornerParity { letter: 'J' });

        assert_ne!(alternative_corners, memo.corners);
        assert_eq!(validate(&cube, &alternative_corners, &memo.edges), Ok(()));
    }

    #[test]
    fn invalid_answer_is_rejected_with_a_specific_reason() {
        let cube = crate::cube::solved();
        let err = validate(&cube, &[Target::CornerSwap { from: 'B', to: 'C' }], &[]).unwrap_err();
        assert!(err.contains("'B' is already solved"), "{err}");
    }

    #[test]
    fn parse_tokens_classifies_same_piece_letters_as_twist_or_flip() {
        // D and I both name UFL (see lettering::tests::corner_piece_groups_the_three_stickers_of_ufl).
        assert_eq!(
            parse_tokens("DI", Piece::Corners),
            Ok(vec![Target::CornerTwist { letters: ['D', 'I'] }])
        );
        // C (UFR) and D (UFL) are different corners.
        assert_eq!(parse_tokens("CD", Piece::Corners), Ok(vec![Target::CornerSwap { from: 'C', to: 'D' }]));

        assert_eq!(parse_tokens("I", Piece::Edges), Ok(vec![Target::EdgeFlip { letter: 'I' }]));
        assert_eq!(parse_tokens("BM", Piece::Edges), Ok(vec![Target::EdgeSwap { from: 'B', to: 'M' }]));

        assert_eq!(
            parse_tokens("DI CD", Piece::Corners),
            Ok(vec![
                Target::CornerTwist { letters: ['D', 'I'] },
                Target::CornerSwap { from: 'C', to: 'D' },
            ])
        );
    }

    #[test]
    fn parse_tokens_rejects_malformed_tokens() {
        assert!(parse_tokens("ABC", Piece::Corners).is_err());
        assert!(parse_tokens("ABC", Piece::Edges).is_err());
        assert!(parse_tokens("A1", Piece::Corners).is_err());
        assert!(parse_tokens("Az", Piece::Edges).is_err());
    }

    #[test]
    fn parse_tokens_accepts_a_lone_corner_letter_as_parity() {
        assert_eq!(parse_tokens("G", Piece::Corners), Ok(vec![Target::CornerParity { letter: 'G' }]));
    }
}
