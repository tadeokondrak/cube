use super::CornerPermutation;
use crate::{CornerOrientation, CornerSticker, Face};

/// CornersFixed for a fixed-corner representation cube.
///
/// The DBL piece is fixed so that only U, F and R moves are possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CornersFixed {
    pub permutation: [CornerPermutationFixed; 7],
    pub orientation: [CornerOrientation; 7],
}

/// Corner permutation for a fixed-corner representation cube.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CornerPermutationFixed {
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
}

impl CornerPermutationFixed {
    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn from_index(num: usize) -> CornerPermutationFixed {
        CornerPermutationFixed::SOLVED[num]
    }
}

impl CornerPermutationFixed {
    pub const SOLVED: [CornerPermutationFixed; 7] = [
        CornerPermutationFixed::Ubl,
        CornerPermutationFixed::Ubr,
        CornerPermutationFixed::Ufr,
        CornerPermutationFixed::Ufl,
        CornerPermutationFixed::Dfl,
        CornerPermutationFixed::Dfr,
        CornerPermutationFixed::Dbr,
    ];

    pub const STICKERS: [[CornerSticker; 3]; 7] = [
        [CornerSticker::Ubl, CornerSticker::Lub, CornerSticker::Bul],
        [CornerSticker::Ubr, CornerSticker::Bur, CornerSticker::Rub],
        [CornerSticker::Ufr, CornerSticker::Ruf, CornerSticker::Fur],
        [CornerSticker::Ufl, CornerSticker::Ful, CornerSticker::Luf],
        [CornerSticker::Dfl, CornerSticker::Ldf, CornerSticker::Fdl],
        [CornerSticker::Dfr, CornerSticker::Fdr, CornerSticker::Rdf],
        [CornerSticker::Dbr, CornerSticker::Rdb, CornerSticker::Bdr],
    ];
}

impl Default for CornersFixed {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CornerPermutationFixed> for CornerPermutation {
    fn from(value: CornerPermutationFixed) -> CornerPermutation {
        match value {
            CornerPermutationFixed::Ubl => CornerPermutation::Ubl,
            CornerPermutationFixed::Ubr => CornerPermutation::Ubr,
            CornerPermutationFixed::Ufr => CornerPermutation::Ufr,
            CornerPermutationFixed::Ufl => CornerPermutation::Ufl,
            CornerPermutationFixed::Dfl => CornerPermutation::Dfl,
            CornerPermutationFixed::Dfr => CornerPermutation::Dfr,
            CornerPermutationFixed::Dbr => CornerPermutation::Dbr,
        }
    }
}

impl CornersFixed {
    pub fn new() -> CornersFixed {
        CornersFixed {
            permutation: CornerPermutationFixed::SOLVED,
            orientation: [CornerOrientation::Good; 7],
        }
    }

    pub fn from_coordinate(coordinate: u32) -> CornersFixed {
        let permutation_coordinate =
            u16::try_from(coordinate / u32::from(CornersFixed::NUM_ORIENTATION_COORDINATES))
                .unwrap();
        let orientation_coordinate =
            u16::try_from(coordinate % u32::from(CornersFixed::NUM_ORIENTATION_COORDINATES))
                .unwrap();
        CornersFixed::from_coordinates(permutation_coordinate, orientation_coordinate)
    }

    pub fn from_coordinates(permutation: u16, orientation: u16) -> CornersFixed {
        let permutation = decode_permutation(permutation);
        let orientation = decode_orientation(orientation);
        CornersFixed {
            permutation,
            orientation,
        }
    }

    pub fn at(&self, position: CornerSticker) -> CornerSticker {
        CornerSticker::from_permutation_and_orientation(
            self.permutation[position.permutation().index()].into(),
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
        assert!(matches!(face, Face::U | Face::F | Face::R));
        self.cycle(&CornerSticker::face_cycle(face), count);
    }

    pub fn are_solved(&self) -> bool {
        self.permutation == CornerPermutationFixed::SOLVED
            && self.orientation == [CornerOrientation::Good; 7]
    }

    pub const NUM_COORDINATES: u32 = CornersFixed::NUM_PERMUTATION_COORDINATES as u32
        * CornersFixed::NUM_ORIENTATION_COORDINATES as u32;

    pub fn coordinate(&self) -> u32 {
        u32::from(self.permutation_coordinate())
            * u32::from(CornersFixed::NUM_ORIENTATION_COORDINATES)
            + u32::from(self.orientation_coordinate())
    }

    pub const NUM_ORIENTATION_COORDINATES: u16 = 3u16.pow(6);

    pub fn orientation_coordinate(&self) -> u16 {
        let sum = self.orientation[0..6]
            .iter()
            .enumerate()
            .map(|(i, co)| co.index() as u16 * 3u16.pow(i as u32))
            .sum();
        sum
    }

    pub const NUM_PERMUTATION_COORDINATES: u16 = super::FACTORIAL_U16[7];

    pub fn permutation_coordinate(&self) -> u16 {
        let mut x = 0;
        for i in (1..7).rev() {
            let mut s = 0;
            for j in (0..i).rev() {
                if self.permutation[j] > self.permutation[i] {
                    s += 1;
                }
            }
            x += s;
            x *= i;
        }
        x as u16
    }
}

fn decode_orientation(mut orientation: u16) -> [CornerOrientation; 7] {
    let mut sum = CornerOrientation::Good;
    let mut res = [CornerOrientation::Good; 7];
    for co in res.iter_mut().take(6) {
        *co = CornerOrientation::from_index(usize::from(orientation % 3));
        sum += *co;
        orientation /= 3;
    }
    res[6] = -sum;
    res
}

fn decode_permutation(mut permutation: u16) -> [CornerPermutationFixed; 7] {
    let mut res = [CornerPermutationFixed::Ubl; 7];

    let mut used = [false; 7];
    let mut order = [0; 7];

    for i in 0..7 {
        order[usize::from(i)] = permutation % (i + 1);
        permutation /= i + 1;
    }

    for i in (0..7).rev() {
        let mut k = 6;
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
        res[i] = CornerPermutationFixed::from_index(k);
        used[k] = true;
    }

    res
}
