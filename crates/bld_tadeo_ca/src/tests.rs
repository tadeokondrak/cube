use super::*;
use expect_test::expect;

#[test]
fn alg() {
    let mut cube = Cube::new_solved(3);
    apply_alg(&mut cube, "3Rw");
    expect!["UUU UUU UUU / LLL LLL LLL / FFF FFF FFF / RRR RRR RRR / BBB BBB BBB / DDD DDD DDD"]
    .assert_eq(&format!("{cube:?}"));
}

#[test]
fn memorize_test() {
    expect![[r#"
        Edges: BA
        Corners: "#]]
    .assert_eq(&memorize(
        3,
        "R2 U' S R2 S' R2 U R2",
        "ABCDEFGHIJKLMNOPQRSTUVWX",
        2,
        2,
        17,
        2,
        2,
        2,
        2,
    ));
    expect![[r#"
        Edges: VL GW MQ; flip DJU
        Corners: TP AD HD; twist NL"#]]
    .assert_eq(&memorize(
        3,
        "R' F2 L D2 L D2 L U2 L' B2 L U' F L2 F2 D' R2 B' R' F R' Rw'",
        "ABCDEFGHIJKLMNOPQRSTUVWX",
        2,
        2,
        17,
        2,
        2,
        2,
        2,
    ));
    expect![[r#"
        Corners: KW QA IU SA

        Layer 1
        X-centers: IU QA JV RB KW SD LX T
        Wings: KW QM PO NM IU SA IE FG HE DL XR DB JV TB"#]]
    .assert_eq(&memorize(
        4,
        "F Fw' Lw' R r u",
        "ABCDEFGHIJKLMNOPQRSTUVWX",
        2,
        2,
        17,
        2,
        2,
        2,
        2,
    ));
    expect![[r#"
        Corners: VH OA IN R; twist L

        Layer 1
        X-centers: EW FM IG NR DO JH PK LT XL
        Wings: TU LE QF VJ WI GX RM DN KP OH PB SA B"#]]
    .assert_eq(&memorize(
        4,
        "F L' F B' U' D' L F2 D' F2 B2 R' F2 L U2 L2 F2 L F2 D2 Rw2 F' Uw2 F' D Fw2 R2 U2 Rw2 F D' F R Rw2 D2 Rw' D2 L' Rw2 Uw F2 Uw Fw' D R2 Fw'",
        "ABCDEFGHIJKLMNOPQRSTUVWX",
        2,
        2,
        17,
        2,
        2,
        2,
        2,
    ));
}
