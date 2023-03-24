pub mod draw_gl;
pub mod window_gl;

pub struct Player {
    pub x_pos: f32,
    pub y_pos: f32,
    pub player_dir: f32,
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
        let tl_point = [
            draw_gl::get_x(x),
            draw_gl::get_y(y),
            0.0,
        ];
        let tr_point = [
            draw_gl::get_x(x + 8.0),
            draw_gl::get_y(y),
            0.0,
        ];
        let bl_point = [
            draw_gl::get_x(x),
            draw_gl::get_y(y + 8.0),
            0.0,
        ];
        let br_point = [
            draw_gl::get_x(x + 8.0),
            draw_gl::get_y(y + 8.0),
            0.0,
        ];

        let color = [0.0, 0.0, 1.0];

        Player {
            x_pos,
            y_pos,
            player_dir,
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

    fn set_conrners(&mut self) {
        self.tl_point = [
            draw_gl::get_x(self.x_pos, window_gl::WIDTH),
            draw_gl::get_y(self.y_pos, window_gl::HEIGHT),
            0.0,
        ];
        self.tr_point = [
            draw_gl::get_x(self.x_pos + 8.0, window_gl::WIDTH),
            draw_gl::get_y(self.y_pos, window_gl::HEIGHT),
            0.0,
        ];
        self.bl_point = [
            draw_gl::get_x(self.x_pos, window_gl::WIDTH),
            draw_gl::get_y(self.y_pos + 8.0, window_gl::HEIGHT),
            0.0,
        ];
        self.br_point = [
            draw_gl::get_x(self.x_pos + 8.0, window_gl::WIDTH),
            draw_gl::get_y(self.y_pos + 8.0, window_gl::HEIGHT),
            0.0,
        ];
    }
}