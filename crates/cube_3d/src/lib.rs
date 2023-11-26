use cube::{Cube, Face};
use glam::{vec2, vec4, Vec3, Vec4};

pub const VERTICES_PER_FACELET: usize = 4;
pub const INDICES_PER_FACELET: usize = 6;

/// Returns the number of facelets on an n by n Rubik's cube.
pub fn num_facelets(n: u16) -> usize {
    usize::from(n) * usize::from(n) * 6
}

pub fn update_facelet_colors(cube: &Cube, colors: &mut [Vec4]) {
    let n = cube.n;
    debug_assert_eq!(colors.len(), num_facelets(n) * VERTICES_PER_FACELET);

    let half_n = (n / 2) as i16;
    let mut vertex_index = 0;
    for face in Face::ALL {
        for y in -half_n..=half_n {
            if y == 0 && n % 2 == 0 {
                continue;
            }
            for x in -half_n..=half_n {
                if x == 0 && n % 2 == 0 {
                    continue;
                }
                let color = cube.color_at(face, x, y);
                let color = match color {
                    Face::U => vec4(1.0, 1.0, 1.0, 1.0),
                    Face::L => vec4(1.0, 0.5, 0.0, 1.0),
                    Face::F => vec4(0.0, 1.0, 0.0, 1.0),
                    Face::R => vec4(1.0, 0.0, 0.0, 1.0),
                    Face::B => vec4(0.0, 0.0, 1.0, 1.0),
                    Face::D => vec4(1.0, 1.0, 0.0, 1.0),
                };
                colors[vertex_index..vertex_index + 4].fill(color);
                vertex_index += 4;
            }
        }
    }
}

pub fn set_up_facelets(n: u16, vertices: &mut [Vec3], indices: &mut [u32]) {
    debug_assert_eq!(vertices.len(), num_facelets(n) * VERTICES_PER_FACELET);
    debug_assert_eq!(indices.len(), num_facelets(n) * INDICES_PER_FACELET);

    let half_n = (n / 2) as i16;
    let mut vertex_index = 0;
    let mut index_index = 0;
    for face in Face::ALL {
        let (lr_axis, ud_axis, fb_axis) = match face {
            Face::F => (Vec3::X, -Vec3::NEG_Y, Vec3::Z),
            Face::B => (Vec3::NEG_X, -Vec3::NEG_Y, Vec3::NEG_Z),
            Face::U => (Vec3::X, -Vec3::NEG_Z, Vec3::NEG_Y),
            Face::D => (Vec3::X, -Vec3::Z, Vec3::Y),
            Face::L => (Vec3::NEG_Z, -Vec3::NEG_Y, Vec3::X),
            Face::R => (Vec3::Z, -Vec3::NEG_Y, Vec3::NEG_X),
        };

        let size = 0.45;
        let top_left_offset = lr_axis * -size + ud_axis * -size - fb_axis * 0.5;
        let top_right_offset = lr_axis * size + ud_axis * -size - fb_axis * 0.5;
        let bottom_left_offset = lr_axis * -size + ud_axis * size - fb_axis * 0.5;
        let bottom_right_offset = lr_axis * size + ud_axis * size - fb_axis * 0.5;

        for y in -half_n..=half_n {
            if y == 0 && n % 2 == 0 {
                continue;
            }
            for x in -half_n..=half_n {
                if x == 0 && n % 2 == 0 {
                    continue;
                }

                let pt = vec2(x.into(), y.into());
                let pt = pt.x * lr_axis + pt.y * ud_axis - fb_axis * f32::from(n / 2);

                vertices[vertex_index..vertex_index + 4].copy_from_slice(&[
                    (pt + top_left_offset) / f32::from(n),
                    (pt + top_right_offset) / f32::from(n),
                    (pt + bottom_right_offset) / f32::from(n),
                    (pt + bottom_left_offset) / f32::from(n),
                ]);

                indices[index_index..index_index + 6].copy_from_slice(&[
                    vertex_index as u32 + 0,
                    vertex_index as u32 + 1,
                    vertex_index as u32 + 2,
                    vertex_index as u32 + 2,
                    vertex_index as u32 + 3,
                    vertex_index as u32 + 0,
                ]);

                vertex_index += 4;
                index_index += 6;
            }
        }
    }
}
