use crate::{
    map_orientation, orientation_after_move, AnySticker, CornerOrientation, CornerPermutation, CornerSticker, Corners, EdgeOrientation, EdgePermutation, EdgeSticker, Edges, Face, Handedness, Obliques, ObliquesPair, TCenters, Wings, XCenters
};
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;
use oorandom::Rand32;

#[derive(Debug)]
pub struct RotatedCube<'a> {
    pub cube: &'a mut Cube,
    pub orientation: EdgeSticker,
}

/// A representation of the state of an NxNxN Rubik's cube.
#[derive(Clone, PartialEq, Eq)]
pub struct Cube {
    pub n: u16,
    pub corners: Corners,
    pub edges: Edges,
    pub layers: Vec<CubeLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeLayer {
    pub wings: Wings,
    pub tcenters: TCenters,
    pub xcenters: XCenters,
    pub obliques: Vec<ObliquesPair>,
}

/// Returns the number of [`CubeLayer`]s necessary for an `n` by `n` cube.
fn n_layers(n: u16) -> u16 {
    (n / 2).saturating_sub(1)
}

fn shuffled<const N: usize, T>(rand: &mut Rand32, mut arr: [T; N], parity: bool) -> [T; N]
where
    T: Copy,
{
    // Fisher-Yates
    let n = arr.len();
    let mut n_swaps = 0;
    for i in (1..n).rev() {
        let j = rand.rand_range(0..(i + 1) as u32) as usize;
        arr.swap(i, j);
        if i != j {
            n_swaps += 1;
        }
    }
    if (n_swaps % 2 == 0) != parity {
        arr.swap(0, 1);
    }
    arr
}

fn random_corner_orientation(rand: &mut Rand32) -> [CornerOrientation; 8] {
    let mut arr = [CornerOrientation::Good; 8];
    for co in arr.iter_mut().take(7) {
        let index = rand.rand_range(0..3);
        *co = CornerOrientation::from_index(index as usize);
    }

    arr[7] = match arr.iter().copied().map(|co| co.index()).sum::<usize>() % 3 {
        0 => CornerOrientation::Good,
        1 => CornerOrientation::BadCcw,
        2 => CornerOrientation::BadCw,
        _ => unreachable!(),
    };

    assert_eq!(
        arr.iter().copied().map(|co| co.index()).sum::<usize>() % 3,
        0
    );

    arr
}

fn random_edge_orientation(rand: &mut Rand32) -> [EdgeOrientation; 12] {
    let mut arr = [EdgeOrientation::Good; 12];
    for eo in arr.iter_mut().take(11) {
        let index = rand.rand_range(0..2);
        *eo = EdgeOrientation::from_index(index as usize);
    }

    arr[11] = match arr.iter().copied().map(|eo| eo as u8).sum::<u8>() % 2 {
        0 => EdgeOrientation::Good,
        1 => EdgeOrientation::Bad,
        _ => unreachable!(),
    };

    arr
}

impl Cube {
    pub fn is_solved(&self) -> bool {
        self.corners.are_solved()
            && self.edges.are_solved()
            && self.layers.iter().all(|layer| layer.is_solved())
    }

    pub fn is_solved_supercube(&self) -> bool {
        self.corners.are_solved()
            && self.edges.are_solved()
            && self.layers.iter().all(|layer| layer.is_solved_supercube())
    }

    pub fn is_solved_in_any_orientation(&self) -> bool {
        for face_index in 0..6 {
            let face = Face::from_index(face_index);
            let mut color = None;
            for y in 0..(self.n | 1) {
                for x in 0..(self.n | 1) {
                    let adj_x = x as i16 - self.n as i16 / 2;
                    let adj_y = -(y as i16 - self.n as i16 / 2);
                    if self.n % 2 == 0 && (adj_x == 0 || adj_y == 0) {
                        continue;
                    }

                    let this_color = self.color_at(face, adj_x, adj_y);
                    if this_color != color.unwrap_or(this_color) {
                        return false;
                    }
                    color = Some(this_color);
                }
            }
        }
        true
    }

