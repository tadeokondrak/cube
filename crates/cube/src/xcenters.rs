use crate::{sticker_cycle, CornerSticker, EdgeSticker, Face};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XCenters {
    pub permutation: [CornerSticker; 24],
}

impl Default for XCenters {
    fn default() -> Self {
        Self::new()
    }
}

impl XCenters {
    pub fn new() -> XCenters {
        XCenters {
            permutation: CornerSticker::SOLVED,
        }
    }

    pub fn cycle(&mut self, positions: &[CornerSticker], count: u8) {
        sticker_cycle(&mut self.permutation, positions, count);
    }

    pub fn rotate_face(&mut self, face: crate::Face, count: u8) {
        self.cycle(&CornerSticker::face_cycle(face), count);
    }

    pub fn at(&self, sticker: CornerSticker) -> CornerSticker {
        self.permutation[sticker.index()]
    }

    pub const MAX_PERMUTATION_COORDINATE: u128 = 620448401733239439359999; // fact(24) - 1

    pub fn permutation_coordinate(&self) -> u128 {
        self.permutation
            .iter()
            .copied()
            .enumerate()
            .skip(1)
            .rev()
            .fold(0, |coord, (i, permutation)| {
                (coord
                    + self.permutation[0..i - 1]
                        .iter()
                        .copied()
                        .rev()
                        .filter(|&other_permutation| other_permutation > permutation)
                        .count() as u128)
                    * i as u128
            })
    }

    pub fn are_solved(&self) -> bool {
        self.permutation
            .iter()
            .zip(EdgeSticker::SOLVED)
            .all(|(a, b)| a.color() == b.color())
    }

    pub fn are_solved_supercube(&self) -> bool {
        self.permutation == CornerSticker::SOLVED
    }
}

impl CornerSticker {
    pub fn slice_center_cycle_lh(face: Face) -> [CornerSticker; 4] {
        let [a, b, c, d] = face.neighbors();
        [
            CornerSticker::from_faces(a, face, a.cross_lh(face).unwrap()),
            CornerSticker::from_faces(b, face, b.cross_lh(face).unwrap()),
            CornerSticker::from_faces(c, face, c.cross_lh(face).unwrap()),
            CornerSticker::from_faces(d, face, d.cross_lh(face).unwrap()),
        ]
    }

    pub fn slice_center_cycle_rh(face: Face) -> [CornerSticker; 4] {
        let [a, b, c, d] = face.neighbors();
        [
            CornerSticker::from_faces(a, face, a.cross_rh(face).unwrap()),
            CornerSticker::from_faces(b, face, b.cross_rh(face).unwrap()),
            CornerSticker::from_faces(c, face, c.cross_rh(face).unwrap()),
            CornerSticker::from_faces(d, face, d.cross_rh(face).unwrap()),
        ]
    }
}
