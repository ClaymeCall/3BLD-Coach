//! Shared permutation-query and cycle-walking primitives for piece-type
//! letter schemes, used by both `memo` (Old Pochmann/M2) and `threestyle`.
//!
//! The core move is [`PieceCycles::next_letter`]: given a sticker letter
//! `t`, read the colors currently sitting on `t`'s physical piece, find the
//! (unique) other physical piece on the cube with that same set of colors,
//! and return the letter of *that* piece's sticker whose home color matches
//! the color actually sitting at `t`. Starting anywhere and repeatedly
//! applying this traces exactly the letter sequence a blindfolded solver
//! would memorize: each step names where the piece currently occupying the
//! previous letter's spot actually belongs. [`walk_cycles`] generalizes this
//! into visiting every disjoint cycle of a piece type's permutation.

use std::collections::HashSet;

use cubesim::{sticker_index, Cube, Face};

use crate::cube::CubeState;
use crate::lettering;

/// Piece type being solved, which determines which stickers matter and how
/// cycles are broken.
#[derive(Clone, Copy)]
pub enum Piece {
    Corners,
    Edges,
}

/// The distinct physical pieces of `piece`'s type, as groups of their
/// letters.
pub(crate) fn distinct_pieces(piece: Piece) -> Vec<Vec<char>> {
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

pub(crate) fn sticker(piece: Piece, letter: char) -> (Face, i32) {
    match piece {
        Piece::Corners => lettering::corner_sticker(letter),
        Piece::Edges => lettering::edge_sticker(letter),
    }
}

/// Queries a cube's permutation of one piece type, letter by letter.
pub struct PieceCycles {
    piece: Piece,
    state: Vec<Face>,
    slots: Vec<Vec<char>>,
}

impl PieceCycles {
    pub fn new(cube: &CubeState, piece: Piece) -> Self {
        Self { piece, state: cube.state(), slots: distinct_pieces(piece) }
    }

    fn slot_index(&self, letter: char) -> usize {
        self.slots
            .iter()
            .position(|group| group.contains(&letter))
            .expect("every 'A'..='X' letter belongs to exactly one piece")
    }

    fn color_at(&self, letter: char) -> Face {
        let (face, idx) = sticker(self.piece, letter);
        self.state[sticker_index(3, face, idx) as usize]
    }

    fn home_face(&self, letter: char) -> Face {
        sticker(self.piece, letter).0
    }

    /// The letter of whichever sticker currently physically occupies slot
    /// `t`.
    pub fn next_letter(&self, t: char) -> char {
        let slot = &self.slots[self.slot_index(t)];
        let colors: HashSet<Face> = slot.iter().map(|&l| self.color_at(l)).collect();
        let target = self
            .slots
            .iter()
            .find(|group| group.iter().map(|&l| self.home_face(l)).collect::<HashSet<_>>() == colors)
            .expect("a piece's current colors always match some physical piece on a valid cube");
        let c = self.color_at(t);
        *target
            .iter()
            .find(|&&l| self.home_face(l) == c)
            .expect("a piece's colors always match one of its own stickers")
    }

    fn is_fixed(&self, t: char) -> bool {
        self.next_letter(t) == t
    }

    fn slot_count(&self) -> usize {
        self.slots.len()
    }
}

/// Walks a single cycle of `piece`'s permutation on `cube`, starting at
/// `start_letter`, calling `on_raw` once with the letters visited after
/// `start_letter` (never including it) if the cycle is non-trivial. Unlike
/// [`walk_cycles`], never continues into further floating cycles -- callers
/// that need to discover every disjoint cycle (not just ones reachable via
/// per-physical-slot continuation from one starting point) drive that loop
/// themselves.
pub fn walk_one_cycle(cube: &CubeState, piece: Piece, start_letter: char, mut on_raw: impl FnMut(&[char])) {
    let pc = PieceCycles::new(cube, piece);
    let mut raw = Vec::new();
    let mut t = start_letter;
    loop {
        let next = pc.next_letter(t);
        if next == start_letter {
            break;
        }
        raw.push(next);
        t = next;
    }
    if !raw.is_empty() {
        on_raw(&raw);
    }
}

/// Walks every disjoint cycle of `piece`'s permutation on `cube`: starts at
/// `start_letter` first, then any remaining unvisited, non-fixed letter in
/// alphabetical order (the "buffer first, then floating cycles" order both
/// Old Pochmann and 3-style use). Calls `on_cycle(start, raw)` once per
/// non-trivial cycle, `raw` being the letters visited after `start` in walk
/// order (never including `start` itself).
pub fn walk_cycles(cube: &CubeState, piece: Piece, start_letter: char, mut on_cycle: impl FnMut(char, &[char])) {
    let pc = PieceCycles::new(cube, piece);
    let mut visited = vec![false; pc.slot_count()];
    let mut entry = Some(start_letter);

    while let Some(start) = entry {
        visited[pc.slot_index(start)] = true;
        let mut t = start;
        let mut raw = Vec::new();
        loop {
            let next = pc.next_letter(t);
            if next == start {
                break;
            }
            raw.push(next);
            visited[pc.slot_index(next)] = true;
            t = next;
        }
        if !raw.is_empty() {
            on_cycle(start, &raw);
        }

        entry = ('A'..='X').filter(|&l| !visited[pc.slot_index(l)]).find(|&l| !pc.is_fixed(l));
    }
}
