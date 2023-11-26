use crate::Face;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corners {
    pub permutation: [CornerPermutation; 8],
    pub orientation: [CornerOrientation; 8],
}

impl Default for Corners {
    fn default() -> Self {
        Self::new()
    }
}

impl Corners {
    pub fn new() -> Corners {
        Corners {
            permutation: CornerPermutation::SOLVED,
            orientation: [CornerOrientation::Good; 8],
        }
    }

    pub fn from_coordinate(coordinate: u32) -> Corners {
        let permutation_coordinate =
            u16::try_from(coordinate / u32::from(Corners::NUM_ORIENTATION_COORDINATES)).unwrap();
        let orientation_coordinate =
            u16::try_from(coordinate % u32::from(Corners::NUM_ORIENTATION_COORDINATES)).unwrap();
        Corners::from_coordinates(permutation_coordinate, orientation_coordinate)
    }

    pub fn from_coordinates(permutation: u16, orientation: u16) -> Corners {
        let permutation = decode_permutation(permutation);
        let orientation = decode_orientation(orientation);
        Corners {
            permutation,
            orientation,
        }
    }

    pub fn at(&self, position: CornerSticker) -> CornerSticker {
        CornerSticker::from_permutation_and_orientation(
            self.permutation[position.permutation().index()],
            self.orientation[position.permutation().index()] + position.orientation(),
        )
    }

    pub fn cycle(&mut self, positions: &[CornerSticker], count: u8) {
        let old_permutation = self.permutation;
        let old_orientation = self.orientation;

        for i in 0..positions.len() {
            let j = (i + usize::from(count) + positions.len()) % positions.len();
            let from = positions[i];
            let to = positions[j];
            self.permutation[to.permutation().index()] =
                old_permutation[from.permutation().index()];
            self.orientation[to.permutation().index()] = old_orientation
                [from.permutation().index()]
                + from.orientation()
                + -to.orientation();
        }
    }

    pub fn rotate_face(&mut self, face: Face, count: u8) {
        self.cycle(&CornerSticker::face_cycle(face), count);
    }

    pub fn are_solved(&self) -> bool {
        self.permutation == CornerPermutation::SOLVED
            && self.orientation == [CornerOrientation::Good; 8]
    }

    pub const NUM_COORDINATES: u32 =
        Corners::NUM_PERMUTATION_COORDINATES as u32 * Corners::NUM_ORIENTATION_COORDINATES as u32;

    pub fn coordinate(&self) -> u32 {
        u32::from(self.permutation_coordinate()) * u32::from(Corners::NUM_ORIENTATION_COORDINATES)
            + u32::from(self.orientation_coordinate())
    }

    pub const NUM_ORIENTATION_COORDINATES: u16 = 3u16.pow(7);

    pub fn orientation_coordinate(&self) -> u16 {
        let sum = self.orientation[0..7]
            .iter()
            .enumerate()
            .map(|(i, co)| co.index() as u16 * 3u16.pow(i as u32))
            .sum();
        sum
    }

    pub const NUM_PERMUTATION_COORDINATES: u16 = FACTORIAL_U16[8];

    pub fn permutation_coordinate(&self) -> u16 {
        let mut x = 0;
        for i in (1..8).rev() {
            let mut s = 0;
            for j in (0..i).rev() {
                if self.permutation[j] > self.permutation[i] {
                    s += 1;
                }
            }
            x = (x + s) * i;
        }
        x as u16
    }
}

const FACTORIAL_U16: [u16; 9] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320];

fn decode_orientation(mut orientation: u16) -> [CornerOrientation; 8] {
    let mut sum = CornerOrientation::Good;
    let mut res = [CornerOrientation::Good; 8];
    for co in res.iter_mut().take(7) {
        *co = CornerOrientation::from_index(usize::from(orientation % 3));
        sum += *co;
        orientation /= 3;
    }
    res[7] = -sum;
    res
}