    // TODO: Not truly uniform because the seed is only 64 bits?
    pub fn new_random(n: u16, seed: u64) -> Cube {
        assert!(n > 0);

        let mut rand = Rand32::new(seed);

        let corner_edge_parity = rand.rand_range(0..2) != 0;
        // TODO figure out how these actually work
        let left_oblique_parity = rand.rand_range(0..2) != 0;
        let right_oblique_parity = rand.rand_range(0..2) != 0;
        let wing_parity = rand.rand_range(0..2) != 0;
        let tcenter_parity = rand.rand_range(0..2) != 0;
        let xcenter_parity = rand.rand_range(0..2) != 0;

        Cube {
            n,
            corners: Corners {
                permutation: shuffled(&mut rand, CornerPermutation::SOLVED, corner_edge_parity),
                orientation: random_corner_orientation(&mut rand),
            },
            edges: Edges {
                permutation: shuffled(&mut rand, EdgePermutation::SOLVED, corner_edge_parity),
                orientation: random_edge_orientation(&mut rand),
            },
            layers: (0..n_layers(n))
                .map(|i| {
                    let obliques = (0..i)
                        .map(|_| ObliquesPair {
                            left: Obliques {
                                permutation: shuffled(
                                    &mut rand,
                                    EdgeSticker::SOLVED,
                                    left_oblique_parity,
                                ),
                            },
                            right: Obliques {
                                permutation: shuffled(
                                    &mut rand,
                                    EdgeSticker::SOLVED,
                                    right_oblique_parity,
                                ),
                            },
                        })
                        .collect();
                    CubeLayer {
                        wings: Wings {
                            permutation: shuffled(&mut rand, EdgeSticker::SOLVED, wing_parity),
                        },
                        tcenters: TCenters {
                            permutation: shuffled(&mut rand, EdgeSticker::SOLVED, tcenter_parity),
                        },
                        xcenters: XCenters {
                            permutation: shuffled(&mut rand, CornerSticker::SOLVED, xcenter_parity),
                        },
                        obliques,
                    }
                })
                .collect(),
        }
    }

    /// Returns a solved `n` by `n` cube.
    pub fn new_solved(n: u16) -> Cube {
        assert!(n > 0);

        Cube {
            n,
            corners: Corners::new(),
            edges: Edges::new(),
            layers: (0..n_layers(n))
                .map(|i| {
                    let obliques = vec![
                        ObliquesPair {
                            left: Obliques::new(),
                            right: Obliques::new()
                        };
                        usize::from(i)
                    ];
                    CubeLayer {
                        wings: Wings::new(),
                        tcenters: TCenters::new(),
                        xcenters: XCenters::new(),
                        obliques,
                    }
                })
                .collect(),
        }
    }

    pub fn color_at(&self, face: Face, x: i16, y: i16) -> Face {
        let sticker = AnySticker::at(self.n, face, x, y);

        match sticker {
            AnySticker::Center(face) => face,
            AnySticker::Edge(sticker) => self.edges.at(sticker).color(),
            AnySticker::Corner(sticker) => self.corners.at(sticker).color(),
            AnySticker::Wing(layer, position) => {
                self.layers[usize::from(layer)].wings.at(position).color()
            }
            AnySticker::TCenter(layer, position) => {
                self.layers[usize::from(layer)].tcenters.permutation[position.index()].color()
            }
            AnySticker::XCenter(layer, position) => {
                self.layers[usize::from(layer)].xcenters.permutation[position.index()].color()
            }
            AnySticker::Oblique(layer, index, position, kind) => {
                match kind {
                    Handedness::Left => self.layers[usize::from(layer)].obliques
                        [usize::from(index)]
                    .left
                    .at(position),
                    Handedness::Right => self.layers[usize::from(layer)].obliques
                        [usize::from(index)]
                    .right
                    .at(position),
                }
            }
            .color(),
        }
    }

