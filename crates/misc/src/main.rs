use cube::{CornerSticker, Cube, EdgeSticker, WingSticker};
use cube_bld::{memo, Pieces};
use std::collections::HashMap;

fn main() {
    let mut occurences = HashMap::<[u8; 2], u32>::new();

    let mut total = 0;
    for i in 0..10_000_000 {
        let cube = Cube::new_random(5, i);
        collect(&cube.edges, EdgeSticker::Uf, &mut occurences, &mut total, |p| p.index() as u8);
        collect(&cube.corners, CornerSticker::Ufr, &mut occurences, &mut total, |p| p.index() as u8);
        collect(&cube.layers[0].wings, WingSticker::Ufr, &mut occurences, &mut total, |p| p.edge_sticker_considering_handedness().index() as u8);
        collect(&cube.layers[0].tcenters, EdgeSticker::Uf, &mut occurences, &mut total, |p| p.index() as u8);
        collect(&cube.layers[0].xcenters, CornerSticker::Ufr, &mut occurences, &mut total, |p| p.index() as u8);
    }

    for a in 0..24 {
        for b in 0..24 {
            print!(
                "{},",
                *occurences.get(&[a, b]).unwrap_or(&0) as f32 / total as f32
            );
        }
        println!();
    }
}

fn collect<P: Pieces>(
    pieces: &P,
    buffer: P::Sticker,
    occurences: &mut HashMap<[u8; 2], u32>,
    total: &mut i32,
    index: impl Fn(P::Sticker) -> u8
) {
    let memo = memo(pieces, buffer);
    for [_a, b, c] in memo.cycles {
        *occurences
            .entry([index(b), index(c)])
            .or_default() += 1;
        *total += 1;
    }
    if let Some([_b, c]) = memo.parity {
        *occurences
            .entry([index(c), index(c)])
            .or_default() += 1;
        *total += 1;
    }
}
