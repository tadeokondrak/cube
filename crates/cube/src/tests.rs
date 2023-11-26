use super::*;
use std::fmt::Write;
use expect_test::{expect, expect_file, Expect, ExpectFile};

#[test]
fn cube_new() {
    for n in 1..=255 {
        Cube::new_solved(n);
    }
}

impl Cube {
    pub(crate) fn expect(&self, expect: Expect) {
        expect.assert_eq(&format!("{self:?}"));
    }
}

#[test]
fn cycle_corners() {
    let mut state = Cube::new_solved(3);
    state.expect(expect!["UUU UUU UUU / LLL LLL LLL / FFF FFF FFF / RRR RRR RRR / BBB BBB BBB / DDD DDD DDD"]);

    state.corners.cycle(&CornerSticker::face_cycle(Face::U), 1);

    state.expect(expect!["UUU UUU UUU / FLF LLL LLL / RFR FFF FFF / BRB RRR RRR / LBL BBB BBB / DDD DDD DDD"]);

    state.corners.cycle(&CornerSticker::face_cycle(Face::R), 1);

    state.expect(expect!["UUR UUU UUF / FLF LLL LLL / RFD FFF FFD / RRB RRR RRB / UBL BBB UBB / DDB DDD DDL"]);
}

#[test]
fn cycle_edges() {
    let mut state = Cube::new_solved(3);
    state.expect(expect!["UUU UUU UUU / LLL LLL LLL / FFF FFF FFF / RRR RRR RRR / BBB BBB BBB / DDD DDD DDD"]);

    state.edges.cycle(&EdgeSticker::face_cycle(Face::U), 1);

    state.expect(expect!["UUU UUU UUU / LFL LLL LLL / FRF FFF FFF / RBR RRR RRR / BLB BBB BBB / DDD DDD DDD"]);

    state.edges.cycle(&EdgeSticker::face_cycle(Face::R), 1);

    state.expect(expect!["UUU UUF UUU / LFL LLL LLL / FRF FFD FFF / RRR RRB RRR / BLB UBB BBB / DDD DDB DDD"]);
}

#[test]
fn cycle_wings() {
    let mut state = Cube::new_solved(7);
    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD"]);

    state.layers[0]
        .wings
        .cycle(&EdgeSticker::face_cycle(Face::U), 1);

    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU / LLFLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFRFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF / RRBRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBLBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD"]);

    state.layers[0].wings.cycle(
        &EdgeSticker::flipped_cycle(EdgeSticker::face_cycle(Face::U)),
        1,
    );

    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU / LLFLFLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFRFRFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF / RRBRBRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBLBLBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD"]);

    state.layers[0]
        .wings
        .cycle(&EdgeSticker::face_cycle(Face::R), 1);

    state.layers[0].wings.cycle(
        &EdgeSticker::flipped_cycle(EdgeSticker::face_cycle(Face::R)),
        1,
    );

    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUF UUUUUUU UUUUUUF UUUUUUU UUUUUUU / LLFLFLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFRFRFF FFFFFFF FFFFFFD FFFFFFF FFFFFFD FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRB RRRRRRR RRRRRRB RRRRRRR RRRRRRR / BBLBLBB BBBBBBB UBBBBBB BBBBBBB UBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDB DDDDDDD DDDDDDB DDDDDDD DDDDDDD"]);
}

#[test]
fn cycle_tcenters_slice() {
    let mut state = Cube::new_solved(7);
    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD"]);

    state.layers[0]
        .tcenters
        .cycle(&EdgeSticker::slice_center_cycle(Face::R), 1);

    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUFUU UUUUUUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFFF FFFFDFF FFFFFFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBBBBBB BBUBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDBDD DDDDDDD DDDDDDD DDDDDDD"]);
}

#[test]
fn cycle_xcenters_slice() {
    let mut state = Cube::new_solved(7);
    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD"]);

    state.layers[0]
        .xcenters
        .cycle(&CornerSticker::slice_center_cycle_lh(Face::R), 1);

    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUFUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFDFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBUBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDBDD DDDDDDD DDDDDDD"]);

    state.layers[0]
        .xcenters
        .cycle(&CornerSticker::slice_center_cycle_rh(Face::R), 1);

    state.expect(expect!["UUUUUUU UUUUUUU UUUUFUU UUUUUUU UUUUFUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFDFF FFFFFFF FFFFDFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBUBBBB BBBBBBB BBUBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDBDD DDDDDDD DDDDBDD DDDDDDD DDDDDDD"]);
}

