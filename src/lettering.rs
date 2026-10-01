//! Speffz-style letter scheme for naming stickers, and the buffer pieces a
//! blindfolded solve is anchored to.
//!
//! Not yet implemented: translating a `cube::CubeState`'s stickers into
//! Speffz letters requires mapping `cubesim`'s `sticker_index` layout onto
//! the standard A-X lettering per face. That mapping is still TODO.

use cubesim::Face;

/// The fixed sticker a solver looks at first for each piece type; cycles are
/// broken relative to these.
#[allow(dead_code)]
pub struct Buffers {
    pub corner: char,
    pub edge: char,
}

impl Default for Buffers {
    fn default() -> Self {
        // UBL corner, UB edge: the defaults assumed by most beginner
        // buffer-based methods (Old Pochmann, M2) that this project targets
        // first.
        Self { corner: 'A', edge: 'A' }
    }
}

/// Maps a sticker -- identified by the face it's on and its facelet index
/// within that face, matching `cubesim::sticker_index`'s addressing -- to
/// its Speffz letter.
#[allow(dead_code, unused_variables)]
pub fn speffz_letter(face: Face, facelet_index: i32) -> char {
    todo!("map cubesim's sticker layout onto the Speffz A-X scheme")
}
