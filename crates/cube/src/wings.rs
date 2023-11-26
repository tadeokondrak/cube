use crate::{sticker_cycle, EdgeSticker, Face, Handedness};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum WingSticker {
    Ubr = 0,
    Bur,
    Urf,
    Ruf,
    Ufl,
    Ful,
    Ulb,
    Lub,
    Luf,
    Ulf,
    Lfd,
    Fld,
    Ldb,
    Dlb,
    Lbu,
    Blu,
    Fur,
    Ufr,
    Frd,
    Rfd,
    Fdl,
    Dfl,
    Flu,
    Lfu,
    Rub,
    Urb,
    Rbd,
    Brd,
    Rdf,
    Drf,
    Rfu,
    Fru,
    Bul,
    Ubl,
    Bld,
    Lbd,
    Bdr,
    Dbr,
    Bru,
    Rbu,
    Dfr,
    Fdr,
    Drb,
    Rdb,
    Dbl,
    Bdl,
    Dlf,
    Ldf,
}

impl WingSticker {
    pub fn from_permutation_and_handedness_considering_orientation(
        permutation: EdgeSticker,
        handedness: Handedness,
    ) -> WingSticker {
        WingSticker::FROM_PERMUTATION_AND_HANDEDNESS[permutation.index()][handedness.index()]
    }

    pub fn from_permutation_and_handedness_ignoring_orientation(
        permutation: EdgeSticker,
        handedness: Handedness,
    ) -> WingSticker {
        WingSticker::from_index((permutation.index() * 2) + handedness.index())
    }

    /// Misnomer
    pub fn permutation(self) -> EdgeSticker {
        EdgeSticker::from_index(self.index() / 2)
    }

    pub fn edge_sticker_considering_handedness(self) -> EdgeSticker {
        match self.handedness() {
            Handedness::Left => self.permutation(),
            Handedness::Right => self.permutation().flipped(),
        }
    }

    pub fn handedness(self) -> Handedness {
        [Handedness::Left, Handedness::Right][self.index() % 2]
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn color(self) -> Face {
        match self.handedness() {
            Handedness::Left => self.permutation().color(),
            Handedness::Right => self.permutation().flipped().color(),
        }
    }

    pub fn lh(self) -> WingSticker {
        WingSticker::from_index(self.index() & !1)
    }

    pub fn rh(self) -> WingSticker {
        WingSticker::from_index(self.index() | 1)
    }

    pub fn from_index(index: usize) -> WingSticker {
        WingSticker::SOLVED[index]
    }

    pub fn with_handedness(self, handedness: Handedness) -> WingSticker {
        match handedness {
            Handedness::Left => self.lh(),
            Handedness::Right => self.rh(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wings {
    pub permutation: [EdgeSticker; 24],
}

impl Default for Wings {
    fn default() -> Self {
        Self::new()
    }
}

impl Wings {
    pub fn new() -> Wings {
        Wings {
            permutation: EdgeSticker::SOLVED,
        }
    }

    pub fn are_solved(&self) -> bool {
        self.permutation == EdgeSticker::SOLVED
    }

    pub fn cycle(&mut self, positions: &[EdgeSticker], count: u8) {
        sticker_cycle(&mut self.permutation, positions, count);
    }

    pub fn rotate_face(&mut self, face: crate::Face, count: u8) {
        self.cycle(&EdgeSticker::face_cycle(face), count);
        self.cycle(
            &EdgeSticker::flipped_cycle(EdgeSticker::face_cycle(face)),
            count,
        );
    }

    pub fn at(&self, position: WingSticker) -> WingSticker {
        let permutation = self.permutation[position.lh().permutation().index()];
        WingSticker::from_permutation_and_handedness_ignoring_orientation(
            permutation,
            position.handedness(),
        )
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
}

impl EdgeSticker {
    pub fn slice_wing_cycle_lh(face: Face) -> [EdgeSticker; 4] {
        let [a, b, c, d] = face.neighbors();
        [
            EdgeSticker::from_faces(a, a.cross_lh(face).unwrap()),
            EdgeSticker::from_faces(b, b.cross_lh(face).unwrap()),
            EdgeSticker::from_faces(c, c.cross_lh(face).unwrap()),
            EdgeSticker::from_faces(d, d.cross_lh(face).unwrap()),
        ]
    }

    pub fn slice_wing_cycle_rh(face: Face) -> [EdgeSticker; 4] {
        let [a, b, c, d] = face.neighbors();
        [
            EdgeSticker::from_faces(a, a.cross_rh(face).unwrap()),
            EdgeSticker::from_faces(b, b.cross_rh(face).unwrap()),
            EdgeSticker::from_faces(c, c.cross_rh(face).unwrap()),
            EdgeSticker::from_faces(d, d.cross_rh(face).unwrap()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lh_rh() {
        assert_eq!(WingSticker::Ufr.lh(), WingSticker::Fur);
        assert_eq!(WingSticker::Ufr.rh(), WingSticker::Ufr);

        assert_eq!(WingSticker::Ufl.lh(), WingSticker::Ufl);
        assert_eq!(WingSticker::Ufl.rh(), WingSticker::Ful);
    }
}
