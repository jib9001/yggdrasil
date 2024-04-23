extern crate gl;
extern crate sdl2;

use crate::window_gl::HEIGHT;
use crate::window_gl::MAP;
use crate::window_gl::WIDTH;
use draw_gl::get_x;
use draw_gl::get_y;
use sdl2::keyboard::Scancode;
use std::f32::consts::PI;
use std::ffi::CString;
use window_gl::single_index_map;
use window_gl::MAP_S;
use window_gl::MAP_X;
use window_gl::MAP_Y;

pub mod draw_gl;
pub mod log;
pub mod player;
pub mod render_gl;
pub mod square;
pub mod window_gl;

fn main() {
    let mut isLog = 0;
    // initialize sdl2
    let sdl = sdl2::init().unwrap();
    // find the C opengl libraries
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    // set opengl profile to core for current feature set
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // specify opengl version 4.1
    gl_attr.set_context_version(4, 1);

    // create window object, needs opengl context
    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // create opengl context
    let _gl_context = window.gl_create_context().unwrap();
    // I think this ititializes opengl?
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // compile vertex shader "triangle.vert"
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("./shaders/triangle.vert")).unwrap(),
    )
    .unwrap();

    // compile fragment shader "triangle.frag"
    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("./shaders/triangle.frag")).unwrap(),
    )
    .unwrap();

    // create shader program and link compiled shaders to it
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up shared state for window
    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    //create player
    let mut player: player::Player = player::Player::new(200.0, 200.0);

    // create vertex buffer object for squares
    let vbo_squares: gl::types::GLuint = 0;
    // create vertex attribute object for sauares
    let vao_squares: gl::types::GLuint = 0;

    // object for binding arrays to the
    let mut bab: draw_gl::BufferArrayBinder =
        draw_gl::BufferArrayBinder::new(vao_squares, vbo_squares);

    // main loop
    let mut event_pump = sdl.event_pump().unwrap();
    let mut i: i32 = 0;
    //let mut isLog :i32 = 0;
    'main: loop {
        i += 1;
        if i % 30 == 0 {
            isLog = 1;
        } else {
            isLog = 0;
        }

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'main;
                }
                _ => {}
            }
        }

        // create vertex array wrapper
        let mut vertices: VertexArrayWrapper = VertexArrayWrapper::new();

        player = get_input(&event_pump, player);
        construct_vertices(&player, &mut vertices, isLog);

        bab.set_buffers(&vertices.points());
        bab.set_vertex_attribs(3, 6, 3);

        // create opengl buffer from buffer object
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();

        bab.draw_arrays(gl::TRIANGLES, 6, 0, vertices.triangle_end() as i32);
        bab.draw_arrays(gl::LINES, 6, 0, vertices.len() as i32);

        window.gl_swap_window();
    }
}

fn get_input(event_pump: &sdl2::EventPump, mut player: player::Player) -> player::Player {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
        player.update_dir(player.get_dir() - 0.1);

        if player.get_dir() < 0.0 {
            player.update_dir(player.get_dir() + 2.0 * PI);
        }

        player.update_x_dir(player.get_dir().cos());
        player.update_y_dir(player.get_dir().sin());
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
        player.update_dir(player.get_dir() + 0.1);

        if player.get_dir() > 2.0 * PI {
            player.update_dir(player.get_dir() - 2.0 * PI);
        }

        player.update_x_dir(player.get_dir().cos());
        player.update_y_dir(player.get_dir().sin());
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
        player.update_pos(
            player.x_pos + player.get_x_dir() * 5.0,
            player.y_pos + player.get_y_dir() * 5.0,
        );
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
        player.update_pos(
            player.x_pos - player.get_x_dir() * 5.0,
            player.y_pos - player.get_y_dir() * 5.0,
        );
    }

    return player;
}

fn construct_vertices(player: &player::Player, mut vertices: &mut VertexArrayWrapper, isLog: i32) {
    for i in 0..=7 {
        for ii in 0..=7 {
            if MAP[i][ii] == 1 {
                push_square_vertices(
                    &mut vertices,
                    square::Square::new(ii as i32, i as i32, draw_gl::Color::new(1.0, 1.0, 1.0)),
                );
            } else {
                push_square_vertices(
                    &mut vertices,
                    square::Square::new(ii as i32, i as i32, draw_gl::Color::new(0.0, 0.0, 0.0)),
                );
            }
        }
    }
    push_player_vertices(&mut vertices, player);
    push_line_vertices(&mut vertices, player);
    cast_rays(&mut vertices, player, isLog);
}

