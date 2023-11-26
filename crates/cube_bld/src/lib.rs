use cube::{
    CornerOrientation, CornerPermutation, CornerSticker, Corners, EdgeOrientation, EdgePermutation,
    EdgeSticker, Edges, Face, Handedness, Obliques, TCenters, WingSticker, Wings, XCenters,
};
use std::{
    collections::HashSet,
    fmt::Debug,
    hash::{BuildHasher, Hash, RandomState},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Memo<P: Pieces> {
    pub cycles: Vec<[P::Sticker; 3]>,
    pub parity: Option<[P::Sticker; 2]>,
    pub twists: Vec<(P::Permutation, P::Orientation)>,
}

pub trait Pieces: 'static + Copy + Debug {
    type Sticker: Sticker;
    type Permutation: Permutation;
    type Orientation: Orientation;

    fn at(&self, sticker: Self::Sticker) -> Self::Sticker;
    fn sticker(permutation: Self::Permutation, orientation: Self::Orientation) -> Self::Sticker;
    fn sticker_permutation(sticker: Self::Sticker) -> Self::Permutation;
    fn sticker_orientation(sticker: Self::Sticker) -> Self::Orientation;
    fn cycle(&mut self, positions: &[Self::Sticker], count: u8);
}

pub trait Sticker: 'static + Copy + Debug + Hash + Eq + Ord {
    const SOLVED: &'static [Self];

    fn on_face(face: Face) -> [Self; 4];
    fn color(self) -> Face;
    fn index(self) -> usize;
    fn from_index(index: usize) -> Self;
}

pub trait Permutation: 'static + Copy + Debug + Hash + Eq + Ord {
    const N: usize;
    const SOLVED: &'static [Self];
}

pub trait Orientation: 'static + Copy + Debug + Hash + Eq + Ord {
    const GOOD: Self;
    const N: usize;
    fn from_index(i: usize) -> Self;
}

impl Pieces for Edges {
    type Sticker = EdgeSticker;
    type Permutation = EdgePermutation;
    type Orientation = EdgeOrientation;

    fn at(&self, sticker: EdgeSticker) -> EdgeSticker {
        self.at(sticker)
    }

    fn sticker(permutation: EdgePermutation, orientation: EdgeOrientation) -> EdgeSticker {
        EdgeSticker::from_permutation_and_orientation(permutation, orientation)
    }

    fn sticker_permutation(sticker: EdgeSticker) -> EdgePermutation {
        sticker.permutation()
    }

    fn sticker_orientation(sticker: EdgeSticker) -> EdgeOrientation {
        sticker.orientation()
    }

    fn cycle(&mut self, positions: &[Self::Sticker], count: u8) {
        self.cycle(positions, count)
    }
}

impl Sticker for EdgeSticker {
    const SOLVED: &'static [EdgeSticker] = &EdgeSticker::SOLVED;

    fn on_face(face: Face) -> [Self; 4] {
        EdgeSticker::face_cycle(face)
    }

    fn color(self) -> Face {
        self.color()
    }

    fn index(self) -> usize {
        self.index()
    }

    fn from_index(index: usize) -> Self {
        EdgeSticker::from_index(index)
    }
}

impl Permutation for EdgePermutation {
    const N: usize = 12;
    const SOLVED: &'static [EdgePermutation] = &EdgePermutation::SOLVED;
}

impl Orientation for EdgeOrientation {
    const GOOD: EdgeOrientation = EdgeOrientation::Good;

    const N: usize = 2;

    fn from_index(i: usize) -> Self {
        EdgeOrientation::from_index(i)
    }
}

impl Pieces for Corners {
    type Sticker = CornerSticker;
    type Permutation = CornerPermutation;
    type Orientation = CornerOrientation;

    fn at(&self, sticker: CornerSticker) -> CornerSticker {
        self.at(sticker)
    }

    fn sticker(permutation: CornerPermutation, orientation: CornerOrientation) -> CornerSticker {
        CornerSticker::from_permutation_and_orientation(permutation, orientation)
    }

    fn sticker_permutation(sticker: CornerSticker) -> CornerPermutation {
        sticker.permutation()
    }

    fn sticker_orientation(sticker: CornerSticker) -> CornerOrientation {
        sticker.orientation()
    }

    fn cycle(&mut self, positions: &[Self::Sticker], count: u8) {
        self.cycle(positions, count)
    }
}

impl Sticker for CornerSticker {
    const SOLVED: &'static [Self] = &CornerSticker::SOLVED;

    fn on_face(face: Face) -> [Self; 4] {
        CornerSticker::face_cycle(face)
    }

