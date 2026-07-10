use bevy::prelude::*;

const HEX_WIDTH: f32 = 16.125;
const HEX_HEIGHT: f32 = 15.25;

const SQRT_3_OVER_3: f32 = 0.577_350_26;
const SQRT_3: f32 = 1.732_050_8;
const TWO_THIRDS: f32 = 2.0 / 3.0;
const THREE_HALVES: f32 = 3.0 / 2.0;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}
impl Hex {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    pub fn to_pixel(&self) -> Vec2 {
        let q = self.q as f32;
        let r = self.r as f32;

        let x = HEX_WIDTH * SQRT_3 * (q + r * 0.5);

        let y = HEX_HEIGHT * THREE_HALVES * r;

        Vec2::new(x, y)
    }
    pub fn from_world(pixel: Vec2) -> Hex {
        // Normalize coordinates
        let px = pixel.x / HEX_WIDTH;
        let py = pixel.y / HEX_HEIGHT;

        // Calculate r
        let r_float = py * TWO_THIRDS;

        //  Calculate q reusing r
        let q_float = (px * SQRT_3_OVER_3) - (r_float * 0.5);

        hex_round(q_float, r_float)
    }
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
