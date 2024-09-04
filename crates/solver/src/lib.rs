use cube::{CornersFixed, Face};
use cube_notation::{Move, ParseMode, Token};
use std::time::Instant;

pub fn solve(scramble: &str, pruning: &[u8]) -> String {
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

fn solve2(corners: CornersFixed, pruning: &[u8]) -> Vec<Move> {
    fn go(corners: CornersFixed, moves: &mut Vec<Move>, pruning: &[u8]) -> bool {
        let old_index = corners.coordinate() as usize;

        if old_index == 0 {
            return true;
        }

        for face in [Face::U, Face::F, Face::R] {
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
    let mut table = vec![u8::MAX; CornersFixed::NUM_COORDINATES as usize];
    let mut depth = 0;
    let mut entries_filled = 1;
    table[0] = 0;
    let mut start = Instant::now();
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
        eprintln!(
            "{depth} {entries_filled}/{} {:?}",
            table.len(),
            start.elapsed()
        );
        start = Instant::now();
        depth += 1;
    }
    table
}

pub fn get_pruning_table() -> Vec<u8> {
    // TODO: use OnceLock
    make_pruning_table()
}

#[cfg(test)]
mod tests {
    use super::*;
    use oorandom::Rand32;

    #[test]
    fn solve_yperm() {
        let pruning = get_pruning_table();
        let start = std::time::Instant::now();
        assert_eq!(
            solve("F R U' R' U' R U R' F' R U R' U' R' F R F'", &pruning),
            "F2 U' F U R U2 R U' R' F R'"
        );
        eprintln!("{:?}", start.elapsed());
    }

    #[test]
    fn solve_random_states() {
        let pruning = get_pruning_table();
        for i in 0..1024 {
            eprintln!("solved {i} cubes so far");
            let mut rand = Rand32::new(i);
            let mut corners =
                CornersFixed::from_coordinate(rand.rand_range(0..CornersFixed::NUM_COORDINATES));
            assert!(!corners.are_solved());
            for mov in solve2(corners, &pruning) {
                eprintln!("{mov:?}");
                corners.rotate_face(mov.face, mov.count);
            }
            assert!(corners.are_solved());
        }
    }
}
