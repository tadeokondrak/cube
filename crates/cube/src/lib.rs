//! This crate contains abstractions for Rubik's cube state.
//!
//! It provides composable helpers for the states of mostly-independent
//! components like the [`Corners`] and [`Edges`] of a 3x3x3 cube.
//!
//! It also provides types to describe other parts of the cube, like [`Axis`],
//! [`Face`] and [`EdgeSticker`].
//!
//! All of this is combined in the [`Cube`] and [`RotatedCube`] abstractions,
//! which describe the state of an NxNxN Rubik's cube with and without keeping
//! track of its orientation, respectively.
//!
//! Note: Most of the abstractions in this crate are not meant to be extremely
//! fast, yet. See [`corners::fixed`] for some preliminary work in this respect.

#![no_std]
extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod corners;
pub mod cube;
pub mod edges;
pub mod obliques;
pub mod tables;
pub mod tcenters;
pub mod wings;
pub mod xcenters;

#[cfg(test)]
mod tests;

pub use corners::{
    fixed::{CornerCoordsFixed, CornerCoordsMoveTableFixed, CornerPermutationFixed, CornersFixed},
    CornerDirection, CornerOrientation, CornerPermutation, CornerSticker, Corners,
};
pub use cube::{Cube, CubeLayer, RotatedCube};
pub use edges::{EdgeDirection, EdgeOrientation, EdgePermutation, EdgeSticker, Edges};
pub use obliques::{Obliques, ObliquesPair};
pub use tcenters::TCenters;
pub use wings::{WingSticker, Wings};
pub use xcenters::XCenters;

use core::{cmp::Ordering, fmt::Debug, ops::Range};

/// Axis of rotation.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    /// X follows R
    X,
    /// Y follows U
    Y,
    /// Z follows F
    Z,
}

impl Axis {
    pub const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
}

pub fn map_orientation(orientation: EdgeSticker, face: Face) -> Face {
    match face {
        Face::U => orientation.color(),
        Face::L => orientation
            .color()
            .cross_lh(orientation.flipped().color())
            .unwrap(),
        Face::F => orientation.flipped().color(),
        Face::R => orientation
            .color()
            .cross_rh(orientation.flipped().color())
            .unwrap(),
        Face::B => orientation.flipped().color().opposite(),
        Face::D => orientation.color().opposite(),
    }
}

pub fn orientation_after_move(
    n: u16,
    orientation: EdgeSticker,
    face: Face,
    layers: Range<u16>,
    count: u8,
) -> EdgeSticker {
    if n % 2 == 0 {
        return EdgeSticker::Uf;
    }
    if layers.end > n / 2 {
        let (axis, invert) = match face {
            Face::R => (Axis::X, false),
            Face::L => (Axis::X, true),
            Face::U => (Axis::Y, false),
            Face::D => (Axis::Y, true),
            Face::F => (Axis::Z, false),
            Face::B => (Axis::Z, true),
        };
        let count = if invert { 4 - count % 4 } else { count };
        EdgeSticker::from_faces(
            rotate_face(map_orientation(orientation, Face::U), axis, count),
            rotate_face(map_orientation(orientation, Face::F), axis, count),
        )
    } else {
        orientation
    }
}

pub fn rotate_face(face: Face, axis: Axis, count: u8) -> Face {
    let axis_face = match axis {
        Axis::X => Face::R,
        Axis::Y => Face::U,
        Axis::Z => Face::F,
    };
    match count % 4 {
        0 => face,
        1 => face.cross_lh(axis_face).unwrap_or(face),
        2 => {
            if face == axis_face || face == axis_face.opposite() {
                face
            } else {
                face.opposite()
            }
        }
        3 => face.cross_rh(axis_face).unwrap_or(face),
        _ => unreachable!("count"),
    }
}

/// One of the six faces of a cube.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Face {
    /// The top face.
    U = 0,
    /// The left face.
    L,
    /// The front face.
    F,
    /// The right face.
    R,
    /// The back face.
    B,
    /// The bottom face.
    D,
}