    fn color(self) -> Face {
        self.color()
    }

    fn index(self) -> usize {
        self.index()
    }

    fn from_index(index: usize) -> Self {
        CornerSticker::from_index(index)
    }
}

impl Permutation for CornerPermutation {
    const N: usize = 8;
    const SOLVED: &'static [Self] = &CornerPermutation::SOLVED;
}

impl Orientation for CornerOrientation {
    const GOOD: CornerOrientation = CornerOrientation::Good;

    const N: usize = 3;

    fn from_index(i: usize) -> Self {
        CornerOrientation::from_index(i)
    }
}

impl Pieces for Wings {
    type Sticker = WingSticker;
    type Permutation = EdgeSticker;
    type Orientation = ();

    fn at(&self, sticker: WingSticker) -> WingSticker {
        self.at(sticker.rh())
    }

    fn sticker(permutation: EdgeSticker, (): ()) -> WingSticker {
        WingSticker::from_permutation_and_handedness_ignoring_orientation(
            permutation,
            Handedness::Right,
        )
    }

    fn sticker_permutation(sticker: WingSticker) -> EdgeSticker {
        sticker.permutation()
    }

    fn sticker_orientation(_: WingSticker) {}

    fn cycle(&mut self, _positions: &[Self::Sticker], _count: u8) {
        // This is theoretically possible to implement but I can't be bothered to right now
        todo!()
    }
}

impl Sticker for WingSticker {
    const SOLVED: &'static [WingSticker] = &WingSticker::SOLVED;

    fn on_face(face: Face) -> [Self; 4] {
        EdgeSticker::face_cycle(face).map(|permutation| Wings::sticker(permutation, ()))
    }

    fn color(self) -> Face {
        self.color()
    }

    fn index(self) -> usize {
        self.index()
    }

    fn from_index(index: usize) -> Self {
        WingSticker::from_index(index)
    }
}

impl Permutation for EdgeSticker {
    const N: usize = 24;
    const SOLVED: &'static [Self] = &EdgeSticker::SOLVED;
}

impl Permutation for CornerSticker {
    const N: usize = 24;
    const SOLVED: &'static [Self] = &CornerSticker::SOLVED;
}

impl Orientation for () {
    const GOOD: () = ();

    const N: usize = 1;

    fn from_index(_: usize) -> Self {}
}

fn find_unsolved_piece<P: Pieces>(
    solved_pieces: &HashSet<P::Permutation>,
    exclude: &[P::Permutation],
) -> Option<P::Sticker> {
    P::Permutation::SOLVED
        .iter()
        .copied()
        .find(|&piece| !solved_pieces.contains(&piece) && !exclude.contains(&piece))
        .map(|sticker| P::sticker(sticker, P::Orientation::GOOD))
}

#[allow(dead_code)]
fn find_unsolved_piece_randomly<P: Pieces>(
    solved_pieces: &HashSet<P::Permutation>,
    exclude: &[P::Permutation],
) -> Option<P::Sticker> {
    let candidates: Vec<_> = P::Permutation::SOLVED
        .iter()
        .copied()
        .filter(|&piece| !solved_pieces.contains(&piece) && !exclude.contains(&piece))
        .collect();

    if candidates.is_empty() {
        return None;
    }

    let seed = RandomState::new().hash_one(0);
    let seed2 = RandomState::new().hash_one(0);

    let piece = candidates[seed as usize % candidates.len()];
    Some(P::sticker(
        piece,
        P::Orientation::from_index(seed2 as usize % P::Orientation::N),
    ))
}

fn find_unsolved_piece_on_face<P: Pieces>(
    solved_pieces: &HashSet<P::Permutation>,
    face: Face,
    exclude: &[P::Permutation],
) -> Option<P::Sticker> {
    P::Sticker::on_face(face).iter().copied().find(|&piece| {
        !solved_pieces.contains(&P::sticker_permutation(piece))
            && !exclude.contains(&P::sticker_permutation(piece))
    })
}

