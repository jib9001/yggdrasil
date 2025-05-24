// --- External Crates ---
extern crate gl; // OpenGL bindings
extern crate sdl2; // SDL2 bindings

// --- Imports from Other Modules ---
use crate::window_gl::{ HEIGHT, WIDTH, MAP, MAP_S, MAP_X, MAP_Y, single_index_map };
use draw_gl::{ get_x, get_y };
use sdl2::keyboard::Scancode;
use std::f32::consts::PI;
use std::ffi::CString;

// --- Submodules ---
pub mod draw_gl;
pub mod log;
pub mod player;
pub mod render_gl;
pub mod square;
pub mod window_gl;

// --- Main Function ---
fn main() {
    // --- State Variables ---
    let mut _is_log = 0; // Toggle for logging/debugging

    // Pixel buffer for the raycasted scene (used as a texture)
    let mut _pixels: [[[u8; 3]; 60]; 60] = [[[0u8; 3]; 60]; 60];
    // Arrays to store horizontal and vertical ray distances for each column
    let mut hrays: [f32; 60] = [0.0; 60];
    let mut vrays: [f32; 60] = [0.0; 60];

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
        construct_vertices(&player, &mut vertices, &mut hrays, &mut vrays, _is_log);

        // --- Raycasting: Draw Walls to Pixel Buffer ---
        draw_walls_to_pixels(
            &mut _pixels,
            &hrays,
            &vrays,
            [200, 200, 200], // horizontal wall color (light gray)
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

// --- Construct Vertices for All Geometry (map, player, lines, rays, canvas) ---
fn construct_vertices(
    player: &player::Player,
    mut vertices: &mut VertexArrayWrapper,
    hrays: &mut [f32; 60],
    vrays: &mut [f32; 60],
    _is_log: i32
) {
    // Map squares
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
    // Player quad
    push_player_vertices(&mut vertices, player);
    // Player direction line
    push_line_vertices(&mut vertices, player);
    // Ray lines and ray distances
    cast_rays(&mut vertices, player, hrays, vrays, _is_log);
    // Canvas quad (for displaying the raycasted texture)
    create_canvas(&mut vertices);
}

// --- Push Vertices for a Map Square (as two triangles) ---
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

// --- Create Canvas Quad Vertices (for displaying the texture) ---
fn create_canvas(vertices: &mut VertexArrayWrapper) {
    // Each vertex: [x, y, z, r, g, b, u, v]
    let points = [
        [get_x(513.0, WIDTH), get_y(0.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 0.0, 0.0], // top-left
        [get_x(1025.0, WIDTH), get_y(0.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 1.0, 0.0], // top-right
        [get_x(513.0, WIDTH), get_y(512.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 0.0, 1.0], // bottom-left
        [get_x(1025.0, WIDTH), get_y(512.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 1.0, 1.0], // bottom-right
    ];
    // First triangle
    for &i in &[0, 1, 2] {
        for &num in &points[i] {
            vertices.push(num);
        }
    }
    // Second triangle
    for &i in &[1, 3, 2] {
        for &num in &points[i] {
            vertices.push(num);
        }
    }
}

// --- Push Vertices for the Player Quad ---
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

// --- Push Vertices for the Player's Direction Line ---
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

// --- Raycasting: Cast Rays, Store Distances, and Push Ray Vertices ---
fn cast_rays(
    vertices: &mut VertexArrayWrapper,
    player: &player::Player,
    hrays: &mut [f32; 60],
    vrays: &mut [f32; 60],
    _is_log: i32
) {
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

            // Break if out of map bounds
            if mx < 0 || mx >= MAP_X || my < 0 || my >= MAP_Y {
                break;
            }

            // Stop if a wall is hit
            if (0..MAP_X * MAP_Y).contains(&_mp) && map[_mp as usize] == 1 {
                break;
            }

            // Step to next grid intersection
            rx += xo;
            ry += yo;
            dof += 1;
        }

        hrays[_r] = distance_3d((player.x_pos + 4.0, player.y_pos + 4.0, 0.0), (rx, ry, 0.0));

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

            // Break if out of map bounds
            if mx < 0 || mx >= MAP_X || my < 0 || my >= MAP_Y {
                break;
            }

            // Stop if a wall is hit
            if (0..MAP_X * MAP_Y).contains(&_mp) && map[_mp as usize] == 1 {
                break;
            }

            // Step to next grid intersection
            rx += xo;
            ry += yo;
            dof += 1;
        }

        vrays[_r] = distance_3d((player.x_pos + 4.0, player.y_pos + 4.0, 0.0), (rx, ry, 0.0));

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

    vertices.set_line_end(vertices.len());
}

// --- Utility: 3D Distance Calculation ---
fn distance_3d(begin: (f32, f32, f32), end: (f32, f32, f32)) -> f32 {
    let dx = end.0 - begin.0;
    let dy = end.1 - begin.1;
    let dz = end.2 - begin.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// --- Vertex Array Wrapper Struct and Methods ---
struct VertexArrayWrapper {
    points: Vec<f32>,
    triangle_end: usize,
    line_end: usize,
}

impl VertexArrayWrapper {
    pub fn new() -> VertexArrayWrapper {
        let points = Vec::new();
        let triangle_end = 0;
        let line_end = 0;
        VertexArrayWrapper {
            points,
            triangle_end,
            line_end,
        }
    }
    pub fn set_triangle_end(&mut self, end: usize) {
        self.triangle_end = end;
    }
    pub fn set_line_end(&mut self, end: usize) {
        self.line_end = end;
    }
    pub fn len(&self) -> usize {
        self.points.len()
    }
    pub fn push(&mut self, num: f32) {
        self.points.push(num);
    }
    pub fn points(&self) -> &Vec<f32> {
        &self.points
    }
    pub fn triangle_end(&self) -> usize {
        self.triangle_end
    }
    pub fn line_end(&self) -> usize {
        self.line_end
    }
}

// --- Raycasting: Draw Walls to Pixel Buffer (with fisheye correction) ---
fn draw_walls_to_pixels(
    _pixels: &mut [[[u8; 3]; 60]; 60],
    hrays: &[f32; 60],
    vrays: &[f32; 60],
    horiz_color: [u8; 3],
    vert_color: [u8; 3],
    background_color: [u8; 3]
) {
    let screen_height = 60;
    let screen_width = 60;
    let fov = std::f32::consts::FRAC_PI_3; // 60 degrees
    let proj_plane_dist = (screen_width as f32) / 2.0 / (fov / 2.0).tan();
    let wall_height_world = 1.0;

    for x in 0..screen_width {
        let h_dist = hrays[x].max(0.0001);
        let v_dist = vrays[x].max(0.0001);

        // Use the shorter distance for wall height
        let (raw_dist, color) = if h_dist < v_dist {
            (h_dist, horiz_color)
        } else {
            (v_dist, vert_color)
        };

        // --- Fisheye correction: project ray distance onto view direction ---
        let ray_angle_offset =
            ((x as f32) - (screen_width as f32) / 2.0) * (fov / (screen_width as f32));
        let dist = raw_dist * ray_angle_offset.cos();

        // Calculate projected wall height in pixels
        let mut wall_height = (20.0 * (wall_height_world * proj_plane_dist)) / dist;
        if wall_height > (screen_height as f32) {
            wall_height = screen_height as f32;
        }

        // Compute top and bottom of the wall slice
        let wall_top = (((screen_height as f32) - wall_height) / 2.0).round() as i32;
        let wall_bottom = (((screen_height as f32) + wall_height) / 2.0).round() as i32;

        // Fill the pixel buffer for this column
        for y in 0..screen_height {
            if (y as i32) < wall_top {
                _pixels[y][x] = background_color; // Ceiling
            } else if (y as i32) >= wall_top && (y as i32) < wall_bottom {
                _pixels[y][x] = color; // Wall
            } else {
                _pixels[y][x] = background_color; // Floor
            }
        }
        // Uncomment for debugging wall heights:
        // println!("x: {}, wall_height: {}", x, wall_height);
    }
}
