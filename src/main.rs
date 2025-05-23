// Import external crates
extern crate gl; // OpenGL bindings
extern crate sdl2; // SDL2 bindings

// Import constants and utilities from other modules
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

// Declare submodules
pub mod draw_gl;
pub mod log;
pub mod player;
pub mod render_gl;
pub mod square;
pub mod window_gl;

fn main() {
    let mut _is_log; // Variable to toggle logging

    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    // Initialize the video subsystem
    let video_subsystem = sdl.video().unwrap();

    // Configure OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core); // Use core profile
    gl_attr.set_context_version(4, 1); // OpenGL version 4.1

    // Create a window with OpenGL context
    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    // Load OpenGL functions
    let _gl = gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    // Compile shaders
    let vert_shader = render_gl::Shader
        ::from_vert_source(&CString::new(include_str!("./shaders/triangle.vert")).unwrap())
        .unwrap();
    let frag_shader = render_gl::Shader
        ::from_frag_source(&CString::new(include_str!("./shaders/triangle.frag")).unwrap())
        .unwrap();

    // Link shaders into a program
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // Set up OpenGL state
    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32); // Set viewport size
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set background color
    }

    // Create the player object
    let mut player: player::Player = player::Player::new(200.0, 200.0);

    // Create buffer objects for rendering
    let vbo_squares: gl::types::GLuint = 0;
    let vao_squares: gl::types::GLuint = 0;
    let mut bab: draw_gl::BufferArrayBinder = draw_gl::BufferArrayBinder::new(
        vao_squares,
        vbo_squares
    );

    // Main game loop
    let mut event_pump = sdl.event_pump().unwrap();
    let mut i: i32 = 0;
    'main: loop {
        // Toggle logging every 30 frames
        i += 1;
        if i % 30 == 0 {
            _is_log = 1;
        } else {
            _is_log = 0;
        }

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'main; // Exit the game loop on quit
                }
                _ => {}
            }
        }

        // Create a vertex array wrapper
        let mut vertices: VertexArrayWrapper = VertexArrayWrapper::new();

        // Process player input
        player = get_input(&event_pump, player);

        // Construct vertices for rendering
        construct_vertices(&player, &mut vertices, _is_log);

        // Bind vertex data to buffers
        bab.set_buffers(&vertices.points());
        bab.set_vertex_attribs(3, 6, 3);

        // Clear the screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Use the shader program
        shader_program.set_used();

        // Draw triangles and lines
        bab.draw_arrays(gl::TRIANGLES, 6, 0, vertices.triangle_end() as i32);
        bab.draw_arrays(gl::LINES, 6, 0, vertices.len() as i32);

        // Swap the window buffer
        window.gl_swap_window();
    }
}

// Handle player input
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
            player.y_pos + player.get_y_dir() * 5.0
        );
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
        player.update_pos(
            player.x_pos - player.get_x_dir() * 5.0,
            player.y_pos - player.get_y_dir() * 5.0
        );
    }
    return player;
}

// Construct vertices for rendering
fn construct_vertices(
    player: &player::Player,
    mut vertices: &mut VertexArrayWrapper,
    _is_log: i32
) {
    for i in 0..=7 {
        for ii in 0..=7 {
            if MAP[i][ii] == 1 {
                push_square_vertices(
                    &mut vertices,
                    square::Square::new(ii as i32, i as i32, draw_gl::Color::new(1.0, 1.0, 1.0))
                );
            } else {
                push_square_vertices(
                    &mut vertices,
                    square::Square::new(ii as i32, i as i32, draw_gl::Color::new(0.0, 0.0, 0.0))
                );
            }
        }
    }
    push_player_vertices(&mut vertices, player);
    push_line_vertices(&mut vertices, player);
    cast_rays(&mut vertices, player, _is_log);
}

// Push square vertices to the vertex array
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

// Push player vertices to the vertex array
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

// Push line vertices to the vertex array
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

