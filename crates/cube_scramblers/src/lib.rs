use cube::{CornerCoordsMoveTableFixed, Corners, CornersFixed, Face};
use cube_notation::{format_moves, Canceler, Move};
use oorandom::Rand32;

mod solve222;

fn scramble_nnn_random_moves_length(n: u16) -> usize {
    match n {
        0 | 1 => 0,
        2 | 3 => 25,
        _ => 40 + (usize::from(n) - 4) * 20,
    }
}

pub fn scramble_nnn_random_moves(n: u16, seed: u64) -> String {
    let mut rand = Rand32::new(seed);
    let mut canceler = Canceler::new();
    let length = scramble_nnn_random_moves_length(n);
    while canceler.moves.len() < length {
        let mv = Move {
            n,
            face: Face::from_index(rand.rand_range(0..6) as usize),
            start: 0,
            end: rand.rand_range(1..(n / 2 + 1) as u32) as u16,
            count: rand.rand_range(0..3) as u8,
        };
        canceler.cancel(mv);
    }
    format_moves(&canceler.moves)
}

pub fn scramble_222_random_state(seed: u64) -> String {
    let pruntab = solve222::PruningTables::make();
    let movetab = CornerCoordsMoveTableFixed::new();
    let corners =
        CornersFixed::from_coordinate(Rand32::new(seed).rand_range(0..Corners::NUM_COORDINATES));
    let moves = solve222::solve_state(corners.into(), &pruntab, &movetab);
    format_moves(&moves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    #[test]
    fn random_state_222() {
        expect!["U R2 F U2 F R' U2 F R U"].assert_eq(&scramble_222_random_state(0));
    }

    #[test]
    fn random_moves() {
        fn check(n: u16, expect: Expect) {
            expect.assert_eq(&scramble_nnn_random_moves(n, 0));
        }
        check(
            2,
            expect!["R' F2 R F2 U R2 F U' F2 R U2 R' U F2 U2 R2 F U2 R2 F U2 F R' F U"],
        );
        check(
            3,
            expect!["R L2 B2 L F2 U R2 F D2 U B2 R D2 R2 L U B2 D2 R2 B2 F' D2 R L B"],
        );
        check(4, expect!["Rw R2 Fw2 L F2 U Rw2 Fw D2 Uw B2 R Uw2 Rw2 L U B2 Uw2 R2 B2 Fw F2 D2 R L Fw D2 Fw Rw' Fw U L D2 U Fw2 Rw2 D Rw Uw2 L"]);
        check(5, expect!["Rw' R2 Lw2 Bw2 L F2 U Rw2 Fw D2 Uw B2 R Dw2 Rw2 L U B2 Dw2 R2 B2 Fw F2 D2 R L Bw D2 Bw Rw Lw2 Fw U L D2 U Fw2 Rw2 D Lw Dw2 Rw L Uw Rw2 L2 F R2 Bw2 D U R Dw Rw Dw Uw2 Rw2 Bw U2 B"]);
        check(6, expect!["3Rw2 Rw2 Dw2 D2 Rw' Bw2 L Fw2 U Rw2 3Fw D2 3Uw B2 R Dw2 3Rw2 L Uw B2 3Uw2 Rw2 Bw2 Fw F2 D2 3Rw2 Rw Lw2 L 3Fw D2 3Fw 3Rw' 3Fw Uw Lw D2 Uw 3Fw2 3Rw2 D Lw Dw 3Uw Rw L Uw Rw2 L2 Fw R2 3Fw2 Dw U R 3Uw Rw 3Uw' 3Rw2 3Fw Dw2 D2 U2 Bw B2 D 3Uw2 3Rw U2 3Fw' L2 3Uw2 F2 Rw B 3Uw' 3Fw2 Dw Uw"]);
        check(7, expect!["Rw2 3Lw2 Dw2 D2 Rw' Bw2 L Fw2 U Rw2 3Fw D2 3Uw B2 R Dw2 3Rw2 L Uw B2 3Dw2 Rw2 Bw2 Fw F2 D2 Rw 3Lw2 Lw2 L 3Bw D2 3Bw 3Rw 3Lw2 3Fw Uw Lw D2 Uw 3Fw2 3Rw2 D Lw 3Dw Dw Rw L Uw Rw2 L2 Fw R2 3Bw2 Dw U R 3Dw Rw 3Dw 3Uw2 3Rw2 3Bw Dw2 D2 U2 Bw B2 D 3Uw2 3Lw U2 3Bw' L2 3Uw2 F2 Rw B 3Dw2 3Bw2 3Fw2 3Dw 3Bw2 Dw Uw Bw2 3Rw2 3Bw Fw2 R2 Lw2 3Bw' B2 Fw2 F' R Lw 3Fw2 F2 Rw"]);
    }
}
