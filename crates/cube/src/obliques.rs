use crate::{sticker_cycle, EdgeSticker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObliquesPair {
    pub left: Obliques,
    pub right: Obliques,
}
impl ObliquesPair {
    pub fn are_solved(&self) -> bool {
        self.left.are_solved() && self.right.are_solved()
    }

    pub fn are_solved_supercube(&self) -> bool {
        self.left.are_solved_supercube() && self.right.are_solved_supercube()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obliques {
    pub permutation: [EdgeSticker; 24],
}

impl Default for Obliques {
    fn default() -> Self {
        Self::new()
    }
}

impl Obliques {
    pub fn new() -> Obliques {
        Obliques {
            permutation: EdgeSticker::SOLVED,
        }
    }

    pub fn cycle(&mut self, positions: &[EdgeSticker], count: u8) {
        sticker_cycle(&mut self.permutation, positions, count)
    }

    pub fn at(&self, position: EdgeSticker) -> EdgeSticker {
        self.permutation[position.index()]
    }

    pub fn rotate_face(&mut self, face: crate::Face, count: u8) {
        self.cycle(&EdgeSticker::face_cycle(face), count);
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

    fn are_solved_supercube(&self) -> bool {
        self.permutation == EdgeSticker::SOLVED
    }
}