    pub fn rotate_face(&mut self, face: Face, count: u8) {
        self.corners.rotate_face(face, count);
        if self.n % 2 == 1 {
            self.edges.rotate_face(face, count);
        }
        for layer in &mut self.layers {
            layer.rotate_face(face, count);
        }
    }

    pub fn rotate(&mut self, face: Face, layers: Range<u16>, count: u8) {
        for mut i in layers {
            if i >= self.n / 2 && self.n % 2 == 0 {
                i += 1;
            }
            if i <= self.n / 2 {
                self.rotate_slice(face, self.n / 2 - i, count);
            } else {
                let inverse = 4 - count % 4;
                self.rotate_slice(face.opposite(), i - self.n / 2, inverse);
            }
        }
    }

    pub fn rotate_slice(&mut self, face: Face, layer_index: u16, count: u8) {
        if layer_index == self.n / 2 {
            self.rotate_face(face, count)
        } else if layer_index == 0 {
            self.rotate_middle_slice(face, count);
        } else {
            self.rotate_non_middle_slice(layer_index - 1, face, count);
        }
    }

    pub fn rotate_non_middle_slice(&mut self, layer_index: u16, face: Face, count: u8) {
        let layer = &mut self.layers[usize::from(layer_index)];
        layer
            .wings
            .cycle(&EdgeSticker::slice_wing_cycle_rh(face), count);
        if self.n % 2 == 1 {
            layer
                .tcenters
                .cycle(&EdgeSticker::slice_center_cycle(face), count);
        }
        layer
            .xcenters
            .cycle(&CornerSticker::slice_center_cycle_lh(face), count);
        layer
            .xcenters
            .cycle(&CornerSticker::slice_center_cycle_rh(face), count);

        let ob_index = layer_index;
        for ob_layer in 1..((self.n / 2) - 1) {
            if ob_index >= ob_layer {
                continue;
            }

            self.layers[usize::from(ob_layer)].obliques[usize::from(ob_index)]
                .left
                .cycle(&EdgeSticker::slice_wing_cycle_rh(face), count);

            self.layers[usize::from(ob_layer)].obliques[usize::from(ob_index)]
                .right
                .cycle(&EdgeSticker::slice_wing_cycle_lh(face), count);
        }

        let ob_layer = layer_index;
        for ob_index in 0..(self.n / 2) {
            if ob_index >= ob_layer {
                continue;
            }

            self.layers[usize::from(ob_layer)].obliques[usize::from(ob_index)]
                .left
                .cycle(&EdgeSticker::slice_center_cycle(face), count);

            self.layers[usize::from(ob_layer)].obliques[usize::from(ob_index)]
                .right
                .cycle(&EdgeSticker::slice_center_cycle(face), count);
        }
    }

    fn rotate_middle_slice(&mut self, face: Face, count: u8) {
        if self.n % 2 == 0 {
            return;
        }
        self.rotate(face, 0..self.n / 2, 4 - count % 4);
        self.rotate(face.opposite(), 0..self.n / 2, count);
    }
}

impl<'a> RotatedCube<'a> {
    pub fn new(cube: &'a mut Cube) -> RotatedCube<'a> {
        RotatedCube {
            cube,
            orientation: EdgeSticker::Uf,
        }
    }

    pub fn rotate(&mut self, face: Face, layers: Range<u16>, count: u8) {
        let face = map_orientation(self.orientation, face);
        self.orientation =
            orientation_after_move(self.cube.n, self.orientation, face, layers.clone(), count);

        if layers.start != 0 {
            self.cube.rotate(face, 0..layers.end, count);
            self.cube.rotate(face, 0..layers.start, 4 - count % 4);
        } else if layers.end == self.cube.n / 2
            && self.cube.n % 2 == 0
            && face < face.opposite()
            && self.cube.n % 2 == 1
        {
            self.cube.rotate(face.opposite(), layers, count);
        } else {
            self.cube.rotate(face, layers, count);
        }
    }
}

impl CubeLayer {
    fn rotate_face(&mut self, face: Face, count: u8) {
        self.wings.rotate_face(face, count);
        self.xcenters.rotate_face(face, count);
        self.tcenters.rotate_face(face, count);

        for obliques in self.obliques.iter_mut() {
            obliques.left.rotate_face(face, count);
            obliques.right.rotate_face(face, count);
        }
    }

