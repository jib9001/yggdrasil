use crate::draw_gl::get_x;
use crate::draw_gl::get_y;
use crate::draw_gl::Color;
use crate::window_gl::HEIGHT;
use crate::window_gl::WIDTH;
use crate::window_gl::MAP_S;

pub struct Square {
    tl_point: [f32; 3],
    tr_point: [f32; 3],
    bl_point: [f32; 3],
    br_point: [f32; 3],
    color: [f32; 3],
}

impl Square {
    pub fn new(index_1: i32, index_2: i32, color: Color) -> Square {
        let tl_point = [
            get_x((index_1 * MAP_S) as f32, WIDTH),
            get_y((index_2 * MAP_S) as f32, HEIGHT),
            0.0,
        ];
        let tr_point = [
            get_x((index_1 * MAP_S + MAP_S) as f32, WIDTH),
            get_y((index_2 * MAP_S) as f32, HEIGHT),
            0.0,
        ];
        let bl_point = [
            get_x((index_1 * MAP_S) as f32, WIDTH),
            get_y((index_2 * MAP_S + MAP_S) as f32, HEIGHT),
            0.0,
        ];
        let br_point = [
            get_x((index_1 * MAP_S + MAP_S) as f32, WIDTH),
            get_y((index_2 * MAP_S + MAP_S) as f32, HEIGHT),
            0.0,
        ];
        let color = color.get_colors();

        Square {
            tl_point,
            tr_point,
            bl_point,
            br_point,
            color,
        }
    }

    pub fn get_color(&self) -> [f32; 3] {
        return self.color;
    }

    pub fn get_vertices(&self) -> [[f32; 3]; 4] {
        return [self.tl_point, self.tr_point, self.bl_point, self.br_point];
    }
}