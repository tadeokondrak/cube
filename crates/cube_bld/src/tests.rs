use super::*;
use cube::{CornerOrientation, CornerSticker, Cube, EdgeSticker, Face};
use expect_test::expect;

#[test]
fn real() {
    let mut cube = Cube::new_solved(3);
    cube.rotate_face(Face::U, 3);
    cube.rotate_face(Face::L, 2);
    cube.rotate_face(Face::B, 2);
    cube.rotate_face(Face::D, 2);
    cube.rotate_face(Face::L, 2);
    cube.rotate_face(Face::U, 3);
    cube.rotate_face(Face::F, 2);
    cube.rotate_face(Face::U, 1);
    cube.rotate_face(Face::F, 2);
    cube.rotate_face(Face::B, 1);
    cube.rotate_face(Face::L, 2);
    cube.rotate_face(Face::R, 1);
    cube.rotate_face(Face::F, 2);
    cube.rotate_face(Face::R, 1);
    cube.rotate_face(Face::D, 3);
    cube.rotate_face(Face::B, 3);
    cube.rotate_face(Face::R, 1);
    cube.rotate_face(Face::D, 2);
    cube.rotate_face(Face::R, 2);
    let edge_memo = crate::memo(&cube.edges, EdgeSticker::Uf);
    expect!["Memo { cycles: [[Uf, Ul, Dl], [Uf, Fl, Ub], [Uf, Rb, Bl], [Uf, Ru, Rd], [Uf, Df, Rf]], parity: Some([Uf, Db]), twists: [] }"]
        .assert_eq(&format!("{edge_memo:?}"));
    expect!["Memo { cycles: [[Uf, Ul, Dl], [Uf, Fl, Ub], [Uf, Rb, Bl], [Uf, Ru, Rd], [Uf, Df, Rf]], parity: Some([Uf, Db]), twists: [] }"]
    .assert_eq(&format!("{edge_memo:?}"));
    let corner_memo = crate::memo(&cube.corners, CornerSticker::Ufr);
    expect!["Memo { cycles: [[Ufr, Bur, Fdl], [Ufr, Ful, Ldb], [Ufr, Lub, Rdb]], parity: Some([Ufr, Rdf]), twists: [] }"]
        .assert_eq(&format!("{corner_memo:?}"));
    expect!["Memo { cycles: [[Ufr, Bur, Fdl], [Ufr, Ful, Ldb], [Ufr, Lub, Rdb]], parity: Some([Ufr, Rdf]), twists: [] }"]
    .assert_eq(&format!("{corner_memo:?}"));
}

#[test]
fn test_memo_2() {
    for cube_seed in 0..1024 {
        let edge_buffer = EdgeSticker::Uf;
        let corner_buffer = CornerSticker::Ufr;
        let mut cube = Cube::new_random(3, cube_seed);
        eprintln!("--- seed: {cube_seed} ---");
        eprintln!("{cube:?}");
        let edge_memo = memo(&cube.edges, EdgeSticker::Uf);
        let corner_memo = memo(&cube.corners, CornerSticker::Ufr);
        eprintln!("{edge_memo:?}");
        for cycle in &edge_memo.cycles {
            cube.edges.cycle(cycle, 1);
        }
        if let Some(parity) = &edge_memo.parity {
            cube.edges.cycle(parity, 1);
        }
        for &(piece, _) in &edge_memo.twists {
            cube.edges.orientation[piece.index()] = !cube.edges.orientation[piece.index()];
            cube.edges.orientation[edge_buffer.index()] =
                !cube.edges.orientation[edge_buffer.index()];
        }
        for cycle in &corner_memo.cycles {
            cube.corners.cycle(cycle, 1);
        }
        if let Some(parity) = &corner_memo.parity {
            cube.corners.cycle(parity, 1);
        }
        for &(piece, direction) in &corner_memo.twists {
            cube.corners.cycle(
                &[
                    CornerSticker::from_permutation_and_orientation(piece, direction),
                    CornerSticker::from_permutation_and_orientation(piece, CornerOrientation::Good),
                ],
                1,
            );
            cube.corners.cycle(
                &[
                    CornerSticker::from_permutation_and_orientation(
                        corner_buffer.permutation(),
                        CornerOrientation::Good,
                    ),
                    CornerSticker::from_permutation_and_orientation(
                        corner_buffer.permutation(),
                        direction,
                    ),
                ],
                1,
            );
        }
        assert_eq!(edge_memo.parity.is_some(), corner_memo.parity.is_some());
        assert!(cube.is_solved());
    }
}

