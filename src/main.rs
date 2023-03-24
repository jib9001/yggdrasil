extern crate gl;
extern crate sdl2;

use crate::draw_gl::get_y;
use crate::draw_gl::get_x;
use crate::window_gl::HEIGHT;
use crate::window_gl::WIDTH;
use crate::window_gl::MAP;
use sdl2::keyboard::Scancode;
use std::ffi::CString;

pub mod render_gl;
pub mod draw_gl;
pub mod window_gl;
pub mod player;
pub mod square;

fn main() {
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
    let _gl = gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    // compile vertex shader "triangle.vert"
    let vert_shader = render_gl::Shader
        ::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
        .unwrap();

    // compile fragment shader "triangle.frag"
    let frag_shader = render_gl::Shader
        ::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
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

    // set up vertex buffer object
    let mut vertices: Vec<f32>;

    // create vertex buffer object
    let mut vbo: gl::types::GLuint = 0;

    // create vertex array object
    let mut vao: gl::types::GLuint = 0;

    let mut bab: draw_gl::BufferArrayBinder = draw_gl::BufferArrayBinder::new(vao, vbo);

    // main loop
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'main;
                }
                _ => {}
            }
        }

        player = get_input(&event_pump, player);
        vertices = construct_vertices(&player);

        bab.set_buffers(&vertices);

        // create opengl buffer from buffer object
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();

        bab.draw_arrays(gl::TRIANGLES, 6, &vertices);

        window.gl_swap_window();
    }
}

fn get_input(event_pump: &sdl2::EventPump, mut player: player::Player) -> player::Player {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
        player.update_x_pos(player.x_pos - 8.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
        player.update_x_pos(player.x_pos + 8.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
        player.update_y_pos(player.y_pos - 8.0);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
        player.update_y_pos(player.y_pos + 8.0);
    }

    return player;
}

fn construct_vertices(player: &player::Player) -> Vec<f32> {
    let mut vertices: Vec<f32> = Vec::new();
    for i in 0..=7 {
        for ii in 0..=7 {
            if MAP[i][ii] == 1 {
                vertices = push_square_vertices(
                    vertices,
                    square::Square::new(ii as i32, i as i32, draw_gl::Color::new(1.0, 1.0, 1.0))
                );
            } else {
                vertices = push_square_vertices(
                    vertices,
                    square::Square::new(ii as i32, i as i32, draw_gl::Color::new(0.0, 0.0, 0.0))
                );
            }
        }
    }
    vertices = push_player_vertices(vertices, player);

    return vertices;
}

fn push_square_vertices(mut vertices: Vec<f32>, wall: square::Square) -> Vec<f32> {
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

    return vertices;
}

fn push_player_vertices(mut vertices: Vec<f32>, player: &player::Player) -> Vec<f32> {
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

    return vertices;
}