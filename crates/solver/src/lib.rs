use cube::{Corners, Face};
use cube_notation::{Move, ParseMode, Token};
use std::time::Instant;

pub fn solve(scramble: &str, pruning: &[u8]) -> String {
    let mut corners = Corners::new();
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

fn solve2(corners: Corners, pruning: &[u8]) -> Vec<Move> {
    fn go(corners: Corners, moves: &mut Vec<Move>, pruning: &[u8]) -> bool {
        let old_index = corners.coordinate() as usize;

        if old_index == 0 {
            return true;
        }

        for face in [Face::U, Face::R, Face::F, Face::L, Face::B, Face::D] {
            if moves.last().is_some_and(|mv| mv.face == face) {
                continue;
            }
            if moves.last().is_some_and(|mv| mv.face == face.opposite()) && face < face.opposite() {
                continue;
            }
            for count in 1..4 {
                let mut corners = corners;
                corners.rotate_face(face, count);

                let index = corners.coordinate() as usize;

                if pruning[index] >= pruning[old_index] {
                    continue;
                }

                moves.push(Move {
                    n: 2,
                    face,
                    start: 0,
                    end: 1,
                    count,
                });

                if go(corners, moves, pruning) {
                    return true;
                } else {
                    moves.pop();
                }
            }
        }

        false
    }

    let mut moves = Vec::new();
    assert!(go(corners, &mut moves, pruning));
    moves
}

pub fn make_pruning_table() -> Vec<u8> {
    let mut table = vec![u8::MAX; Corners::NUM_COORDINATES as usize];
    let mut depth = 0;
    let mut entries_filled = 1;
    table[0] = 0;
    let mut start = Instant::now();
    while entries_filled < table.len() {
        for coord in 0..Corners::NUM_COORDINATES {
            if table[coord as usize] == depth {
                let corners = Corners::from_coordinate(coord);
                for face in Face::ALL {
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
        eprintln!("{depth} {entries_filled}/{} {:?}", table.len(), start.elapsed());
        start = Instant::now();
        depth += 1;
    }
    table
}

pub fn get_pruning_table() -> Vec<u8> {
    match std::fs::read("2x2pruning.bin") {
        Ok(table) => table,
        Err(_) => {
            let table = make_pruning_table();
            std::fs::write("2x2pruning.bin", &table).unwrap();
            table
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cube::Cube;

    #[test]
    #[ignore = "pruning table is slow to generate"]
    fn solve_yperm() {
        let pruning = get_pruning_table();
        let start = std::time::Instant::now();
        assert_eq!(
            solve("F R U' R' U' R U R' F' R U R' U' R' F R F'", &pruning),
            "R U' R2 U' F' R' U F2 R U' F"
        );
        eprintln!("{:?}", start.elapsed());
    }

    #[test]
    #[ignore = "pruning table is slow to generate"]
    fn solve_random_states() {
        let pruning = get_pruning_table();
        for i in 0..1024 {
            eprintln!("solved {i} cubes so far");
            let mut cube = Cube::new_random(2, i);
            for mov in solve2(cube.corners, &pruning) {
                //eprintln!("{mov:?}");
                cube.corners.rotate_face(mov.face, mov.count);
            }
            assert!(cube.corners.are_solved());
        }
    }
}
