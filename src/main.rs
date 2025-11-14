// --- External Crates ---
extern crate gl; // OpenGL bindings
extern crate sdl2; // SDL2 bindings

// --- Imports from Other Modules ---
use crate::window_gl::{ HEIGHT, RAYS_COUNT, RENDER_X, RENDER_Y, WIDTH }; // Window dimensions
use crate::draw_gl::VertexArrayWrapper; // Wrapper for vertex array management
use sdl2::keyboard::Scancode; // Keyboard input handling
use std::f32::consts::PI; // Mathematical constant for pi
use std::ffi::CString; // String conversion for OpenGL

// --- Submodules ---
pub mod draw_gl; // OpenGL utilities
pub mod log; // Logging utilities
pub mod player; // Player logic
pub mod render_gl; // Shader and OpenGL program management
pub mod square; // Map square representation
pub mod window_gl; // SDL2 window and OpenGL context setup

// --- Main Function ---
fn main() {
    // --- State Variables ---
    let mut _is_log = 0; // Toggle for logging/debugging

    // Pixel buffer for the raycasted scene (used as a texture)
    let mut _pixels: [[[u8; 3]; RENDER_X as usize]; RENDER_Y as usize] = [
        [[0u8; 3]; RENDER_X as usize];
        RENDER_Y as usize
    ];
    // Arrays to store horizontal and vertical ray distances for each column
    let mut hrays: [f32; RAYS_COUNT as usize] = [0.0; RAYS_COUNT as usize];
    let mut vrays: [f32; RAYS_COUNT as usize] = [0.0; RAYS_COUNT as usize];

    // --- SDL2 and OpenGL Initialization ---
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Configure OpenGL context attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    // Create the window and OpenGL context
    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    // --- Shader Compilation ---
    let vert_shader = render_gl::Shader
        ::from_vert_source(&CString::new(include_str!("./shaders/triangle.vert")).unwrap())
        .unwrap();
    let frag_shader = render_gl::Shader
        ::from_frag_source(&CString::new(include_str!("./shaders/triangle.frag")).unwrap())
        .unwrap();
    let tex_frag_shader = render_gl::Shader
        ::from_frag_source(&CString::new(include_str!("./shaders/tex.frag")).unwrap())
        .unwrap();
    let tex_vert_shader = render_gl::Shader
        ::from_vert_source(&CString::new(include_str!("./shaders/tex.vert")).unwrap())
        .unwrap();

    // --- Shader Program Linking ---
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    let tex_shader_program = render_gl::Program
        ::from_shaders(&[tex_vert_shader, tex_frag_shader])
        .unwrap();

    // --- OpenGL State Setup ---
    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32); // Set viewport size
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set background color
    }

    // --- Game State Initialization ---
    let mut player: player::Player = player::Player::new(200.0, 200.0); // Player starting position

    // Texture manager for uploading the raycasted scene as a texture
    let mut _texture_manager = draw_gl::TextureManager::new();

    // BufferArrayBinder manages VAO/VBO for rendering
    let vbo_squares: gl::types::GLuint = 0;
    let vao_squares: gl::types::GLuint = 0;
    let mut bab: draw_gl::BufferArrayBinder = draw_gl::BufferArrayBinder::new(
        vao_squares,
        vbo_squares
    );

    // --- Main Game Loop ---
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        // --- Event Handling ---
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'main; // Exit the game loop on quit
                }
                _ => {}
            }
        }

        // --- Vertex Construction ---
        let mut vertices: VertexArrayWrapper = VertexArrayWrapper::new();

        // --- Player Input ---
        player = get_input(&event_pump, player);

        // --- Build All Vertices (map, player, lines, rays, canvas) ---
        render_gl::construct_vertices(&player, &mut vertices, &mut hrays, &mut vrays, _is_log);

        // --- Raycasting: Draw Walls to Pixel Buffer ---
        render_gl::draw_walls_to_pixels(
            &mut _pixels,
            &hrays,
            &vrays,
            [120, 120, 120], // horizontal wall color (light gray)
            [80, 80, 80], // vertical wall color (dark gray)
            [30, 30, 60] // background color (dark blue)
        );

        // --- Upload Pixel Buffer as Texture ---
        _texture_manager.load_texture(_pixels).unwrap();

        // --- Bind Vertex Data to Buffers ---
        bab.set_buffers(&vertices.points());
        bab.set_vertex_attribs(3, 6, 3);

        // --- Clear the Screen ---
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // --- Draw Map, Player, and Lines (non-textured geometry) ---
        shader_program.set_used();
        bab.set_vertex_attribs(3, 6, 3); // 3 pos, 3 color, stride 6
        bab.draw_arrays(gl::TRIANGLES, 6, 0, vertices.triangle_end() as i32);
        bab.draw_arrays(gl::LINES, 6, vertices.triangle_end() as i32, vertices.line_end() as i32);

        // --- Draw Canvas (textured quad) ---
        tex_shader_program.set_used();
        // For canvas: 3 pos, 3 color, 2 texcoord = 8 floats per vertex
        bab.set_vertex_attribs(3, 8, 3);
        // Set up texcoord attribute (location 2)
        unsafe {
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2, // location 2 in shader
                2, // 2 floats for texcoord
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, _texture_manager.id);
            let tex_loc = gl::GetUniformLocation(
                tex_shader_program.id(),
                b"tex\0".as_ptr() as *const _
            );
            gl::Uniform1i(tex_loc, 0);
        }
        bab.draw_arrays(
            gl::TRIANGLES,
            8, // vertex size for canvas
            vertices.line_end() as i32,
            vertices.len() as i32
        );

        // --- Swap the Window Buffer (display the frame) ---
        window.gl_swap_window();
    }
}