impl Face {
    pub fn from_index(i: usize) -> Face {
        assert!(i < 6);
        [Face::U, Face::L, Face::F, Face::R, Face::B, Face::D][i]
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn neighbors(self) -> [Face; 4] {
        Face::NEIGHBORS[self.index()]
    }

    pub const fn opposite(self) -> Face {
        Face::OPPOSITES[self.index()]
    }

    pub fn cross_rh(self, other: Face) -> Option<Face> {
        if self == other || self == other.opposite() {
            None
        } else {
            Some(Face::CROSS[self.index()][other.index()])
        }
    }

    pub fn cross_lh(self, other: Face) -> Option<Face> {
        if self == other || self == other.opposite() {
            None
        } else {
            Some(Face::CROSS[other.index()][self.index()])
        }
    }

    pub fn is_less_ergonomic_than_the_opposite_face(self) -> bool {
        match self {
            Face::U | Face::F | Face::R => false,
            Face::D | Face::B | Face::L => true,
        }
    }
}

/// For wings and obliques, handedness determines which side of a pair of pieces or stickers is referred to.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handedness {
    Left = 0,
    Right,
}

impl Handedness {
    fn index(self) -> usize {
        self as usize
    }
}

pub(crate) fn sticker_cycle<const N: usize, P>(permutation: &mut [P; N], positions: &[P], count: u8)
where
    P: Copy + Into<u8>,
{
    let old_permutation = *permutation;

    for i in 0..positions.len() {
        let j = (i + usize::from(count) + positions.len()) % positions.len();
        let to_sticker = positions[j];
        let from_sticker = positions[i];
        permutation[usize::from(to_sticker.into())] =
            old_permutation[usize::from(from_sticker.into())];
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AnySticker {
    Center(Face),
    Edge(EdgeSticker),
    Corner(CornerSticker),
    Wing(u16, WingSticker),
    TCenter(u16, EdgeSticker),
    XCenter(u16, CornerSticker),
    Oblique(u16, u16, EdgeSticker, Handedness),
}

impl AnySticker {
    pub(crate) fn at(n: u16, face: Face, x: i16, y: i16) -> AnySticker {
        if n % 2 == 0 {
            assert_ne!(x, 0);
            assert_ne!(y, 0);
        }

        if x == 0 && y == 0 {
            AnySticker::Center(face)
        } else if x == 0 || y == 0 {
            let dir = match (x.cmp(&0), y.cmp(&0)) {
                (Ordering::Equal, Ordering::Greater) => EdgeDirection::Top,
                (Ordering::Greater, Ordering::Equal) => EdgeDirection::Right,
                (Ordering::Equal, Ordering::Less) => EdgeDirection::Bottom,
                (Ordering::Less, Ordering::Equal) => EdgeDirection::Left,
                _ => unreachable!(),
            };
            let sticker = EdgeSticker::from_face_and_direction(face, dir);

            if x == 0 && y.unsigned_abs() == n / 2 || y == 0 && x.unsigned_abs() == n / 2 {
                AnySticker::Edge(sticker)
            } else {
                AnySticker::TCenter(core::cmp::max(x.abs(), y.abs()) as u16 - 1, sticker)
            }
        } else if x.abs() == y.abs() {
            let dir = match (x.cmp(&0), y.cmp(&0)) {
                (Ordering::Less, Ordering::Less) => CornerDirection::BottomLeft,
                (Ordering::Less, Ordering::Greater) => CornerDirection::TopLeft,
                (Ordering::Greater, Ordering::Less) => CornerDirection::BottomRight,
                (Ordering::Greater, Ordering::Greater) => CornerDirection::TopRight,
                _ => unreachable!(),
            };
            let sticker = CornerSticker::from_face_and_direction(face, dir);
            if x.unsigned_abs() == n / 2 {
                AnySticker::Corner(sticker)
            } else {
                AnySticker::XCenter(x.unsigned_abs() - 1, sticker)
            }
        } else {
            #[derive(Clone, Copy)]
            enum Edge {
                X,
                Y,
            }

            let at_x_edge = x.unsigned_abs() == n / 2;
            let at_y_edge = y.unsigned_abs() == n / 2;

            if at_x_edge || at_y_edge {
                let edge = match (at_x_edge, at_y_edge) {
                    (true, true) | (false, false) => unreachable!(),
                    (true, false) => Edge::X,
                    (false, true) => Edge::Y,
                };

                let dir = match (x.cmp(&0), y.cmp(&0), edge) {
                    (Ordering::Equal, _, _) | (_, Ordering::Equal, _) => unreachable!(),
                    (Ordering::Less, Ordering::Less, Edge::X)
                    | (Ordering::Less, Ordering::Greater, Edge::X) => EdgeDirection::Left,
                    (Ordering::Greater, Ordering::Greater, Edge::X)
                    | (Ordering::Greater, Ordering::Less, Edge::X) => EdgeDirection::Right,
                    (Ordering::Less, Ordering::Greater, Edge::Y)
                    | (Ordering::Greater, Ordering::Greater, Edge::Y) => EdgeDirection::Top,
                    (Ordering::Greater, Ordering::Less, Edge::Y)
                    | (Ordering::Less, Ordering::Less, Edge::Y) => EdgeDirection::Bottom,
                };

                let handedness = match (x.cmp(&0), y.cmp(&0), edge) {
                    (Ordering::Equal, _, _) | (_, Ordering::Equal, _) => unreachable!(),
                    (Ordering::Greater, Ordering::Greater, Edge::X)
                    | (Ordering::Less, Ordering::Less, Edge::X)
                    | (Ordering::Greater, Ordering::Less, Edge::Y)
                    | (Ordering::Less, Ordering::Greater, Edge::Y) => Handedness::Right,

                    (Ordering::Greater, Ordering::Less, Edge::X)
                    | (Ordering::Less, Ordering::Greater, Edge::X)
                    | (Ordering::Greater, Ordering::Greater, Edge::Y)
                    | (Ordering::Less, Ordering::Less, Edge::Y) => Handedness::Left,
                };

                let layer = core::cmp::min(x.abs(), y.abs()) as u16 - 1;

                let edge_sticker = EdgeSticker::from_face_and_direction(face, dir);

                let wing_sticker =
                    WingSticker::from_permutation_and_handedness_considering_orientation(
                        edge_sticker,
                        handedness,
                    );

                AnySticker::Wing(layer, wing_sticker)
            } else {
                let layer = core::cmp::max(x.abs(), y.abs()) as u16 - 1;
                let index = core::cmp::min(x.abs(), y.abs()) as u16 - 1;
                let at_x_edge = x.unsigned_abs() - 1 == layer;
                let at_y_edge = y.unsigned_abs() - 1 == layer;
                let edge = match (at_x_edge, at_y_edge) {
                    (true, true) | (false, false) => unreachable!(),
                    (true, false) => Edge::X,
                    (false, true) => Edge::Y,
                };

                let dir = match (x.cmp(&0), y.cmp(&0), edge) {
                    (Ordering::Equal, _, _) | (_, Ordering::Equal, _) => unreachable!(),
                    (Ordering::Less, Ordering::Less, Edge::X)
                    | (Ordering::Less, Ordering::Greater, Edge::X) => EdgeDirection::Left,
                    (Ordering::Greater, Ordering::Greater, Edge::X)
                    | (Ordering::Greater, Ordering::Less, Edge::X) => EdgeDirection::Right,
                    (Ordering::Less, Ordering::Greater, Edge::Y)
                    | (Ordering::Greater, Ordering::Greater, Edge::Y) => EdgeDirection::Top,
                    (Ordering::Greater, Ordering::Less, Edge::Y)
                    | (Ordering::Less, Ordering::Less, Edge::Y) => EdgeDirection::Bottom,
                };

                let handedness = match (x.cmp(&0), y.cmp(&0), edge) {
                    (Ordering::Equal, _, _) | (_, Ordering::Equal, _) => unreachable!(),
                    (Ordering::Greater, Ordering::Greater, Edge::X)
                    | (Ordering::Less, Ordering::Less, Edge::X)
                    | (Ordering::Greater, Ordering::Less, Edge::Y)
                    | (Ordering::Less, Ordering::Greater, Edge::Y) => Handedness::Right,

                    (Ordering::Greater, Ordering::Less, Edge::X)
                    | (Ordering::Less, Ordering::Greater, Edge::X)
                    | (Ordering::Greater, Ordering::Greater, Edge::Y)
                    | (Ordering::Less, Ordering::Less, Edge::Y) => Handedness::Left,
                };

                let sticker = EdgeSticker::from_face_and_direction(face, dir);
                AnySticker::Oblique(layer, index, sticker, handedness)
            }
        }
    }
}
