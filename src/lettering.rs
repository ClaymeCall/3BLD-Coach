//! Speffz-style letter scheme for naming stickers, and the buffer pieces a
//! blindfolded solve is anchored to.
//!
//! Speffz assigns each of the 24 corner stickers a letter A-X, and
//! separately assigns each of the 24 edge stickers a letter A-X, going face
//! by face in the order U, L, F, R, B, D and, within each face, clockwise
//! (as viewed with that face pointing at you) starting from the sticker
//! nearest the "upper-left" of the standard net layout. That makes UBL's
//! U-facing sticker corner letter A, and UB's U-facing sticker edge letter
//! A -- the conventional Old Pochmann / M2 buffer defaults below.
//!
//! `cubesim`'s facelet layout was confirmed empirically (not just from
//! docs): applying each single face turn to a solved cube and observing
//! which 3-sticker row/column of each neighboring face changes pins down,
//! per face, which row is U-adjacent vs. D-adjacent and which column is
//! adjacent to which neighbor. That gives the row-major layout assumed
//! here: position 1 is top-left, 3 top-right, 7 bottom-left, 9 bottom-right
//! (corners), and 2/4/6/8 are top/left/right/bottom (edges), matching
//! `cubesim::sticker_index`'s 1-based `index` addressing.

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

/// Position, within a face's 3x3 grid of facelet indices (1-9, matching
/// `cubesim::sticker_index`), of the four corner stickers in clockwise
/// order starting top-left, and of the four edge stickers in the same
/// clockwise order.
#[allow(dead_code)]
const CORNER_POSITIONS: [i32; 4] = [1, 3, 9, 7];
#[allow(dead_code)]
const EDGE_POSITIONS: [i32; 4] = [2, 6, 8, 4];

/// Faces in Speffz lettering order: each face contributes 4 consecutive
/// letters to the corner scheme, and 4 consecutive letters to the (separate)
/// edge scheme.
#[allow(dead_code)]
const FACE_ORDER: [Face; 6] = [Face::U, Face::L, Face::F, Face::R, Face::B, Face::D];

#[allow(dead_code)]
fn face_rank(face: Face) -> usize {
    FACE_ORDER
        .iter()
        .position(|&f| f == face)
        .unwrap_or_else(|| panic!("{face} has no Speffz lettering"))
}

/// Maps a sticker -- identified by the face it's on and its 1-based facelet
/// index within that face, matching `cubesim::sticker_index`'s addressing --
/// to its Speffz letter.
///
/// Panics if `facelet_index` is out of range `1..=9`, or names the center
/// (5), which has no Speffz letter.
#[allow(dead_code)]
pub fn speffz_letter(face: Face, facelet_index: i32) -> char {
    let offset = CORNER_POSITIONS
        .iter()
        .chain(EDGE_POSITIONS.iter())
        .position(|&p| p == facelet_index)
        .unwrap_or_else(|| {
            panic!("facelet index {facelet_index} is not a corner or edge sticker")
        })
        % 4;

    let letter = (face_rank(face) * 4 + offset) as u8;
    (b'A' + letter) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_buffer_default_matches_ubl() {
        // UBL's U-facing sticker is the conventional corner buffer, 'A'.
        assert_eq!(speffz_letter(Face::U, 1), Buffers::default().corner);
    }

    #[test]
    fn edge_buffer_default_matches_ub() {
        // UB's U-facing sticker is the conventional edge buffer, 'A'.
        assert_eq!(speffz_letter(Face::U, 2), Buffers::default().edge);
    }

    #[test]
    fn corner_letters_follow_face_order_clockwise_from_top_left() {
        assert_eq!(speffz_letter(Face::U, 1), 'A'); // UBL
        assert_eq!(speffz_letter(Face::U, 3), 'B'); // UBR
        assert_eq!(speffz_letter(Face::U, 9), 'C'); // UFR
        assert_eq!(speffz_letter(Face::U, 7), 'D'); // UFL
        assert_eq!(speffz_letter(Face::L, 1), 'E'); // LUB
        assert_eq!(speffz_letter(Face::D, 7), 'X'); // DBL
    }

    #[test]
    fn edge_letters_follow_face_order_clockwise_from_top() {
        assert_eq!(speffz_letter(Face::U, 2), 'A'); // UB
        assert_eq!(speffz_letter(Face::U, 6), 'B'); // UR
        assert_eq!(speffz_letter(Face::U, 8), 'C'); // UF
        assert_eq!(speffz_letter(Face::U, 4), 'D'); // UL
        assert_eq!(speffz_letter(Face::L, 2), 'E'); // LU
        assert_eq!(speffz_letter(Face::D, 4), 'X'); // DL
    }

    #[test]
    fn a_physical_corner_s_three_stickers_get_distinct_letters() {
        // The UFL corner: U-facing, L-facing, and F-facing stickers.
        let u_facing = speffz_letter(Face::U, 7);
        let l_facing = speffz_letter(Face::L, 3);
        let f_facing = speffz_letter(Face::F, 1);
        assert_eq!((u_facing, l_facing, f_facing), ('D', 'F', 'I'));
    }

    #[test]
    fn a_physical_edge_s_two_stickers_get_distinct_letters() {
        // The UF edge: U-facing and F-facing stickers.
        let u_facing = speffz_letter(Face::U, 8);
        let f_facing = speffz_letter(Face::F, 2);
        assert_eq!((u_facing, f_facing), ('C', 'I'));
    }

    #[test]
    fn corner_letters_are_a_bijection_onto_a_through_x() {
        let mut letters: Vec<char> = FACE_ORDER
            .iter()
            .flat_map(|&face| CORNER_POSITIONS.iter().map(move |&p| speffz_letter(face, p)))
            .collect();
        letters.sort_unstable();
        assert_eq!(letters, ('A'..='X').collect::<Vec<_>>());
    }

    #[test]
    fn edge_letters_are_a_bijection_onto_a_through_x() {
        let mut letters: Vec<char> = FACE_ORDER
            .iter()
            .flat_map(|&face| EDGE_POSITIONS.iter().map(move |&p| speffz_letter(face, p)))
            .collect();
        letters.sort_unstable();
        assert_eq!(letters, ('A'..='X').collect::<Vec<_>>());
    }

    #[test]
    #[should_panic(expected = "not a corner or edge sticker")]
    fn center_facelet_has_no_letter() {
        speffz_letter(Face::U, 5);
    }

    #[test]
    #[should_panic(expected = "no Speffz lettering")]
    fn masked_face_has_no_letter() {
        speffz_letter(Face::X, 1);
    }
}