#[test]
fn test_memo() {
    {
        let cube = Cube::new_random(3, 0);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect!["Memo { cycles: [[Uf, Bu, Ur], [Uf, Dl, Ul], [Uf, Bd, Bl], [Uf, Rb, Df], [Uf, Fr, Ul], [Uf, Fl, Rd]], parity: Some([Uf, Lf]), twists: [] }"]
            .assert_eq(&format!("{memo:?}"));

        let cube = Cube::new_random(3, 1);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect!["Memo { cycles: [[Uf, Ub, Rd], [Uf, Dl, Rb], [Uf, Ub, Ul], [Uf, Lb, Fr], [Uf, Db, Df]], parity: Some([Uf, Lu]), twists: [(Ur, Bad)] }"]
                .assert_eq(&format!("{memo:?}"));

        let cube = Cube::new_random(3, 2);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect!["Memo { cycles: [[Uf, Rb, Lb], [Uf, Rf, Ur], [Uf, Db, Ld], [Uf, Df, Ub], [Uf, Lu, Fl]], parity: None, twists: [] }"]
                    .assert_eq(&format!("{memo:?}"));

        let cube = Cube::new_random(3, 3);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect!["Memo { cycles: [[Uf, Ld, Bu], [Uf, Ur, Dr], [Uf, Fr, Db], [Uf, Fl, Df]], parity: Some([Uf, Bl]), twists: [(Ul, Bad)] }"]
                                .assert_eq(&format!("{memo:?}"));

        let cube = Cube::new_random(3, 4);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect!["Memo { cycles: [[Uf, Lb, Ub], [Uf, Bd, Br], [Uf, Bu, Fr], [Uf, Fl, Fr], [Uf, Df, Dl], [Uf, Dr, Fd]], parity: None, twists: [] }"]
                    .assert_eq(&format!("{memo:?}"));

        let cube = Cube::new_random(3, 5);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect!["Memo { cycles: [[Uf, Rb, Ur], [Uf, Dr, Ub], [Uf, Fd, Dl], [Uf, Lb, Fr], [Uf, Ub, Ul], [Uf, Lf, Ul]], parity: None, twists: [(Db, Bad)] }"]
                    .assert_eq(&format!("{memo:?}"));
    }

    {
        let mut cube = Cube::new_solved(3);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect![["Memo { cycles: [], parity: None, twists: [] }"]].assert_eq(&format!("{memo:?}"));
        cube.rotate_face(Face::R, 1);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect![
            "Memo { cycles: [[Uf, Ur, Fr], [Uf, Dr, Br]], parity: Some([Uf, Ur]), twists: [] }"
        ]
        .assert_eq(&format!("{memo:?}"));
        cube.rotate_face(Face::R, 1);
        let memo = crate::memo(&cube.edges, EdgeSticker::Uf);
        expect![
            "Memo { cycles: [[Uf, Ur, Dr], [Uf, Ur, Fr], [Uf, Br, Fr]], parity: None, twists: [] }"
        ]
        .assert_eq(&format!("{memo:?}"));
    }
}

#[test]
fn wings_memo() {
    for cube_seed in 0..1024 {
        let mut cube = Cube::new_solved(5);
        cube.layers[0].wings = Cube::new_random(5, cube_seed).layers[0].wings;

        eprintln!("--- seed: {cube_seed} ---");
        eprintln!("{cube:?}");
        let xcenter_memo = crate::memo(&cube.layers[0].wings, WingSticker::Ufr);
        for cycle in &xcenter_memo.cycles {
            cube.layers[0]
                .wings
                .cycle(&cycle.map(|x| x.permutation()), 1);
        }
        eprintln!("{cube:?}");
        if let Some(parity) = &xcenter_memo.parity {
            cube.layers[0]
                .wings
                .cycle(&parity.map(|x| x.permutation()), 1);
        }
        eprintln!("{cube:?}");
        assert!(cube.is_solved());
    }
}

#[test]
fn tcenters_memo() {
    for cube_seed in 0..1024 {
        let mut cube = Cube::new_solved(5);
        cube.layers[0].tcenters = Cube::new_random(5, cube_seed).layers[0].tcenters;

        eprintln!("--- seed: {cube_seed} ---");
        eprintln!("{cube:?}");
        let tcenter_memo = crate::memo_centers(&cube.layers[0].tcenters, EdgeSticker::Uf);

        for cycle in &tcenter_memo.cycles {
            cube.layers[0].tcenters.cycle(cycle, 1);
        }
        eprintln!("{cube:?}");
        if let Some(parity) = &tcenter_memo.parity {
            cube.layers[0].tcenters.cycle(parity, 1);
        }
        eprintln!("{cube:?}");
        assert!(cube.is_solved());
    }
}