fn push_square_vertices(vertices: &mut VertexArrayWrapper, wall: square::Square) {
    let points: [[f32; 3]; 4] = wall.get_vertices();

    for i in 0..=2 {
        for num in points[i] {
            vertices.push(num);
        }
        for num in wall.get_color() {
            vertices.push(num);
        }
    }

    for i in 1..=3 {
        for num in points[i] {
            vertices.push(num);
        }
        for num in wall.get_color() {
            vertices.push(num);
        }
    }
}

fn push_player_vertices(vertices: &mut VertexArrayWrapper, player: &player::Player) {
    let points: [[f32; 3]; 4] = [
        player.tl_point,
        player.tr_point,
        player.bl_point,
        player.br_point,
    ];

    for i in 0..=2 {
        for num in points[i] {
            vertices.push(num);
        }
        for num in player.color {
            vertices.push(num);
        }
    }

    for i in 1..=3 {
        for num in points[i] {
            vertices.push(num);
        }
        for num in player.color {
            vertices.push(num);
        }
    }

    vertices.set_triangle_end(vertices.len());
}

fn push_line_vertices(vertices: &mut VertexArrayWrapper, player: &player::Player) {
    vertices.push(player.get_player_x(4.0));
    vertices.push(player.get_player_y(4.0));
    vertices.push(0.0);
    vertices.push(1.0);
    vertices.push(1.0);
    vertices.push(0.0);
    vertices.push(player.get_player_x(4.0 + player.get_x_dir() * 20.0));
    vertices.push(player.get_player_y(4.0 + player.get_y_dir() * 20.0));
    vertices.push(0.0);
    vertices.push(1.0);
    vertices.push(1.0);
    vertices.push(0.0);
}

