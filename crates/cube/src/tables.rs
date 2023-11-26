use crate::{
    CornerOrientation, CornerPermutation, CornerSticker, EdgeOrientation, EdgePermutation,
    EdgeSticker, Face, WingSticker,
};

impl EdgeSticker {
    pub const SOLVED: [EdgeSticker; 24] = [
        EdgeSticker::Ub,
        EdgeSticker::Ur,
        EdgeSticker::Uf,
        EdgeSticker::Ul,
        EdgeSticker::Lu,
        EdgeSticker::Lf,
        EdgeSticker::Ld,
        EdgeSticker::Lb,
        EdgeSticker::Fu,
        EdgeSticker::Fr,
        EdgeSticker::Fd,
        EdgeSticker::Fl,
        EdgeSticker::Ru,
        EdgeSticker::Rb,
        EdgeSticker::Rd,
        EdgeSticker::Rf,
        EdgeSticker::Bu,
        EdgeSticker::Bl,
        EdgeSticker::Bd,
        EdgeSticker::Br,
        EdgeSticker::Df,
        EdgeSticker::Dr,
        EdgeSticker::Db,
        EdgeSticker::Dl,
    ];

    pub const PERMUTATIONS: [EdgePermutation; 24] = [
        EdgePermutation::Ub,
        EdgePermutation::Ur,
        EdgePermutation::Uf,
        EdgePermutation::Ul,
        EdgePermutation::Ul,
        EdgePermutation::Fl,
        EdgePermutation::Dl,
        EdgePermutation::Bl,
        EdgePermutation::Uf,
        EdgePermutation::Fr,
        EdgePermutation::Df,
        EdgePermutation::Fl,
        EdgePermutation::Ur,
        EdgePermutation::Br,
        EdgePermutation::Dr,
        EdgePermutation::Fr,
        EdgePermutation::Ub,
        EdgePermutation::Bl,
        EdgePermutation::Db,
        EdgePermutation::Br,
        EdgePermutation::Df,
        EdgePermutation::Dr,
        EdgePermutation::Db,
        EdgePermutation::Dl,
    ];

    pub const ORIENTATIONS: [EdgeOrientation; 24] = [
        EdgeOrientation::Good,
        EdgeOrientation::Good,
        EdgeOrientation::Good,
        EdgeOrientation::Good,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Good,
        EdgeOrientation::Bad,
        EdgeOrientation::Good,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Bad,
        EdgeOrientation::Good,
        EdgeOrientation::Bad,
        EdgeOrientation::Good,
        EdgeOrientation::Good,
        EdgeOrientation::Good,
        EdgeOrientation::Good,
        EdgeOrientation::Good,
    ];

    /// Arbitrary edge sticker used in tables
    const XX: EdgeSticker = EdgeSticker::Ub;

    #[rustfmt::skip]
    pub const BY_FACES: [[EdgeSticker; 6]; 6] = [
        [EdgeSticker::XX, EdgeSticker::Ul, EdgeSticker::Uf, EdgeSticker::Ur, EdgeSticker::Ub, EdgeSticker::XX,],
        [EdgeSticker::Lu, EdgeSticker::XX, EdgeSticker::Lf, EdgeSticker::XX, EdgeSticker::Lb, EdgeSticker::Ld,],
        [EdgeSticker::Fu, EdgeSticker::Fl, EdgeSticker::XX, EdgeSticker::Fr, EdgeSticker::XX, EdgeSticker::Fd,],
        [EdgeSticker::Ru, EdgeSticker::XX, EdgeSticker::Rf, EdgeSticker::XX, EdgeSticker::Rb, EdgeSticker::Rd,],
        [EdgeSticker::Bu, EdgeSticker::Bl, EdgeSticker::XX, EdgeSticker::Br, EdgeSticker::XX, EdgeSticker::Bd,],
        [EdgeSticker::XX, EdgeSticker::Dl, EdgeSticker::Df, EdgeSticker::Dr, EdgeSticker::Db, EdgeSticker::XX,],
    ];
}