    pub fn is_solved(&self) -> bool {
        self.wings.are_solved()
            && self.tcenters.are_solved()
            && self.xcenters.are_solved()
            && self.obliques.iter().all(|obliques| obliques.are_solved())
    }

    pub fn is_solved_supercube(&self) -> bool {
        self.wings.are_solved()
            && self.tcenters.are_solved_supercube()
            && self.xcenters.are_solved_supercube()
            && self
                .obliques
                .iter()
                .all(|obliques| obliques.are_solved_supercube())
    }
}

impl Cube {
    #[allow(unused)]
    pub fn ansi_repr(&self) -> String {
        let n = self.n;
        let cube = self;
        let mut s = String::new();
        let height = 3 * usize::from(self.n);
        let width = 4 * usize::from(self.n);
        let mut grid = vec![String::from("  "); width * height];

        let face_offsets = [(1, 0), (0, 1), (1, 1), (2, 1), (3, 1), (1, 2)];

        for face_index in 0..6 {
            let face = Face::from_index(face_index);
            let (x_offset, y_offset) = face_offsets[face_index];
            for x in 0..(n | 1) {
                for y in 0..(n | 1) {
                    let adj_x = x as i16 - cube.n as i16 / 2;
                    let adj_y = -(y as i16 - cube.n as i16 / 2);
                    if n % 2 == 0 && (adj_x == 0 || adj_y == 0) {
                        continue;
                    }

                    let x = if n % 2 == 0 && x > n / 2 { x - 1 } else { x };
                    let y = if n % 2 == 0 && y > n / 2 { y - 1 } else { y };

                    let color = cube.color_at(face, adj_x, adj_y);

                    grid[(((y_offset * usize::from(self.n) + usize::from(y)) * width)
                        + x_offset * usize::from(self.n)
                        + usize::from(x))] = format!(
                        "\x1b[{}m{color:?} \x1b[0m",
                        match color {
                            Face::U => "38;2;255;255;255;48;2;255;255;255",
                            Face::L => "38;2;255;127;0;48;2;255;127;0",
                            Face::F => "38;2;0;255;0;48;2;0;255;0",
                            Face::R => "38;2;255;0;0;48;2;255;0;0",
                            Face::B => "38;2;0;0;255;48;2;0;0;255",
                            Face::D => "38;2;255;255;0;48;2;255;255;0",
                        }
                    );
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let t = &grid[y * width + x];
                if t == " " && x > self.n as usize {
                    continue;
                }

                s.push_str(t);
            }
            if y != height - 1 {
                s.push('\n');
            }
        }
        s
    }
}

impl core::fmt::Debug for Cube {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for face_index in 0..6 {
            if face_index != 0 {
                write!(f, " / ")?;
            }
            let face = Face::from_index(face_index);
            for y in 0..(self.n | 1) {
                for x in 0..(self.n | 1) {
                    let adj_x = x as i16 - self.n as i16 / 2;
                    let adj_y = -(y as i16 - self.n as i16 / 2);
                    if self.n % 2 == 0 && (adj_x == 0 || adj_y == 0) {
                        continue;
                    }
                    write!(f, "{:?}", self.color_at(face, adj_x, adj_y))?;
                }
                if y != self.n - 1 {
                    write!(f, " ")?;
                }
            }
        }
        Ok(())
    }
}