#[allow(dead_code)]
fn find_unsolved_piece_on_face_randomly<P: Pieces>(
    solved_pieces: &HashSet<P::Permutation>,
    face: Face,
    exclude: &[P::Permutation],
) -> Option<P::Sticker> {
    let candidates: Vec<_> = P::Sticker::on_face(face)
        .iter()
        .copied()
        .filter(|&piece| {
            !solved_pieces.contains(&P::sticker_permutation(piece))
                && !exclude.contains(&P::sticker_permutation(piece))
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    let seed = RandomState::new().hash_one(0);

    let piece = candidates[seed as usize % candidates.len()];
    Some(piece)
}

pub fn memo<P: Pieces>(pieces: &P, buffer: P::Sticker) -> Memo<P> {
    let mut solved_pieces: HashSet<P::Permutation> = HashSet::new();
    let mut twists = Vec::new();

    // Solve twists
    for &piece in P::Permutation::SOLVED {
        let sticker = P::sticker(piece, P::Orientation::GOOD);

        if P::sticker_permutation(pieces.at(sticker)) == piece
            && piece != P::sticker_permutation(buffer)
        {
            if P::sticker_orientation(pieces.at(sticker)) != P::Orientation::GOOD {
                twists.push((piece, P::sticker_orientation(pieces.at(sticker))));
            }
            solved_pieces.insert(piece);
        }
    }

    // Check if just twists were enough
    if solved_pieces.len() == P::Permutation::N - 1 {
        return Memo {
            cycles: Vec::new(),
            parity: None,
            twists,
        };
    }

    let mut cycles: Vec<[P::Sticker; 3]> = Vec::new();
    // Current refers to the piece at which we can find the piece that is currently in the buffer.
    let mut zeroth = buffer;
    let mut cycle_end = buffer;

    loop {
        let first = pieces.at(zeroth);
        let second = pieces.at(first);

        if solved_pieces.len() == P::Permutation::N - 1 {
            return Memo {
                cycles,
                parity: None,
                twists,
            };
        }

        if solved_pieces.len() == P::Permutation::N - 2 {
            return Memo {
                cycles,
                parity: Some([buffer, first]),
                twists,
            };
        }

        if P::sticker_permutation(first) == P::sticker_permutation(buffer)
            || solved_pieces.contains(&P::sticker_permutation(first))
        {
            // Start new cycle
            let unsolved = find_unsolved_piece::<P>(
                &solved_pieces,
                &[
                    P::sticker_permutation(buffer),
                    P::sticker_permutation(cycle_end),
                ],
            )
            .unwrap();

            let next = pieces.at(unsolved);

            cycles.push([buffer, unsolved, next]);
            solved_pieces.insert(P::sticker_permutation(next));

            zeroth = next;
            cycle_end = unsolved;
        } else if P::sticker_permutation(first) == P::sticker_permutation(cycle_end)
            || P::sticker_permutation(second) == P::sticker_permutation(buffer)
        {
            // Start new cycle
            let unsolved = find_unsolved_piece::<P>(
                &solved_pieces,
                &[
                    P::sticker_permutation(buffer),
                    P::sticker_permutation(first),
                ],
            )
            .unwrap();

            cycles.push([buffer, first, unsolved]);
            solved_pieces.insert(P::sticker_permutation(first));

            zeroth = unsolved;
            cycle_end = zeroth;
        } else {
            cycles.push([buffer, first, second]);
            solved_pieces.insert(P::sticker_permutation(first));
            solved_pieces.insert(P::sticker_permutation(second));

            if P::sticker_permutation(second) == P::sticker_permutation(cycle_end) {
                // We finished a cycle cleanly
                zeroth = buffer;
                cycle_end = buffer;
            } else {
                // We haven't finished this cycle
                zeroth = second;
            }
        }
    }
}

impl Pieces for XCenters {
    type Sticker = CornerSticker;
    type Permutation = CornerSticker;
    type Orientation = ();

    fn at(&self, sticker: CornerSticker) -> CornerSticker {
        self.at(sticker)
    }

    fn sticker(permutation: CornerSticker, (): ()) -> CornerSticker {
        permutation
    }

    fn sticker_permutation(sticker: CornerSticker) -> CornerSticker {
        sticker
    }

    fn sticker_orientation(_: CornerSticker) {}

    fn cycle(&mut self, positions: &[Self::Sticker], count: u8) {
        self.cycle(positions, count)
    }
}

impl Pieces for TCenters {
    type Sticker = EdgeSticker;
    type Permutation = EdgeSticker;
    type Orientation = ();

    fn at(&self, sticker: EdgeSticker) -> EdgeSticker {
        self.at(sticker)
    }

    fn sticker(permutation: EdgeSticker, (): ()) -> EdgeSticker {
        permutation
    }

    fn sticker_permutation(sticker: EdgeSticker) -> EdgeSticker {
        sticker
    }

    fn sticker_orientation(_: EdgeSticker) {}

    fn cycle(&mut self, positions: &[Self::Sticker], count: u8) {
        self.cycle(positions, count)
    }
}

impl Pieces for Obliques {
    type Sticker = EdgeSticker;
    type Permutation = EdgeSticker;
    type Orientation = ();

    fn at(&self, sticker: EdgeSticker) -> EdgeSticker {
        self.at(sticker)
    }

    fn sticker(permutation: EdgeSticker, (): ()) -> EdgeSticker {
        permutation
    }

    fn sticker_permutation(sticker: EdgeSticker) -> EdgeSticker {
        sticker
    }

    fn sticker_orientation(_: EdgeSticker) {}

    fn cycle(&mut self, positions: &[Self::Sticker], count: u8) {
        self.cycle(positions, count)
    }
}

pub fn memo_centers<P: Pieces>(pieces: &P, buffer: P::Sticker) -> Memo<P> {
    let mut solved_pieces = HashSet::new();
    let twists = Vec::new();

    for &piece in P::Sticker::SOLVED {
        if pieces.at(piece).color() == piece.color() && piece != buffer {
            assert!(solved_pieces.insert(P::sticker_permutation(piece)));
        }
    }

    let mut cycles = Vec::new();
    let mut zeroth = Some(buffer);
    let mut cycle_end = buffer;

    loop {
        let mut new_cycle_end = None;
        let mut new_solved_pieces = HashSet::new();

        let first_target = {
            match zeroth {
                Some(zeroth) => {
                    let first = pieces.at(zeroth);
                    match find_unsolved_piece_on_face::<P>(
                        &solved_pieces,
                        first.color(),
                        &[P::sticker_permutation(buffer)],
                    ) {
                        Some(piece) => {
                            new_solved_pieces.insert(P::sticker_permutation(piece));
                            piece
                        }
                        None => {
                            // Cycle break
                            match find_unsolved_piece::<P>(
                                &solved_pieces,
                                &[P::sticker_permutation(buffer)],
                            ) {
                                Some(first_target) => {
                                    assert_eq!(new_cycle_end, None);
                                    new_cycle_end = Some(first_target);
                                    first_target
                                }
                                None => {
                                    assert_eq!(solved_pieces.len(), 23);
                                    return Memo {
                                        cycles,
                                        parity: None,
                                        twists,
                                    };
                                }
                            }
                        }
                    }
                }
                None => match find_unsolved_piece::<P>(
                    &solved_pieces,
                    &[P::sticker_permutation(buffer)],
                ) {
                    Some(first_target) => {
                        assert_eq!(new_cycle_end, None);
                        new_cycle_end = Some(first_target);
                        first_target
                    }
                    None => {
                        return Memo {
                            cycles,
                            parity: None,
                            twists,
                        };
                    }
                },
            }
        };
        let second_target = if first_target == cycle_end {
            // Cycle break
            match find_unsolved_piece::<P>(
                &solved_pieces,
                &[
                    P::sticker_permutation(buffer),
                    P::sticker_permutation(first_target),
                ],
            ) {
                Some(second_target) => {
                    assert_ne!(second_target, cycle_end);
                    assert_eq!(new_cycle_end, None);
                    new_cycle_end = Some(second_target);
                    second_target
                }
                None => {
                    return Memo {
                        cycles,
                        parity: Some([buffer, first_target]),
                        twists,
                    };
                }
            }
        } else {
            let second = pieces.at(first_target);

            match find_unsolved_piece_on_face::<P>(
                &solved_pieces,
                second.color(),
                &[
                    P::sticker_permutation(buffer),
                    P::sticker_permutation(first_target),
                ],
            ) {
                Some(piece) => {
                    new_solved_pieces.insert(P::sticker_permutation(piece));
                    piece
                }
                None => {
                    // Cycle break
                    let second_target = match find_unsolved_piece::<P>(
                        &solved_pieces,
                        &[
                            P::sticker_permutation(buffer),
                            P::sticker_permutation(first_target),
                        ],
                    ) {
                        Some(target) => target,
                        None => {
                            assert_eq!(solved_pieces.len(), 22);
                            return Memo {
                                cycles,
                                parity: Some([buffer, first_target]),
                                twists,
                            };
                        }
                    };

                    assert_ne!(first_target, cycle_end);
                    assert_ne!(second_target, cycle_end);

                    assert_eq!(new_cycle_end, None);
                    new_cycle_end = Some(second_target);

                    second_target
                }
            }
        };

        cycles.push([buffer, first_target, second_target]);

        if second_target == cycle_end {
            zeroth = None;
            cycle_end = buffer;
        } else {
            zeroth = Some(second_target);
            cycle_end = new_cycle_end.unwrap_or(cycle_end);
        }

        solved_pieces.extend(new_solved_pieces);
    }
}
