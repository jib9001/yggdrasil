use crate::draw_gl::get_y;
use crate::draw_gl::get_x;
use crate::window_gl::HEIGHT;
use crate::window_gl::WIDTH;

pub struct Player {
    pub x_pos: f32,
    pub y_pos: f32,
    pub player_dir: f32,
    pub x_dir: f32,
    pub y_dir: f32,
    pub tl_point: [f32; 3],
    pub tr_point: [f32; 3],
    pub bl_point: [f32; 3],
    pub br_point: [f32; 3],
    pub color: [f32; 3],
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        let x_pos = x;
        let y_pos = y;
        let player_dir = 0.0;
        let x_dir = 0.0;
        let y_dir = 0.0;
        let tl_point = [get_x(x, WIDTH), get_y(y, HEIGHT), 0.0];
        let tr_point = [get_x(x + 8.0, WIDTH), get_y(y, HEIGHT), 0.0];
        let bl_point = [get_x(x, WIDTH), get_y(y + 8.0, HEIGHT), 0.0];
        let br_point = [get_x(x + 8.0, WIDTH), get_y(y + 8.0, HEIGHT), 0.0];

        let color = [0.0, 0.0, 1.0];

        Player {
            x_pos,
            y_pos,
            player_dir,
            x_dir,
            y_dir,
            tl_point,
            tr_point,
            bl_point,
            br_point,
            color,
        }
    }

    pub fn update_pos(&mut self, x: f32, y: f32) {
        self.update_x_pos(x);
        self.update_y_pos(y);
    }

    pub fn update_x_pos(&mut self, x: f32) {
        self.x_pos = x;
        self.set_conrners();
    }

    pub fn update_y_pos(&mut self, y: f32) {
        self.y_pos = y;
        self.set_conrners();
    }

    pub fn update_dir(&mut self, new_dir: f32) {
        self.player_dir = new_dir;
    }

    pub fn update_x_dir(&mut self, new_dir: f32) {
        self.x_dir = new_dir;
    }

    pub fn update_y_dir(&mut self, new_dir: f32) {
        self.y_dir = new_dir;
    }    

    pub fn get_dir(&self) -> f32 {
        return self.player_dir;
    }

    pub fn get_x_dir(&self) -> f32 {
        return self.x_dir;
    }

    pub fn get_y_dir(&self) -> f32 {
        return self.y_dir;
    }

    pub fn get_player_x(&self, offset: f32) -> f32 {
        return get_x(self.x_pos + offset, WIDTH);
    }

    pub fn get_player_y(&self, offset: f32) -> f32 {
        return get_y(self.y_pos + offset, HEIGHT);
    }

    fn set_conrners(&mut self) {
        self.tl_point = [get_x(self.x_pos, WIDTH), get_y(self.y_pos, HEIGHT), 0.0];
        self.tr_point = [get_x(self.x_pos + 8.0, WIDTH), get_y(self.y_pos, HEIGHT), 0.0];
        self.bl_point = [get_x(self.x_pos, WIDTH), get_y(self.y_pos + 8.0, HEIGHT), 0.0];
        self.br_point = [get_x(self.x_pos + 8.0, WIDTH), get_y(self.y_pos + 8.0, HEIGHT), 0.0];
    }
}