use crate::{Cube, Face, RotatedCube};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Edges {
    pub permutation: [EdgePermutation; 12],
    pub orientation: [EdgeOrientation; 12],
}

impl Default for Edges {
    fn default() -> Self {
        Self::new()
    }
}

impl Edges {
    pub fn new() -> Edges {
        Edges {
            permutation: EdgePermutation::SOLVED,
            orientation: [EdgeOrientation::Good; 12],
        }
    }

    pub fn at(&self, position: EdgeSticker) -> EdgeSticker {
        EdgeSticker::from_permutation_and_orientation(
            self.permutation[position.permutation().index()],
            self.orientation[position.permutation().index()] ^ position.orientation(),
        )
    }

    pub fn cycle(&mut self, positions: &[EdgeSticker], count: u8) {
        let old_permutation = self.permutation;
        let old_orientation = self.orientation;

        for i in 0..positions.len() {
            let j = (i + usize::from(count) + positions.len()) % positions.len();
            let from = positions[i];
            let to = positions[j];
            self.permutation[to.permutation().index()] =
                old_permutation[from.permutation().index()];
            self.orientation[to.permutation().index()] =
                old_orientation[from.permutation().index()] ^ from.orientation() ^ to.orientation();
        }
    }

    pub fn rotate_face(&mut self, face: crate::Face, count: u8) {
        self.cycle(&EdgeSticker::face_cycle(face), count);
    }

    pub const MAX_ORIENTATION_COORDINATE: u16 = 2047; // 2 ** 11 - 1

    pub fn orientation_coordinate(&self) -> u16 {
        self.orientation[0..11]
            .iter()
            .enumerate()
            .map(|(i, eo)| eo.index() as u16 * 2u16.pow(i as u32))
            .sum()
    }

    pub const MAX_PERMUTATION_COORDINATE: u32 = 479001599; // fact(12) - 1

    pub fn permutation_coordinate(&self) -> u32 {
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
                        .count() as u32)
                    * i as u32
            })
    }

    pub fn are_solved(&self) -> bool {
        self.permutation == EdgePermutation::SOLVED
            && self.orientation == [EdgeOrientation::Good; 12]
    }
}