fn cast_rays(vertices: &mut VertexArrayWrapper, player: &player::Player, isLog: i32) {
    let map = single_index_map();
    let DR: f32 = 0.0174333;
    let mut mx: i32 = 0;
    let mut my: i32 = 0;
    let mut mp: i32 = 0;
    let mut dof: i32;

    let mut rx: f32;
    let mut ry: f32;
    let mut xo: f32;
    let mut yo: f32;
    let mut ra: f32 = player.get_dir() - DR * 30.0;

    for _r in 0..60 {
        dof = 0;
        let a_tan: f32 = -1.0 / ra.tan();

        // If looking down
        if ra > PI && ra < 2.0 * PI {
            ry = (((((player.y_pos + 4.0) / 64.0).round() as i32) * 64) as f32) - 0.0001;
            rx = (player.y_pos + 4.0 - ry) * a_tan + player.x_pos + 4.0;
            yo = -MAP_S as f32;
            xo = -yo * a_tan;
        } else if
            // if looking up
            ra < PI &&
            ra > 0.0
        {
            ry = ((((player.y_pos + 4.0 / 64.0).round() as i32) * 64) as f32) + 64.0;
            rx = (player.y_pos + 4.0 - ry) * a_tan + player.x_pos;
            yo = MAP_S as f32;
            xo = -yo * a_tan;
        } else {
            if ra == 0.0 || ra == 2.0 * PI {
                rx = player.x_pos + 4.0 + 100.0;
                ry = player.y_pos + 4.0;
                yo = 0.0;
                xo = 100.0;
                dof = 8;
            } else if ra == PI {
                rx = player.x_pos + 4.0 - 100.0;
                ry = player.y_pos + 4.0;
                yo = 0.0;
                xo = -100.0;
                dof = 8;
            } else {
                rx = player.x_pos + 4.0 - 100.0;
                ry = player.y_pos + 4.0;
                yo = 0.0;
                xo = -100.0;
                dof = 8;
            }
        }

        mx = (rx as i32) / MAP_S;
        my = (ry as i32) / MAP_S;
        mp = my * MAP_X + mx;

        while dof < 8 {
            mx = (rx as i32) / MAP_S;
            my = (ry as i32) / MAP_S;
            mp = my * MAP_X + mx;

            if mp < MAP_X * MAP_Y && mp >= 0 && map[mp as usize] == 1 {
                dof = 8;
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }

            if dof == 8 {
                log::log_h_ray(mx, my, mp, dof, a_tan, ra, rx, ry, xo, yo);
            }
        }

        if (isLog == 1) {
            log::log_h_ray(mx, my, mp, dof, a_tan, ra, rx, ry, xo, yo);

            log::log_ray_vertices(player, rx, ry);
        }
        vertices.push(player.get_player_x(4.0));
        vertices.push(player.get_player_y(4.0));
        vertices.push(0.0);
        vertices.push(0.0);
        vertices.push(1.0);
        vertices.push(0.0);
        vertices.push(get_x(rx, WIDTH));
        vertices.push(get_y(ry, HEIGHT));
        vertices.push(0.0);
        vertices.push(0.0);
        vertices.push(1.0);
        vertices.push(0.0);

        dof = 0;
        let n_tan: f32 = -ra.tan();
        const P2: f32 = PI / 2.0;
        const P3: f32 = (3.0 * PI) / 2.0;

        if ra > P2 && ra < P3 {
            rx = (((((player.x_pos + 4.0) / 64.0).round() as i32) * 64) as f32) - 0.0001;
            ry = (player.x_pos + 4.0 - rx) * n_tan + player.y_pos + 4.0;
            xo = -MAP_S as f32;
            yo = -xo * n_tan;
        } else if ra < P2 || ra > P3 {
            rx = (((((player.x_pos + 4.0) / 64.0).round() as i32) * 64) as f32) + 64.0;
            ry = (player.x_pos + 4.0 - rx) * n_tan + player.y_pos + 4.0;
            xo = MAP_S as f32;
            yo = -xo * n_tan;
        } else {
            if ra == P2 {
                ry = player.y_pos + 4.0 + 100.0;
                rx = player.x_pos + 4.0;
                xo = 0.0;
                yo = 100.0;
                dof = 8;
            } else {
                ry = player.y_pos + 4.0 - 100.0;
                rx = player.x_pos + 4.0;
                xo = 0.0;
                yo = -100.0;
                dof = 8;
            }
        }

        while dof < 8 {
            mx = (rx as i32) / MAP_S;
            my = (ry as i32) / MAP_S;
            mp = my * MAP_X + mx;

            if mp < MAP_X * MAP_Y && mp >= 0 && map[mp as usize] == 1 {
                dof = 8;
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }

            if dof == 8 {
                log::log_v_ray(mx, my, mp, dof, n_tan, ra, rx, ry, xo, yo);
            }
        }

        if (isLog == 1) {
            log::log_v_ray(mx, my, mp, dof, n_tan, ra, rx, ry, xo, yo);

            log::log_ray_vertices(player, rx, ry);
        }

        vertices.push(player.get_player_x(4.0));
        vertices.push(player.get_player_y(4.0));
        vertices.push(0.0);
        vertices.push(1.0);
        vertices.push(0.0);
        vertices.push(0.0);
        vertices.push(get_x(rx, WIDTH));
        vertices.push(get_y(ry, HEIGHT));
        vertices.push(0.0);
        vertices.push(1.0);
        vertices.push(0.0);
        vertices.push(0.0);

        ra += DR;
    }
}

struct VertexArrayWrapper {
    points: Vec<f32>,
    triangle_end: usize,
}

impl VertexArrayWrapper {
    pub fn new() -> VertexArrayWrapper {
        let points = Vec::new();
        let triangle_end = 0;

        VertexArrayWrapper {
            points,
            triangle_end,
        }
    }

    pub fn set_triangle_end(&mut self, end: usize) {
        self.triangle_end = end;
    }

    pub fn len(&self) -> usize {
        return self.points.len();
    }

    pub fn push(&mut self, num: f32) {
        self.points.push(num);
    }

    pub fn points(&self) -> &Vec<f32> {
        return &self.points;
    }

    pub fn triangle_end(&self) -> usize {
        return self.triangle_end;
    }
}