#[test]
fn cycle_obliques_slice() {
    let mut state = Cube::new_solved(7);
    state.expect(expect!["UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD DDDDDDD"]);

    //state.cycle_obliques(
    //    1,
    //    0,
    //    Handedness::Right,
    //    &EdgeSticker::slice_center_cycle(Face::R),
    //    1,
    //);
    //
    //    state.cycle_obliques(
    //        1,
    //        0,
    //        Handedness::Left,
    //        &dbg!(EdgeSticker::slice_wing_cycle_rh(Face::R)),
    //        1,
    //    );

    state.layers[1].obliques[0]
        .left
        .cycle(&EdgeSticker::slice_center_cycle(Face::R), 1);

    state.layers[1].obliques[0]
        .right
        .cycle(&EdgeSticker::slice_center_cycle(Face::R), 1);

    state.expect(expect!["UUUUUUU UUUUUUU UUUUUFU UUUUUUU UUUUUFU UUUUUUU UUUUUUU / LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL LLLLLLL / FFFFFFF FFFFFFF FFFFFDF FFFFFFF FFFFFDF FFFFFFF FFFFFFF / RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR RRRRRRR / BBBBBBB BBBBBBB BUBBBBB BBBBBBB BUBBBBB BBBBBBB BBBBBBB / DDDDDDD DDDDDDD DDDDDBD DDDDDDD DDDDDBD DDDDDDD DDDDDDD"]);

    //state.cycle_obliques(
    //    1,
    //    0,
    //    Handedness::Right,
    //    &EdgeSticker::slice_center_cycle(Face::R),
    //    1,
    //);

    //state.expect(expect![[r#"
    //           UUUUUUU
    //           UUUUFUU
    //           UUUUUFU
    //           UUUUUUU
    //           UUUUUUU
    //           UUUUUUU
    //           UUUUUUU
    //    LLLLLLLFFFFFFFRRRRRRRBBBBBBB
    //    LLLLLLLFFFFDFFRRRRRRRBBBBBBB
    //    LLLLLLLFFFFFDFRRRRRRRBBBBBBB
    //    LLLLLLLFFFFFFFRRRRRRRBBBBBBB
    //    LLLLLLLFFFFFFFRRRRRRRBUBBBBB
    //    LLLLLLLFFFFFFFRRRRRRRBBUBBBB
    //    LLLLLLLFFFFFFFRRRRRRRBBBBBBB
    //           DDDDDDD
    //           DDDDBDD
    //           DDDDDBD
    //           DDDDDDD
    //           DDDDDDD
    //           DDDDDDD
    //           DDDDDDD              "#]]);
}