impl CornerSticker {
    pub const SOLVED: [CornerSticker; 24] = [
        CornerSticker::Ubl,
        CornerSticker::Ubr,
        CornerSticker::Ufr,
        CornerSticker::Ufl,
        CornerSticker::Lub,
        CornerSticker::Luf,
        CornerSticker::Ldf,
        CornerSticker::Ldb,
        CornerSticker::Ful,
        CornerSticker::Fur,
        CornerSticker::Fdr,
        CornerSticker::Fdl,
        CornerSticker::Ruf,
        CornerSticker::Rub,
        CornerSticker::Rdb,
        CornerSticker::Rdf,
        CornerSticker::Bur,
        CornerSticker::Bul,
        CornerSticker::Bdl,
        CornerSticker::Bdr,
        CornerSticker::Dfl,
        CornerSticker::Dfr,
        CornerSticker::Dbr,
        CornerSticker::Dbl,
    ];

    pub const PERMUTATIONS: [CornerPermutation; 24] = [
        CornerPermutation::Ubl,
        CornerPermutation::Ubr,
        CornerPermutation::Ufr,
        CornerPermutation::Ufl,
        CornerPermutation::Ubl,
        CornerPermutation::Ufl,
        CornerPermutation::Dfl,
        CornerPermutation::Dbl,
        CornerPermutation::Ufl,
        CornerPermutation::Ufr,
        CornerPermutation::Dfr,
        CornerPermutation::Dfl,
        CornerPermutation::Ufr,
        CornerPermutation::Ubr,
        CornerPermutation::Dbr,
        CornerPermutation::Dfr,
        CornerPermutation::Ubr,
        CornerPermutation::Ubl,
        CornerPermutation::Dbl,
        CornerPermutation::Dbr,
        CornerPermutation::Dfl,
        CornerPermutation::Dfr,
        CornerPermutation::Dbr,
        CornerPermutation::Dbl,
    ];

    pub const ORIENTATIONS: [CornerOrientation; 24] = [
        CornerOrientation::Good,
        CornerOrientation::Good,
        CornerOrientation::Good,
        CornerOrientation::Good,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::BadCw,
        CornerOrientation::BadCcw,
        CornerOrientation::Good,
        CornerOrientation::Good,
        CornerOrientation::Good,
        CornerOrientation::Good,
    ];

    /// Arbitrary corner sticker used in tables
    const XXX: CornerSticker = CornerSticker::Ubl;

    #[rustfmt::skip]
    pub const BY_FACES: [[[CornerSticker; 6]; 6]; 6] = [
        [
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Ufl, CornerSticker::XXX, CornerSticker::Ubl, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::Ufl, CornerSticker::XXX, CornerSticker::Ufr, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Ufr, CornerSticker::XXX, CornerSticker::Ubr, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::Ubl, CornerSticker::XXX, CornerSticker::Ubr, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
        ],
        [
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Luf, CornerSticker::XXX, CornerSticker::Lub, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Luf, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Ldf],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Lub, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Ldb],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Ldf, CornerSticker::XXX, CornerSticker::Ldb, CornerSticker::XXX],
        ],
        [
            [CornerSticker::XXX, CornerSticker::Ful, CornerSticker::XXX, CornerSticker::Fur, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Ful, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Fdl],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Fur, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Fdr],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::Fdl, CornerSticker::XXX, CornerSticker::Fdr, CornerSticker::XXX, CornerSticker::XXX],
        ],
        [
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Ruf, CornerSticker::XXX, CornerSticker::Rub, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Ruf, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Rdf],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Rub, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Rdb],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Rdf, CornerSticker::XXX, CornerSticker::Rdb, CornerSticker::XXX],
        ],
        [
            [CornerSticker::XXX, CornerSticker::Bul, CornerSticker::XXX, CornerSticker::Bur, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Bul, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Bdl],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::Bur, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Bdr],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::Bdl, CornerSticker::XXX, CornerSticker::Bdr, CornerSticker::XXX, CornerSticker::XXX],
        ],
        [
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Dfl, CornerSticker::XXX, CornerSticker::Dbl, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::Dfl, CornerSticker::XXX, CornerSticker::Dfr, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::Dfr, CornerSticker::XXX, CornerSticker::Dbr, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::Dbl, CornerSticker::XXX, CornerSticker::Dbr, CornerSticker::XXX, CornerSticker::XXX],
            [CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX, CornerSticker::XXX],
        ],
    ];
}

