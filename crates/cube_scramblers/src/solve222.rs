use cube::{CornerCoordsFixed, CornerCoordsMoveTableFixed, CornersFixed, Face};
use cube_notation::Move;
use alloc::{vec::Vec, vec, boxed::Box};

pub fn solve_state(
    corners: CornerCoordsFixed,
    pruntab: &PruningTables,
    movetab: &CornerCoordsMoveTableFixed,
) -> Vec<Move> {
    fn go(
        corners: CornerCoordsFixed,
        moves: &mut Vec<Move>,
        pruntab: &PruningTables,
        moves_left: u8,
        movetab: &CornerCoordsMoveTableFixed,
    ) -> bool {
        if corners.are_solved() {
            return true;
        }

        if moves_left == 0 {
            return false;
        }

        for face in [Face::U, Face::R, Face::F] {
            if moves.last().is_some_and(|mv| mv.face == face) {
                continue;
            }
            if moves.last().is_some_and(|mv| mv.face == face.opposite()) && face < face.opposite() {
                continue;
            }
            for count in 1..4 {
                let new_corners = movetab.rotate_face(corners, face, count);

                if !pruntab.check(&new_corners, moves_left) {
                    continue;
                }

                moves.push(Move {
                    n: 2,
                    face,
                    start: 0,
                    end: 1,
                    count,
                });

                if go(new_corners, moves, pruntab, moves_left - 1, movetab) {
                    return true;
                } else {
                    moves.pop();
                }
            }
        }

        false
    }

    let mut moves = Vec::new();
    for limit in 0..=11 {
        if !pruntab.check(&corners, limit) {
            continue;
        }

        if go(corners, &mut moves, pruntab, limit, movetab) {
            return moves;
        }
    }
    panic!("no solution found")
}

pub struct PruningTables {
    permutation: Box<[u8; CornersFixed::NUM_PERMUTATION_COORDINATES as usize]>,
    orientation: Box<[u8; CornersFixed::NUM_ORIENTATION_COORDINATES as usize]>,
}

impl PruningTables {
    pub fn make() -> PruningTables {
        let mut permtab = vec![u8::MAX; CornersFixed::NUM_PERMUTATION_COORDINATES as usize];
        let mut depth = 0;
        let mut entries_filled = 1;
        permtab[0] = 0;
        while entries_filled < permtab.len() {
            for permcoord in 0..CornersFixed::NUM_PERMUTATION_COORDINATES {
                if permtab[permcoord as usize] == depth {
                    let corners = CornersFixed::from_coordinates(permcoord, 0);
                    for face in [Face::U, Face::F, Face::R] {
                        for count in 1..4 {
                            let mut corners = corners;
                            corners.rotate_face(face, count);
                            let new_permcoord = corners.permutation_coordinate();
                            if permtab[new_permcoord as usize] == u8::MAX {
                                permtab[new_permcoord as usize] = depth + 1;
                                entries_filled += 1;
                            }
                        }
                    }
                }
            }
            depth += 1;
        }

        let mut oritab = vec![u8::MAX; CornersFixed::NUM_ORIENTATION_COORDINATES as usize];
        let mut depth = 0;
        let mut entries_filled = 1;
        oritab[0] = 0;
        while entries_filled < oritab.len() {
            for oricoord in 0..CornersFixed::NUM_ORIENTATION_COORDINATES {
                if oritab[oricoord as usize] == depth {
                    let corners = CornersFixed::from_coordinates(0, oricoord);
                    for face in [Face::U, Face::F, Face::R] {
                        for count in 1..4 {
                            let mut corners = corners;
                            corners.rotate_face(face, count);
                            let new_oricoord = corners.orientation_coordinate();
                            if oritab[new_oricoord as usize] == u8::MAX {
                                oritab[new_oricoord as usize] = depth + 1;
                                entries_filled += 1;
                            }
                        }
                    }
                }
            }
            depth += 1;
        }

        PruningTables {
            permutation: permtab.into_boxed_slice().try_into().unwrap(),
            orientation: oritab.into_boxed_slice().try_into().unwrap(),
        }
    }

    fn check(&self, state: &CornerCoordsFixed, limit: u8) -> bool {
        self.orientation[state.orientation as usize] <= limit
            && self.permutation[state.permutation as usize] <= limit
    }
}
