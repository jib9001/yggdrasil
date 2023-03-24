use crate::window_gl::MAP_S;
use crate::window_gl::WIDTH;
use crate::window_gl::HEIGHT;
use crate::draw_gl::get_y;
use crate::draw_gl::get_x;

pub mod draw_gl;

pub struct Square {
    pub tl_point: [f32; 3],
    pub tr_point: [f32; 3],
    pub bl_point: [f32; 3],
    pub br_point: [f32; 3],
    pub color: [f32; 3],
}

impl Square {
    pub fn new(index_1: i32, index_2: i32, color: draw_gl::Color) -> Square {
        let tl_point = [
            get_x((index_1 * MAP_S + 1) as f32, WIDTH),
            get_y((index_2 * MAP_S + 1) as f32, HEIGHT),
            0.0,
        ];
        let tr_point = [
            get_x((index_1 * MAP_S + MAP_S - 1) as f32, WIDTH),
            get_y((index_2 * MAP_S + 1) as f32, HEIGHT),
            0.0,
        ];
        let bl_point = [
            get_x((index_1 * MAP_S + 1) as f32, WIDTH),
            get_y((index_2 * MAP_S + MAP_S - 1) as f32, HEIGHT),
            HEIGHT,
            0.0,
        ];
        let br_point = [
            get_x((index_1 * MAP_S + MAP_S - 1) as f32, WIDTH),
            get_y((index_2 * MAP_S + MAP_S - 1) as f32, HEIGHT),
            0.0,
        ];
        let color = [color.r, color.g, color.b];

        Square {
            tl_point,
            tr_point,
            bl_point,
            br_point,
            color,
        }
    }
}