#[test]
fn anysticker_wings() {
    let mut actual = String::new();
    for face_index in 0..6 {
        let face = Face::from_index(face_index);
        for y in [2i16, 1, -1, -2] {
            for x in [-2i16, -1, 1, 2] {
                if x.abs() == y.abs() {
                    continue;
                }
                write!(&mut actual, "{:?} ", AnySticker::at(5, face, x, y)).unwrap();
            }
        }
        writeln!(&mut actual).unwrap();
    }
    expect![[r#"
        Wing(0, Ubl) Wing(0, Ubr) Wing(0, Ulb) Wing(0, Urb) Wing(0, Ulf) Wing(0, Urf) Wing(0, Ufl) Wing(0, Ufr) 
        Wing(0, Lub) Wing(0, Luf) Wing(0, Lbu) Wing(0, Lfu) Wing(0, Lbd) Wing(0, Lfd) Wing(0, Ldb) Wing(0, Ldf) 
        Wing(0, Ful) Wing(0, Fur) Wing(0, Flu) Wing(0, Fru) Wing(0, Fld) Wing(0, Frd) Wing(0, Fdl) Wing(0, Fdr) 
        Wing(0, Ruf) Wing(0, Rub) Wing(0, Rfu) Wing(0, Rbu) Wing(0, Rfd) Wing(0, Rbd) Wing(0, Rdf) Wing(0, Rdb) 
        Wing(0, Bur) Wing(0, Bul) Wing(0, Bru) Wing(0, Blu) Wing(0, Brd) Wing(0, Bld) Wing(0, Bdr) Wing(0, Bdl) 
        Wing(0, Dfl) Wing(0, Dfr) Wing(0, Dlf) Wing(0, Drf) Wing(0, Dlb) Wing(0, Drb) Wing(0, Dbl) Wing(0, Dbr) 
    "#]]
    .assert_eq(&actual);
}

#[test]
fn anysticker_at() {
    fn check_n(n: u16, expect: ExpectFile) {
        let mut actual = String::new();
        for face_index in 0..6 {
            let face = Face::from_index(face_index);
            for y in 0..(n | 1) {
                for x in 0..(n | 1) {
                    let adj_x = x as i16 - n as i16 / 2;
                    let adj_y = -(y as i16 - n as i16 / 2);
                    if n % 2 == 0 && (adj_x == 0 || adj_y == 0) {
                        continue;
                    }
                    writeln!(
                        &mut actual,
                        "{face:?} ({x}, {y}) -> ({adj_x}, {adj_y}) -> {:?}",
                        AnySticker::at(n, face, adj_x, adj_y)
                    )
                    .unwrap();
                }
            }
        }
        expect.assert_eq(&actual)
    }

    check_n(1, expect_file!["../testdata/anysticker/1x1x1.txt"]);
    check_n(2, expect_file!["../testdata/anysticker/2x2x2.txt"]);
    check_n(3, expect_file!["../testdata/anysticker/3x3x3.txt"]);
    check_n(4, expect_file!["../testdata/anysticker/4x4x4.txt"]);
    check_n(5, expect_file!["../testdata/anysticker/5x5x5.txt"]);
    check_n(6, expect_file!["../testdata/anysticker/6x6x6.txt"]);
    check_n(7, expect_file!["../testdata/anysticker/7x7x7.txt"]);
    check_n(8, expect_file!["../testdata/anysticker/8x8x8.txt"]);
    check_n(9, expect_file!["../testdata/anysticker/9x9x9.txt"]);
}

#[test]
fn cube_misc() {
    let mut state = Cube::new_solved(9);
    assert_eq!(state.edges.at(EdgeSticker::Ul), EdgeSticker::Ul);
    assert_eq!(EdgeSticker::Ul.color(), Face::U);
    assert_eq!(state.corners.at(CornerSticker::Ldb), CornerSticker::Ldb);

    expect![[r#"
        UUUUUUUUU UUUUUUUUU UUUUUUUUU UUUUUUUUU UUUUUUUUU UUUUUUUUU UUUUUUUUU UUUUUUUUU UUUUUUUUU / LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL / FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF / RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR / BBBBBBBBB BBBBBBBBB BBBBBBBBB BBBBBBBBB BBBBBBBBB BBBBBBBBB BBBBBBBBB BBBBBBBBB BBBBBBBBB / DDDDDDDDD DDDDDDDDD DDDDDDDDD DDDDDDDDD DDDDDDDDD DDDDDDDDD DDDDDDDDD DDDDDDDDD DDDDDDDDD
    "#]]
    .assert_debug_eq(&state);

    assert_eq!(state.layers[0].wings.at(WingSticker::Ufl), WingSticker::Ufl);
    assert_eq!(state.layers[0].wings.at(WingSticker::Ufr), WingSticker::Ufr);

    assert_eq!(state.layers[0].wings.at(WingSticker::Ful), WingSticker::Ful);
    assert_eq!(state.layers[0].wings.at(WingSticker::Fur), WingSticker::Fur);

    state.rotate_slice(Face::R, 1, 1);
    state.rotate_slice(Face::R, 2, 1);

    expect![[r#"
        UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU / LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL LLLLLLLLL / FFFFFDDFF FFFFFDDFF FFFFFDDFF FFFFFDDFF FFFFFDDFF FFFFFDDFF FFFFFDDFF FFFFFDDFF FFFFFDDFF / RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR RRRRRRRRR / BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB / DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD
    "#]]
    .assert_debug_eq(&state);

    assert_eq!(
        AnySticker::at(5, Face::U, -1, -2),
        AnySticker::Wing(0, WingSticker::Ufl)
    );
    assert_eq!(
        AnySticker::at(5, Face::U, 1, -2),
        AnySticker::Wing(0, WingSticker::Ufr)
    );
    assert_eq!(state.layers[0].wings.at(WingSticker::Ufl), WingSticker::Ufl);

    //assert_eq!(state.layers[0].wings.at( WingSticker::Ufr), WingSticker::Fd);

    assert_eq!(state.layers[0].wings.at(WingSticker::Ful), WingSticker::Ful);
    //assert_eq!(state.layers[0].wings.at( WingSticker::Fur), WingSticker::Df);

    state.rotate_face(Face::F, 1);

    expect![[r#"
        UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU UUUUUFFUU LLLLLLLLL / LLLLLLLLD LLLLLLLLD LLLLLLLLD LLLLLLLLD LLLLLLLLD LLLLLLLLB LLLLLLLLB LLLLLLLLD LLLLLLLLD / FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF FFFFFFFFF DDDDDDDDD DDDDDDDDD FFFFFFFFF FFFFFFFFF / URRRRRRRR URRRRRRRR URRRRRRRR URRRRRRRR URRRRRRRR FRRRRRRRR FRRRRRRRR URRRRRRRR URRRRRRRR / BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB BBUUBBBBB / RRRRRRRRR DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD DDDDDBBDD
    "#]]
    .assert_debug_eq(&state);
}

#[test]
fn color_at() {
    for n in 3..9 {
        let mut state = Cube::new_solved(n);
        for face_index in 0..6 {
            let face = Face::from_index(face_index);
            for x in -(n as i16 / 2)..=(n as i16 / 2) {
                for y in -(n as i16 / 2)..=(n as i16 / 2) {
                    if n % 2 == 0 && (x == 0 || y == 0) {
                        continue;
                    }
                    assert_eq!(state.color_at(face, x, y), face);
                }
            }
            state.rotate_face(face, 1);
            for x in -(n as i16 / 2)..(n as i16 / 2) {
                for y in -(n as i16 / 2)..(n as i16 / 2) {
                    if n % 2 == 0 && (x == 0 || y == 0) {
                        continue;
                    }
                    assert_eq!(state.color_at(face, x, y), face);
                }
            }
            state.rotate_face(face, 1);
            for x in -(n as i16 / 2)..(n as i16 / 2) {
                for y in -(n as i16 / 2)..(n as i16 / 2) {
                    if n % 2 == 0 && (x == 0 || y == 0) {
                        continue;
                    }
                    assert_eq!(state.color_at(face, x, y), face);
                }
            }
            state.rotate_face(face, 2);
        }
    }
}

#[test]
fn rotate_2x2x2() {
    let mut state = Cube::new_solved(9);
    expect!["[Ubl, Ubr, Ufr, Ufl, Dfl, Dfr, Dbr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    expect!["[Good, Good, Good, Good, Good, Good, Good, Good]"]
        .assert_eq(&format!("{:?}", state.corners.orientation));
    state.rotate_face(Face::U, 1);
    expect!["[Good, Good, Good, Good, Good, Good, Good, Good]"]
        .assert_eq(&format!("{:?}", state.corners.orientation));
    expect!["[Ufl, Ubl, Ubr, Ufr, Dfl, Dfr, Dbr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    state.rotate_face(Face::U, 1);
    state.rotate_face(Face::U, 1);
    state.rotate_face(Face::U, 1);
    expect!["[Ubl, Ubr, Ufr, Ufl, Dfl, Dfr, Dbr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    state.rotate_face(Face::U, 2);
    expect!["[Ufr, Ufl, Ubl, Ubr, Dfl, Dfr, Dbr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    state.rotate_face(Face::U, 2);
    expect!["[Ubl, Ubr, Ufr, Ufl, Dfl, Dfr, Dbr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    state.rotate_face(Face::R, 1);
    expect!["[Ubl, Ufr, Dfr, Ufl, Dfl, Dbr, Ubr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    expect!["[Good, BadCcw, BadCw, Good, Good, BadCcw, BadCw, Good]"]
        .assert_eq(&format!("{:?}", state.corners.orientation));
    state.rotate_face(Face::R, 1);
    expect!["[Ubl, Dfr, Dbr, Ufl, Dfl, Ubr, Ufr, Dbl]"]
        .assert_eq(&format!("{:?}", state.corners.permutation));
    expect!["[Good, Good, Good, Good, Good, Good, Good, Good]"]
        .assert_eq(&format!("{:?}", state.corners.orientation));
}

#[test]
fn rotate() {
    let mut s = String::new();
    for face in Face::ALL {
        for axis in Axis::ALL {
            for count in 0..4 {
                writeln!(
                    &mut s,
                    "Face {face:?} rotated on the {axis:?} axis {count:?} times: {:?}",
                    rotate_face(face, axis, count)
                )
                .unwrap();
            }
        }
    }
    expect_file!["../testdata/rotate.txt"].assert_eq(&s);
}

#[test]
fn rotate_orientation() {
    let mut s = String::new();
    for ori in EdgeSticker::SOLVED {
        for face in Face::ALL {
            for count in 0..4 {
                writeln!(
                    &mut s,
                    "Orientation {ori:?} rotated on the {face:?} axis {count:?} times: {:?}",
                    orientation_after_move(3, ori, face, 0..3, count)
                )
                .unwrap();
            }
        }
    }
    expect_file!["../testdata/rotate_orientation.txt"].assert_eq(&s);
}

#[test]
fn slice_moves_3x3x3() {
    let mut state = Cube::new_solved(3);
    state.rotate_slice(Face::R, 0, 1);
    state.expect(expect!["BUB BUB BUB / LLL LLL LLL / UFU UFU UFU / RRR RRR RRR / DBD DBD DBD / FDF FDF FDF"]);

    let mut state = Cube::new_solved(3);
    state.rotate_slice(Face::U, 0, 1);
    state.expect(expect!["UUU UUU UUU / BBB LLL BBB / LLL FFF LLL / FFF RRR FFF / RRR BBB RRR / DDD DDD DDD"]);

    let mut state = Cube::new_solved(3);
    state.rotate_slice(Face::F, 0, 1);
    state.expect(expect!["RRR UUU RRR / ULU ULU ULU / FFF FFF FFF / DRD DRD DRD / BBB BBB BBB / LLL DDD LLL"]);
}

#[test]
fn wide_moves_3x3x3() {
    let mut state = Cube::new_solved(3);
    state.rotate(Face::R, 0..2, 1);
    state.expect(expect!["BUU BUU BUU / LLL LLL LLL / UFF UFF UFF / RRR RRR RRR / BBD BBD BBD / FDD FDD FDD"]);

    let mut state = Cube::new_solved(3);
    state.rotate(Face::U, 0..2, 1);
    state.expect(expect!["UUU UUU UUU / LLL LLL BBB / FFF FFF LLL / RRR RRR FFF / BBB BBB RRR / DDD DDD DDD"]);

    let mut state = Cube::new_solved(3);
    state.rotate(Face::F, 0..2, 1);
    state.expect(expect!["RRR UUU UUU / ULL ULL ULL / FFF FFF FFF / RRD RRD RRD / BBB BBB BBB / DDD DDD LLL"]);
}

#[test]
fn test_orientation_after_move() {
    assert_eq!(orientation_after_move(5, EdgeSticker::Uf, Face::R, 0..3, 1), EdgeSticker::Fd);
    assert_eq!(orientation_after_move(4, EdgeSticker::Uf, Face::R, 0..3, 1), EdgeSticker::Uf);
    assert_eq!(orientation_after_move(4, EdgeSticker::Uf, Face::R, 0..2, 1), EdgeSticker::Uf);
}

#[test]
fn test_rotate_then_wide_move() {
    let mut cube = Cube::new_solved(3);
    let mut cube = RotatedCube::new(&mut cube);
    assert_eq!(cube.orientation, EdgeSticker::Uf);
    cube.rotate(Face::U, 0..3, 1); // y
    assert_eq!(cube.orientation, EdgeSticker::Ur);
    cube.rotate(Face::R, 0..2, 1); // Rw
    assert_eq!(cube.orientation, EdgeSticker::Rd);
    cube.cube.expect(expect!["UUU UUU LLL / LLD LLD LLD / FFF FFF FFF / URR URR URR / BBB BBB BBB / RRR DDD DDD"]);

}
