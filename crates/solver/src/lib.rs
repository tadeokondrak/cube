use cube::{CornersFixed, Face};
use cube_notation::{Move, ParseMode, Token};

pub fn solve(scramble: &str, pruning: &impl PruningTable) -> String {
    let mut corners = CornersFixed::new();
    if let Ok(tree) = cube_notation::parse_alg(2, ParseMode::Wca, scramble) {
        for mv in tree.to_canonical_moves() {
            assert_eq!(mv.start, 0);
            assert_eq!(mv.end, 1);
            corners.rotate_face(mv.face, mv.count)
        }
    }
    let moves = solve2(corners, pruning);
    let tokens = moves.iter().copied().map(Token::Move).collect::<Vec<_>>();
    let mut alg = String::new();
    cube_notation::format_tokens(&mut alg, &tokens).unwrap();
    alg
}

fn solve2(corners: CornersFixed, pruning: &impl PruningTable) -> Vec<Move> {
    fn go(
        corners: CornersFixed,
        moves: &mut Vec<Move>,
        pruning: &impl PruningTable,
        moves_left: u8,
    ) -> bool {
        if corners.are_solved() {
            return true;
        }

        if moves_left == 0 {
            return false;
        }

        for face in [Face::U, Face::F, Face::R] {
            if moves.last().is_some_and(|mv| mv.face == face) {
                continue;
            }
            if moves.last().is_some_and(|mv| mv.face == face.opposite()) && face < face.opposite() {
                continue;
            }
            for count in 1..4 {
                let mut new_corners = corners;
                new_corners.rotate_face(face, count);

                if !pruning.check(&new_corners, moves_left) {
                    continue;
                }

                moves.push(Move {
                    n: 2,
                    face,
                    start: 0,
                    end: 1,
                    count,
                });

                if go(new_corners, moves, pruning, moves_left - 1) {
                    return true;
                } else {
                    moves.pop();
                }
            }
        }

        false
    }

    let mut moves = Vec::new();
    if go(corners, &mut moves, pruning, 11) {
        return moves;
    }
    panic!("no solution found")
}

pub trait PruningTable {
    fn check(&self, state: &CornersFixed, limit: u8) -> bool;
}

impl PruningTable for PruningTablesOptimal {
    fn check(&self, state: &CornersFixed, limit: u8) -> bool {
        self.table[state.coordinate() as usize] <= limit
    }
}

impl PruningTable for PruningTablesFast {
    fn check(&self, state: &CornersFixed, limit: u8) -> bool {
        self.orientation[state.orientation_coordinate() as usize] <= limit
            && self.permutation[state.permutation_coordinate() as usize] <= limit
    }
}

pub struct PruningTablesOptimal {
    table: Box<[u8; CornersFixed::NUM_COORDINATES as usize]>,
}

pub struct PruningTablesFast {
    permutation: Box<[u8; CornersFixed::NUM_PERMUTATION_COORDINATES as usize]>,
    orientation: Box<[u8; CornersFixed::NUM_ORIENTATION_COORDINATES as usize]>,
}

impl PruningTablesFast {
    pub fn make() -> PruningTablesFast {
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

        PruningTablesFast {
            permutation: permtab.into_boxed_slice().try_into().unwrap(),
            orientation: oritab.into_boxed_slice().try_into().unwrap(),
        }
    }
}

impl PruningTablesOptimal {
    pub fn make() -> PruningTablesOptimal {
        let mut table = vec![u8::MAX; CornersFixed::NUM_COORDINATES as usize];
        let mut depth = 0;
        let mut entries_filled = 1;
        table[0] = 0;
        while entries_filled < table.len() {
            for coord in 0..CornersFixed::NUM_COORDINATES {
                if table[coord as usize] == depth {
                    let corners = CornersFixed::from_coordinate(coord);
                    for face in [Face::U, Face::F, Face::R] {
                        for count in 1..4 {
                            let mut corners = corners;
                            corners.rotate_face(face, count);
                            let new_coord = corners.coordinate();
                            if table[new_coord as usize] == u8::MAX {
                                table[new_coord as usize] = depth + 1;
                                entries_filled += 1;
                            }
                        }
                    }
                }
            }
            depth += 1;
        }
        PruningTablesOptimal {
            table: table.into_boxed_slice().try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oorandom::Rand32;

    #[test]
    fn solve_yperm() {
        let pruning = PruningTablesFast::make();
        let start = std::time::Instant::now();
        assert_eq!(
            solve("F R U' R' U' R U R' F' R U R' U' R' F R F'", &pruning),
            "F2 U' F U R U2 R U' R' F R'"
        );
        eprintln!("{:?}", start.elapsed());
    }

    #[test]
    fn solve_random_states() {
        let pruning = PruningTablesFast::make();
        for i in 0..1024 {
            //eprintln!("solved {i} cubes so far");
            let mut rand = Rand32::new(i);
            let mut corners =
                CornersFixed::from_coordinate(rand.rand_range(0..CornersFixed::NUM_COORDINATES));
            assert!(!corners.are_solved());
            for mov in solve2(corners, &pruning) {
                eprint!("{mov:?} ");
                corners.rotate_face(mov.face, mov.count);
            }
            eprintln!();
            assert!(corners.are_solved());
        }
    }
}