impl WingSticker {
    // TODO no table needed anymore
    pub const SOLVED: [WingSticker; 48] = [
        WingSticker::Ubr,
        WingSticker::Bur,
        WingSticker::Urf,
        WingSticker::Ruf,
        WingSticker::Ufl,
        WingSticker::Ful,
        WingSticker::Ulb,
        WingSticker::Lub,
        WingSticker::Luf,
        WingSticker::Ulf,
        WingSticker::Lfd,
        WingSticker::Fld,
        WingSticker::Ldb,
        WingSticker::Dlb,
        WingSticker::Lbu,
        WingSticker::Blu,
        WingSticker::Fur,
        WingSticker::Ufr,
        WingSticker::Frd,
        WingSticker::Rfd,
        WingSticker::Fdl,
        WingSticker::Dfl,
        WingSticker::Flu,
        WingSticker::Lfu,
        WingSticker::Rub,
        WingSticker::Urb,
        WingSticker::Rbd,
        WingSticker::Brd,
        WingSticker::Rdf,
        WingSticker::Drf,
        WingSticker::Rfu,
        WingSticker::Fru,
        WingSticker::Bul,
        WingSticker::Ubl,
        WingSticker::Bld,
        WingSticker::Lbd,
        WingSticker::Bdr,
        WingSticker::Dbr,
        WingSticker::Bru,
        WingSticker::Rbu,
        WingSticker::Dfr,
        WingSticker::Fdr,
        WingSticker::Drb,
        WingSticker::Rdb,
        WingSticker::Dbl,
        WingSticker::Bdl,
        WingSticker::Dlf,
        WingSticker::Ldf,
    ];

    pub const FROM_PERMUTATION_AND_HANDEDNESS: [[WingSticker; 2]; 24] = [
        [WingSticker::Ubr, WingSticker::Ubl],
        [WingSticker::Urf, WingSticker::Urb],
        [WingSticker::Ufl, WingSticker::Ufr],
        [WingSticker::Ulb, WingSticker::Ulf],
        [WingSticker::Luf, WingSticker::Lub],
        [WingSticker::Lfd, WingSticker::Lfu],
        [WingSticker::Ldb, WingSticker::Ldf],
        [WingSticker::Lbu, WingSticker::Lbd],
        [WingSticker::Fur, WingSticker::Ful],
        [WingSticker::Frd, WingSticker::Fru],
        [WingSticker::Fdl, WingSticker::Fdr],
        [WingSticker::Flu, WingSticker::Fld],
        [WingSticker::Rub, WingSticker::Ruf],
        [WingSticker::Rbd, WingSticker::Rbu],
        [WingSticker::Rdf, WingSticker::Rdb],
        [WingSticker::Rfu, WingSticker::Rfd],
        [WingSticker::Bul, WingSticker::Bur],
        [WingSticker::Bld, WingSticker::Blu],
        [WingSticker::Bdr, WingSticker::Bdl],
        [WingSticker::Bru, WingSticker::Brd],
        [WingSticker::Dfr, WingSticker::Dfl],
        [WingSticker::Drb, WingSticker::Drf],
        [WingSticker::Dbl, WingSticker::Dbr],
        [WingSticker::Dlf, WingSticker::Dlb],
    ];
}