// --- Handle Player Input (WASD movement and rotation) ---
fn get_input(event_pump: &sdl2::EventPump, mut player: player::Player) -> player::Player {
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
        player.update_dir(player.get_dir() - 0.03);
        if player.get_dir() < 0.0 {
            player.update_dir(player.get_dir() + 2.0 * PI);
        }
        player.update_x_dir(player.get_dir().cos());
        player.update_y_dir(player.get_dir().sin());
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
        player.update_dir(player.get_dir() + 0.03);
        if player.get_dir() > 2.0 * PI {
            player.update_dir(player.get_dir() - 2.0 * PI);
        }
        player.update_x_dir(player.get_dir().cos());
        player.update_y_dir(player.get_dir().sin());
    }
    // --- Use try_move_player for collision-aware movement ---
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
        let dx = player.get_x_dir() * 1.1;
        let dy = player.get_y_dir() * 1.1;
        try_move_player(&mut player, dx, dy);
    }
    if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
        let dx = -player.get_x_dir() * 1.1;
        let dy = -player.get_y_dir() * 1.1;
        try_move_player(&mut player, dx, dy);
    }
    player
}

// --- Attempt to Move Player (collision detection and response) ---
fn try_move_player(player: &mut player::Player, dx: f32, dy: f32) {
    // Map constants (adjust if your map is not 8x8 or MAP_S is not 64)
    let map = window_gl::MAP;
    let map_s = window_gl::MAP_S as f32;

    // Intended new position
    let mut new_x = player.x_pos + dx;
    let mut new_y = player.y_pos + dy;

    // Calculate map cell indices for intended position
    let mut cell_x: usize = (new_x / map_s) as usize;
    let mut cell_y = (new_y / map_s) as usize;

    if dx > 0.0 {
        cell_x = ((new_x + 8.0) / map_s) as usize;
    }
    if dy > 0.0 {
        cell_y = ((new_y + 8.0) / map_s) as usize;
    }

    // Check bounds
    if cell_x >= map[0].len() || cell_y >= map.len() {
        return; // Out of bounds, don't move
    }

    // If the intended cell is a wall, snap to the edge
    if map[cell_y][cell_x] == 1 {
        // Snap X
        if map[(player.y_pos / map_s) as usize][cell_x] == 1 {
            // Blocked in X direction, snap to edge
            if dx > 0.0 {
                new_x = (cell_x as f32) * map_s - 8.01;
            } else if dx < 0.0 {
                new_x = ((cell_x as f32) + 1.0) * map_s + 0.01;
            } else {
                new_x = player.x_pos;
            }
        }
        // Snap Y
        if map[cell_y][(player.x_pos / map_s) as usize] == 1 {
            // Blocked in Y direction, snap to edge
            if dy > 0.0 {
                new_y = (cell_y as f32) * map_s - 8.01;
            } else if dy < 0.0 {
                new_y = ((cell_y as f32) + 1.0) * map_s + 0.01;
            } else {
                new_y = player.y_pos;
            }
        }
    }

    // If not colliding, or after snapping, update position
    player.update_pos(new_x, new_y);
}