// Cast rays for rendering
fn cast_rays(vertices: &mut VertexArrayWrapper, player: &player::Player, _is_log: i32) {
    let map = single_index_map();
    let dr: f32 = 0.0174333; // Ray angle increment
    let mut mx: i32;
    let mut my: i32;
    let mut _mp: i32;
    let mut dof: i32;

    let mut rx: f32;
    let mut ry: f32;
    let mut xo: f32;
    let mut yo: f32;
    let mut ra: f32 = player.get_dir() - dr * 30.0; // Start angle for rays

    for _r in 0..60 {
        // Normalize ra
        if ra < 0.0 {
            ra += 2.0 * PI;
        } else if ra > 2.0 * PI {
            ra -= 2.0 * PI;
        }

        dof = 0;
        let a_tan: f32 = -1.0 / ra.tan();

        if ra > PI && ra < 2.0 * PI {
            // Looking down
            ry = ((player.y_pos + 4.0) / (MAP_S as f32)).floor() * (MAP_S as f32) - 0.0001;
            rx = (player.y_pos + 4.0 - ry) * a_tan + player.x_pos + 4.0;
            yo = -MAP_S as f32;
            xo = -yo * a_tan;
        } else if ra > 0.0 && ra < PI {
            // Looking up
            ry = ((player.y_pos + 4.0) / (MAP_S as f32)).floor() * (MAP_S as f32) + (MAP_S as f32);
            rx = (player.y_pos + 4.0 - ry) * a_tan + player.x_pos + 4.0;
            yo = MAP_S as f32;
            xo = -yo * a_tan;
        } else {
            // Exactly horizontal (left or right)
            let sign = if ra == 0.0 || ra == 2.0 * PI { 1.0 } else { -1.0 };
            rx = player.x_pos + 4.0 + 100.0 * sign;
            ry = player.y_pos + 4.0;
            yo = 0.0;
            xo = 100.0 * sign;
            dof = 8;
        }

        // Add epsilon to prevent floating-point precision issues
        let epsilon = 0.0001;
        if ra > PI && ra < 2.0 * PI {
            rx -= epsilon;
        } else if ra < PI {
            rx += epsilon;
        }

        mx = (rx as i32) / MAP_S;
        my = (ry as i32) / MAP_S;
        _mp = my * MAP_X + mx;

        while dof < 8 {
            mx = (rx as i32) / MAP_S;
            my = (ry as i32) / MAP_S;
            _mp = my * MAP_X + mx;

            // Map boundary checks
            if mx < 0 || mx >= MAP_X || my < 0 || my >= MAP_Y {
                break;
            }

            if _mp < MAP_X * MAP_Y && _mp >= 0 && map[_mp as usize] == 1 {
                dof = 8;
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
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
            // Looking left
            rx = ((player.x_pos + 4.0) / (MAP_S as f32)).floor() * (MAP_S as f32) - 0.0001;
            ry = (player.x_pos + 4.0 - rx) * n_tan + player.y_pos + 4.0;
            xo = -MAP_S as f32;
            yo = -xo * n_tan;
        } else if ra < P2 || ra > P3 {
            // Looking right
            rx = ((player.x_pos + 4.0) / (MAP_S as f32)).floor() * (MAP_S as f32) + (MAP_S as f32);
            ry = (player.x_pos + 4.0 - rx) * n_tan + player.y_pos + 4.0;
            xo = MAP_S as f32;
            yo = -xo * n_tan;
        } else {
            // Exactly vertical (up or down)
            let sign = if ra == P2 { 1.0 } else { -1.0 };
            rx = player.x_pos + 4.0;
            ry = player.y_pos + 4.0 + 100.0 * sign;
            xo = 0.0;
            yo = 100.0 * sign;
            dof = 8;
        }

        while dof < 8 {
            mx = (rx as i32) / MAP_S;
            my = (ry as i32) / MAP_S;
            _mp = my * MAP_X + mx;

            // Map boundary checks
            if mx < 0 || mx >= MAP_X || my < 0 || my >= MAP_Y {
                break;
            }

            if _mp < MAP_X * MAP_Y && _mp >= 0 && map[_mp as usize] == 1 {
                dof = 8;
            } else {
                rx += xo;
                ry += yo;
                dof += 1;
            }
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

        ra += dr;
    }
}

// Vertex array wrapper for managing vertex data
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