impl EdgeDirection {
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl EdgeSticker {
    pub const fn from_face_and_direction(face: Face, direction: EdgeDirection) -> EdgeSticker {
        EdgeSticker::from_index(face.index() * 4 + direction.index())
    }

    pub const fn with_orientation(self, orientation: EdgeOrientation) -> EdgeSticker {
        EdgeSticker::from_permutation_and_orientation(self.permutation(), orientation)
    }

    pub const fn face_cycle(face: Face) -> [EdgeSticker; 4] {
        const fn real(face: Face) -> [EdgeSticker; 4] {
            [
                EdgeSticker::from_face_and_direction(face, EdgeDirection::Top),
                EdgeSticker::from_face_and_direction(face, EdgeDirection::Right),
                EdgeSticker::from_face_and_direction(face, EdgeDirection::Bottom),
                EdgeSticker::from_face_and_direction(face, EdgeDirection::Left),
            ]
        }
        const TABLE: [[EdgeSticker; 4]; 6] = [
            real(Face::U),
            real(Face::L),
            real(Face::F),
            real(Face::R),
            real(Face::B),
            real(Face::D),
        ];
        TABLE[face.index()]
    }

    pub const fn flipped_cycle(cycle: [EdgeSticker; 4]) -> [EdgeSticker; 4] {
        [
            cycle[0].flipped(),
            cycle[1].flipped(),
            cycle[2].flipped(),
            cycle[3].flipped(),
        ]
    }

    pub const fn slice_center_cycle(face: Face) -> [EdgeSticker; 4] {
        const fn real(face: Face) -> [EdgeSticker; 4] {
            EdgeSticker::flipped_cycle(EdgeSticker::face_cycle(face))
        }
        const TABLE: [[EdgeSticker; 4]; 6] = [
            real(Face::U),
            real(Face::L),
            real(Face::F),
            real(Face::R),
            real(Face::B),
            real(Face::D),
        ];
        TABLE[face.index()]
    }

    pub const fn from_permutation_and_orientation(
        permutation: EdgePermutation,
        orientation: EdgeOrientation,
    ) -> EdgeSticker {
        EdgePermutation::STICKERS[permutation.index()][orientation.index()]
    }

    pub fn color(self) -> Face {
        Face::from_index(self.index() / 4)
    }

    pub const fn flipped(&self) -> EdgeSticker {
        self.with_orientation(self.orientation().flipped())
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn from_index(index: usize) -> EdgeSticker {
        EdgeSticker::SOLVED[index]
    }

    pub const fn permutation(self) -> EdgePermutation {
        EdgeSticker::PERMUTATIONS[self.index()]
    }

    pub const fn orientation(self) -> EdgeOrientation {
        EdgeSticker::ORIENTATIONS[self.index()]
    }

    pub const fn from_faces(a: Face, b: Face) -> EdgeSticker {
        if a.index() == b.index() || a.index() == b.opposite().index() {
            panic!("from_faces called with invalid arguments");
        }
        EdgeSticker::BY_FACES[a.index()][b.index()]
    }
}

/// Edge orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i8)]
pub enum EdgeOrientation {
    Good = 1,
    Bad = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EdgeDirection {
    Top = 0,
    Right,
    Bottom,
    Left,
}

impl From<EdgeSticker> for u8 {
    fn from(edge_sticker: EdgeSticker) -> u8 {
        edge_sticker as u8
    }
}

impl EdgePermutation {
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl EdgeOrientation {
    pub const fn index(self) -> usize {
        match self {
            EdgeOrientation::Good => 0,
            EdgeOrientation::Bad => 1,
        }
    }

    pub fn from_index(index: usize) -> EdgeOrientation {
        match index % 2 {
            0 => EdgeOrientation::Good,
            1 => EdgeOrientation::Bad,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Not for EdgeOrientation {
    type Output = EdgeOrientation;

    fn not(self) -> EdgeOrientation {
        self.flipped()
    }
}

impl EdgeOrientation {
    const fn flipped(self) -> EdgeOrientation {
        match self {
            EdgeOrientation::Good => EdgeOrientation::Bad,
            EdgeOrientation::Bad => EdgeOrientation::Good,
        }
    }
}

impl std::ops::BitXor for EdgeOrientation {
    type Output = EdgeOrientation;

    fn bitxor(self, other: EdgeOrientation) -> EdgeOrientation {
        if self == other {
            EdgeOrientation::Good
        } else {
            EdgeOrientation::Bad
        }
    }
}

impl std::ops::BitXorAssign for EdgeOrientation {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

/// Edge permutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EdgePermutation {
    /// `UB` or `BU`
    Ub = 0,
    /// `UR` or `RU`
    Ur,
    /// `UF` or `FU`
    Uf,
    /// `UL` or `LU`
    Ul,
    /// `FR` or `RF`
    Fr,
    /// `FL` or `LF`
    Fl,
    /// `BL` or `LB`
    Bl,
    /// `BR` or `RB`
    Br,
    /// `DF` or `FD`
    Df,
    /// `DR` or `RD`
    Dr,
    /// `DB` or `BD`
    Db,
    /// `DL` or `LD`
    Dl,
}

/// Edge permutation and orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EdgeSticker {
    /// `UB`
    Ub = 0,
    /// `UR`
    Ur,
    /// `UF`
    Uf,
    /// `UL`
    Ul,
    /// `LU`
    Lu,
    /// `LF`
    Lf,
    /// `LD`
    Ld,
    /// `LB`
    Lb,
    /// `FU`
    Fu,
    /// `FR`
    Fr,
    /// `FD`
    Fd,
    /// `FL`
    Fl,
    /// `RU`
    Ru,
    /// `RB`
    Rb,
    /// `RD`
    Rd,
    /// `RF`
    Rf,
    /// `BU`
    Bu,
    /// `BL`
    Bl,
    /// `BD`
    Bd,
    /// `BR`
    Br,
    /// `DF`
    Df,
    /// `DR`
    Dr,
    /// `DB`
    Db,
    /// `DL`
    Dl,
}

impl EdgeSticker {
    pub fn xyz(self) -> (u8, u8, u8) {
        match self {
            EdgeSticker::Ub => (0, 2, 0),
            EdgeSticker::Ur => (0, 1, 0),
            EdgeSticker::Uf => (0, 0, 0),
            EdgeSticker::Ul => (0, 3, 0),
            EdgeSticker::Lu => (3, 0, 1),
            EdgeSticker::Lf => (0, 0, 1),
            EdgeSticker::Ld => (1, 0, 1),
            EdgeSticker::Lb => (0, 2, 3),
            EdgeSticker::Fu => (3, 0, 2),
            EdgeSticker::Fr => (0, 1, 1),
            EdgeSticker::Fd => (1, 0, 0),
            EdgeSticker::Fl => (0, 3, 3),
            EdgeSticker::Ru => (3, 0, 3),
            EdgeSticker::Rb => (0, 2, 1),
            EdgeSticker::Rd => (1, 0, 3),
            EdgeSticker::Rf => (0, 0, 3),
            EdgeSticker::Bu => (3, 0, 0),
            EdgeSticker::Bl => (0, 3, 1),
            EdgeSticker::Bd => (1, 0, 2),
            EdgeSticker::Br => (0, 1, 3),
            EdgeSticker::Df => (0, 0, 2),
            EdgeSticker::Dr => (0, 1, 2),
            EdgeSticker::Db => (2, 0, 0),
            EdgeSticker::Dl => (0, 3, 2),
        }
    }
}

#[allow(dead_code)]
fn find_xyz_for(orientation: EdgeSticker) -> (u8, u8, u8) {
    let mut all_orientations = Vec::new();
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let mut cube = Cube::new_solved(3);
                let mut cube = RotatedCube::new(&mut cube);
                cube.rotate(Face::R, 0..3, x);
                cube.rotate(Face::U, 0..3, y);
                cube.rotate(Face::F, 0..3, z);
                if orientation == cube.orientation {
                    all_orientations.push((x, y, z));
                }
            }
        }
    }
    all_orientations.sort_by_key(|&(x, y, z)| {
        (
            [x, y, z].into_iter().filter(|&x| x != 0).count(),
            x != 0,
            y != 0,
            z != 0,
        )
    });
    eprintln!("EdgeSticker::{orientation:?} => {:?},", all_orientations[0]);
    all_orientations[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_coordinate() {
        assert_eq!(Edges::new().orientation_coordinate(), 0);
        assert_eq!(Edges::new().permutation_coordinate(), 0);

        let mut edges = Edges::new();
        edges.cycle(&EdgeSticker::face_cycle(Face::F), 1);
        assert_eq!(edges.orientation_coordinate(), 308);
        assert_eq!(edges.permutation_coordinate(), 126024);
    }

    #[test]
    fn edgesticker_xyz() {
        for sticker in EdgeSticker::SOLVED {
            let mut cube = Cube::new_solved(3);
            let mut cube = RotatedCube::new(&mut cube);
            let (x, y, z) = sticker.xyz();
            cube.rotate(Face::R, 0..3, x);
            cube.rotate(Face::U, 0..3, y);
            cube.rotate(Face::F, 0..3, z);
            assert_eq!(sticker, cube.orientation);
        }
    }
}