fn decode_permutation(mut permutation: u16) -> [CornerPermutation; 8] {
    let mut res = [CornerPermutation::Ubl; 8];

    let mut used = [false; 8];
    let mut order = [0; 8];

    for i in 0..8 {
        order[usize::from(i)] = permutation % (i + 1);
        permutation /= i + 1;
    }

    for i in (0..8).rev() {
        let mut k = 7;
        while used[k] {
            k -= 1;
        }
        while order[i] > 0 {
            order[i] -= 1;
            loop {
                k -= 1;
                if !used[k] {
                    break;
                }
            }
        }
        res[i] = CornerPermutation::from_index(k);
        used[k] = true;
    }

    res
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CornerPermutation {
    /// `UBL`, `BUL` or `LUB`
    Ubl = 0,
    /// `UBR`, `BUR` or `RUB`
    Ubr,
    /// `UFR`, `FUR` or `RUF`
    Ufr,
    /// `UFL`, `FUL` or `LUF`
    Ufl,
    /// `DFL`, `FDL` or `LDF`
    Dfl,
    /// `DFR`, `FDR` or `RDF`
    Dfr,
    /// `DBR`, `BDR` or `RDB`
    Dbr,
    /// `DBL`, `BDL` or `LDB`
    Dbl,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerDirection {
    TopLeft = 0,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl CornerDirection {
    pub const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CornerOrientation {
    Good = 0,
    BadCw = 1,
    BadCcw = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CornerSticker {
    /// `UBL`
    Ubl = 0,
    /// `UBR`
    Ubr,
    /// `UFR`
    Ufr,
    /// `UFL`
    Ufl,
    /// `LUB`
    Lub,
    /// `LUF`
    Luf,
    /// `LDF`
    Ldf,
    /// `LDB`
    Ldb,
    /// `FUL`
    Ful,
    /// `FUR`
    Fur,
    /// `FDR`
    Fdr,
    /// `FDL`
    Fdl,
    /// `RUF`
    Ruf,
    /// `RUB`
    Rub,
    /// `RDB`
    Rdb,
    /// `RDF`
    Rdf,
    /// `BUR`
    Bur,
    /// `BUL`
    Bul,
    /// `BDL`
    Bdl,
    /// `BDR`
    Bdr,
    /// `DFL`
    Dfl,
    /// `DFR`
    Dfr,
    /// `DBR`
    Dbr,
    /// `DBL`
    Dbl,
}

impl CornerSticker {
    pub fn from_permutation_and_orientation(
        permutation: CornerPermutation,
        orientation: CornerOrientation,
    ) -> CornerSticker {
        CornerPermutation::STICKERS[permutation.index()][orientation.index()]
    }

    pub fn color(self) -> Face {
        Face::from_index(self.index() / 4)
    }

    pub fn index(self) -> usize {
        u8::from(self).into()
    }

    pub const fn from_face_and_direction(face: Face, direction: CornerDirection) -> CornerSticker {
        CornerSticker::from_index(face.index() * 4 + direction.index())
    }

    pub const fn face_cycle(face: Face) -> [CornerSticker; 4] {
        pub const fn real(face: Face) -> [CornerSticker; 4] {
            [
                CornerSticker::from_face_and_direction(face, CornerDirection::TopLeft),
                CornerSticker::from_face_and_direction(face, CornerDirection::TopRight),
                CornerSticker::from_face_and_direction(face, CornerDirection::BottomRight),
                CornerSticker::from_face_and_direction(face, CornerDirection::BottomLeft),
            ]
        }
        const TABLE: [[CornerSticker; 4]; 6] = [
            real(Face::U),
            real(Face::L),
            real(Face::F),
            real(Face::R),
            real(Face::B),
            real(Face::D),
        ];
        TABLE[face.index()]
    }

    pub fn permutation(self) -> CornerPermutation {
        CornerSticker::PERMUTATIONS[self as usize]
    }

    pub fn orientation(self) -> CornerOrientation {
        CornerSticker::ORIENTATIONS[self as usize]
    }

    pub const fn from_index(index: usize) -> CornerSticker {
        CornerSticker::SOLVED[index]
    }

    pub fn from_faces(a: Face, b: Face, c: Face) -> CornerSticker {
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
        assert_ne!(a, b.opposite());
        assert_ne!(a, c.opposite());
        assert_ne!(b, c.opposite());

        CornerSticker::BY_FACES[a.index()][b.index()][c.index()]
    }
}

impl From<CornerSticker> for u8 {
    fn from(corner_sticker: CornerSticker) -> u8 {
        corner_sticker as u8
    }
}

impl CornerPermutation {
    pub fn index(self) -> usize {
        self as usize
    }

    fn from_index(num: usize) -> CornerPermutation {
        CornerPermutation::SOLVED[num]
    }
}

impl CornerOrientation {
    pub fn index(self) -> usize {
        match self {
            CornerOrientation::Good => 0,
            CornerOrientation::BadCw => 1,
            CornerOrientation::BadCcw => 2,
        }
    }

    pub fn from_index(index: usize) -> CornerOrientation {
        match index % 3 {
            0 => CornerOrientation::Good,
            1 => CornerOrientation::BadCw,
            2 => CornerOrientation::BadCcw,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<CornerOrientation> for CornerOrientation {
    type Output = CornerOrientation;

    fn add(self, rhs: CornerOrientation) -> CornerOrientation {
        CornerOrientation::from_index(self.index() + rhs.index())
    }
}

impl std::ops::AddAssign<CornerOrientation> for CornerOrientation {
    fn add_assign(&mut self, rhs: CornerOrientation) {
        *self = *self + rhs;
    }
}

impl std::ops::Neg for CornerOrientation {
    type Output = CornerOrientation;

    fn neg(self) -> CornerOrientation {
        match self {
            CornerOrientation::Good => CornerOrientation::Good,
            CornerOrientation::BadCw => CornerOrientation::BadCcw,
            CornerOrientation::BadCcw => CornerOrientation::BadCw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cube;

    use std::fmt::{Debug, Display};

    #[test]
    fn orientation_coordinate() {
        assert_eq!(Corners::new().permutation_coordinate(), 0);
        assert_eq!(Corners::new().orientation_coordinate(), 0);

        //let mut corners = Corners::new();
        //corners.cycle(&CornerSticker::face_cycle(Face::F), 1);
        //assert_eq!(corners.permutation_coordinate(), 360);
        //assert_eq!(corners.orientation_coordinate(), 450);
        //assert_eq!(
        //    Corners::from_coordinates(
        //        corners.permutation_coordinate(),
        //        corners.orientation_coordinate()
        //    ),
        //    corners
        //);

        for i in 0..1024 {
            let corners = Cube::new_random(2, i).corners;
            assert_eq!(
                Corners::from_coordinates(
                    corners.permutation_coordinate(),
                    corners.orientation_coordinate()
                ),
                corners
            );
        }
    }

    #[test]
    fn solve_eo() {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct Move {
            face: Face,
            n: u8,
        }

        impl Display for Move {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.face.fmt(f)?;
                match self.n {
                    1 => Ok(()),
                    2 => f.write_str("2"),
                    3 => f.write_str("'"),
                    _ => unreachable!(),
                }
            }
        }

        impl Debug for Move {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&self, f)
            }
        }

        impl Move {
            fn inverse(self) -> Move {
                Move {
                    face: self.face,
                    n: match self.n {
                        1 => 3,
                        2 => 2,
                        3 => 1,

                        _ => unreachable!("{}", self.n),
                    },
                }
            }
        }

        fn go(cube: &mut Cube, moves: &mut Vec<Move>, depth: u8) -> bool {
            let all_moves = Face::ALL
                .iter()
                .copied()
                .flat_map(|face| (1..3).map(move |n| Move { face, n }))
                .collect::<Vec<_>>();

            if cube.edges.orientation_coordinate() == 0 {
                return true;
            }

            if depth == 0 {
                return false;
            }

            for mov in all_moves {
                cube.rotate_face(mov.face, mov.n);
                moves.push(mov);
                if go(cube, moves, depth - 1) {
                    return true;
                } else {
                    moves.pop();
                    cube.rotate_face(mov.face, mov.inverse().n);
                }
            }
            false
        }

        fn apply_alg(cube: &mut Cube, alg: &str) {
            for mov in alg.split_whitespace() {
                let (face, n) = if let Some(face) = mov.strip_suffix('\'') {
                    (face, 3)
                } else if let Some(face) = mov.strip_suffix('2') {
                    (face, 2)
                } else {
                    (mov, 1)
                };
                let face = match face {
                    "U" => Face::U,
                    "L" => Face::L,
                    "F" => Face::F,
                    "R" => Face::R,
                    "B" => Face::B,
                    "D" => Face::D,
                    _ => panic!(),
                };
                cube.rotate_face(face, n);
            }
        }

        let mut cube = Cube::new_solved(3);
        apply_alg(
            &mut cube,
            "R2 U' L U' F2 D' F2 U L2 B2 U' B2 R' F D F2 L2 D F2",
        );

        for i in 0.. {
            let mut moves = Vec::new();
            if go(&mut cube, &mut moves, i) {
                eprintln!("EO in {i} moves: {moves:?}");
                break;
            }
        }
    }
}