impl EdgePermutation {
    pub const SOLVED: [EdgePermutation; 12] = [
        EdgePermutation::Ub,
        EdgePermutation::Ur,
        EdgePermutation::Uf,
        EdgePermutation::Ul,
        EdgePermutation::Fr,
        EdgePermutation::Fl,
        EdgePermutation::Bl,
        EdgePermutation::Br,
        EdgePermutation::Df,
        EdgePermutation::Dr,
        EdgePermutation::Db,
        EdgePermutation::Dl,
    ];

    pub const STICKERS: [[EdgeSticker; 2]; 12] = [
        [EdgeSticker::Ub, EdgeSticker::Bu],
        [EdgeSticker::Ur, EdgeSticker::Ru],
        [EdgeSticker::Uf, EdgeSticker::Fu],
        [EdgeSticker::Ul, EdgeSticker::Lu],
        [EdgeSticker::Fr, EdgeSticker::Rf],
        [EdgeSticker::Fl, EdgeSticker::Lf],
        [EdgeSticker::Bl, EdgeSticker::Lb],
        [EdgeSticker::Br, EdgeSticker::Rb],
        [EdgeSticker::Df, EdgeSticker::Fd],
        [EdgeSticker::Dr, EdgeSticker::Rd],
        [EdgeSticker::Db, EdgeSticker::Bd],
        [EdgeSticker::Dl, EdgeSticker::Ld],
    ];
}

impl CornerPermutation {
    pub const SOLVED: [CornerPermutation; 8] = [
        CornerPermutation::Ubl,
        CornerPermutation::Ubr,
        CornerPermutation::Ufr,
        CornerPermutation::Ufl,
        CornerPermutation::Dfl,
        CornerPermutation::Dfr,
        CornerPermutation::Dbr,
        CornerPermutation::Dbl,
    ];

    pub const STICKERS: [[CornerSticker; 3]; 8] = [
        [CornerSticker::Ubl, CornerSticker::Lub, CornerSticker::Bul],
        [CornerSticker::Ubr, CornerSticker::Bur, CornerSticker::Rub],
        [CornerSticker::Ufr, CornerSticker::Ruf, CornerSticker::Fur],
        [CornerSticker::Ufl, CornerSticker::Ful, CornerSticker::Luf],
        [CornerSticker::Dfl, CornerSticker::Ldf, CornerSticker::Fdl],
        [CornerSticker::Dfr, CornerSticker::Fdr, CornerSticker::Rdf],
        [CornerSticker::Dbr, CornerSticker::Rdb, CornerSticker::Bdr],
        [CornerSticker::Dbl, CornerSticker::Bdl, CornerSticker::Ldb],
    ];
}

impl Face {
    pub const ALL: [Face; 6] = [Face::U, Face::L, Face::F, Face::R, Face::B, Face::D];

    pub const OPPOSITES: [Face; 6] = [Face::D, Face::R, Face::B, Face::L, Face::F, Face::U];

    pub const NEIGHBORS: [[Face; 4]; 6] = [
        [Face::B, Face::R, Face::F, Face::L],
        [Face::U, Face::F, Face::D, Face::B],
        [Face::U, Face::R, Face::D, Face::L],
        [Face::U, Face::B, Face::D, Face::F],
        [Face::U, Face::L, Face::D, Face::R],
        [Face::F, Face::R, Face::B, Face::L],
    ];

    pub const CROSS: [[Face; 6]; 6] = [
        [Face::U, Face::F, Face::R, Face::B, Face::L, Face::U],
        [Face::B, Face::L, Face::U, Face::L, Face::D, Face::F],
        [Face::L, Face::D, Face::F, Face::U, Face::F, Face::R],
        [Face::F, Face::R, Face::D, Face::R, Face::U, Face::B],
        [Face::R, Face::U, Face::B, Face::D, Face::B, Face::L],
        [Face::D, Face::B, Face::L, Face::F, Face::R, Face::D],
    ];
}
