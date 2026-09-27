use super::components::Hex;
use bevy::prelude::*;

// 16 bit tile assets
// const HEX_WIDTH: f32 = 9.25;
// const HEX_HEIGHT: f32 = 8.0;

// 32 bit tile assets
pub const HEX_WIDTH: f32 = 18.5;
pub const HEX_HEIGHT: f32 = 16.0;

pub const SQRT_3_OVER_3: f32 = 0.577_350_26;
pub const SQRT_3: f32 = 1.732_050_8;
pub const TWO_THIRDS: f32 = 2.0 / 3.0;
pub const THREE_HALVES: f32 = 3.0 / 2.0;

pub fn from_hex_to_world(hex: Hex) -> Vec2 {
    let q = hex.q as f32;
    let r = hex.r as f32;

    let x = HEX_WIDTH * SQRT_3 * (q + r * 0.5);

    let y = HEX_HEIGHT * THREE_HALVES * r;

    Vec2::new(x.round(), y.round())
}

pub fn from_world_to_hex(pixel: Vec2) -> Hex {
    let px = pixel.x / HEX_WIDTH;
    let py = pixel.y / HEX_HEIGHT;

    let r_float = py * TWO_THIRDS;

    let q_float = (px * SQRT_3_OVER_3) - (r_float * 0.5);

    hex_round(q_float, r_float)
}

fn hex_round(frac_q: f32, frac_r: f32) -> Hex {
    let frac_s = -frac_q - frac_r;

    let mut q = frac_q.round();
    let mut r = frac_r.round();
    let s = frac_s.round();

    let q_diff = (q - frac_q).abs();
    let r_diff = (r - frac_r).abs();
    let s_diff = (s - frac_s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    }

    Hex::new(q as i32, r as i32)
}
