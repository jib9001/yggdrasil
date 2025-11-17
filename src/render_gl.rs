// Import OpenGL bindings and standard library utilities
use gl;
use std;
use std::ffi::{ CStr, CString };
use crate::window_gl::{
    HEIGHT,
    WIDTH,
    MAP,
    MAP_S,
    MAP_X,
    MAP_Y,
    single_index_map,
    RENDER_X,
    RENDER_Y,
    RAYS_COUNT,
    FOV,
};
use std::f32::consts::PI;
use crate::draw_gl::{ get_x, get_y, VertexArrayWrapper, Color };
use crate::player;
use crate::square;
// Import the `draw_gl` module for drawing utilities

// Represents an OpenGL shader program
pub struct Program {
    id: gl::types::GLuint, // OpenGL ID for the program
}

impl Program {
    // Creates a new shader program from a list of shaders
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        // Create a new OpenGL program
        let program_id = unsafe { gl::CreateProgram() };

        // Attach each shader to the program
        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        // Link the program
        unsafe {
            gl::LinkProgram(program_id);
        }

        // Check if the linking was successful
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        // If linking failed, retrieve and return the error log
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        // Detach shaders after linking
        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    // Returns the OpenGL ID of the program
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    // Sets this program as the active program in OpenGL
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

// Automatically deletes the program when it goes out of scope
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

// Represents an OpenGL shader (vertex or fragment)
pub struct Shader {
    id: gl::types::GLuint, // OpenGL ID for the shader
}

impl Shader {
    // Creates a shader from source code and a shader type (e.g., vertex or fragment)
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    // Convenience function to create a vertex shader
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    // Convenience function to create a fragment shader
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    // Returns the OpenGL ID of the shader
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

// Automatically deletes the shader when it goes out of scope
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

// Compiles a shader from source code
fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    // Create a new shader of the specified type
    let id = unsafe { gl::CreateShader(kind) };

    // Provide the source code to OpenGL and compile the shader
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    // Check if the compilation was successful
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // If compilation failed, retrieve and return the error log
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

// Creates a CString filled with whitespace of the specified length
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Allocate a buffer of the specified size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // Fill the buffer with spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // Convert the buffer to a CString
    unsafe {
        CString::from_vec_unchecked(buffer)
    }
}

// --- Construct Vertices for All Geometry (map, player, lines, rays, canvas) ---
pub fn construct_vertices(
    player: &player::Player,
    mut vertices: &mut VertexArrayWrapper,
    hrays: &mut [f32; RAYS_COUNT as usize],
    vrays: &mut [f32; RAYS_COUNT as usize],
    _is_log: i32
) {
    // Map squares
    for i in 0..=7 {
        for ii in 0..=7 {
            if MAP[i][ii] == 1 {
                push_square_vertices(
                    &mut vertices,
                    square::Square::new(ii as i32, i as i32, Color::new(1.0, 1.0, 1.0))
                );
            } else {
                push_square_vertices(
                    &mut vertices,
                    square::Square::new(ii as i32, i as i32, Color::new(0.0, 0.0, 0.0))
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

// --- Create Canvas Quad Vertices (for displaying the texture) ---
fn create_canvas(vertices: &mut VertexArrayWrapper) {
    let points = [
        [get_x(513.0, WIDTH), get_y(0.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 0.0, 0.0], // top-left
        [get_x(1537.0, WIDTH), get_y(0.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 1.0, 0.0], // top-right
        [get_x(513.0, WIDTH), get_y(512.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 0.0, 1.0], // bottom-left
        [get_x(1537.0, WIDTH), get_y(512.0, HEIGHT), 0.0, 1.0, 1.0, 1.0, 1.0, 1.0], // bottom-right
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
    hrays: &mut [f32; RAYS_COUNT as usize],
    vrays: &mut [f32; RAYS_COUNT as usize],
    _is_log: i32
) {
    let map = single_index_map();
    let dr: f32 = FOV / (RAYS_COUNT as f32); // Ray angle increment scales with FOV and ray count
    let mut mx: i32;
    let mut my: i32;
    let mut _mp: i32;
    let mut dof: i32;

    let mut rx: f32;
    let mut ry: f32;
    let mut xo: f32;
    let mut yo: f32;
    let mut ra: f32 = player.get_dir() - dr * ((RAYS_COUNT as f32) / 2.0); // Start angle for rays

    for _r in 0..RAYS_COUNT {
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

        hrays[_r as usize] = distance_3d(
            (player.x_pos + 4.0, player.y_pos + 4.0, 0.0),
            (rx, ry, 0.0)
        );

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

        vrays[_r as usize] = distance_3d(
            (player.x_pos + 4.0, player.y_pos + 4.0, 0.0),
            (rx, ry, 0.0)
        );

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

// --- Raycasting: Draw Walls to Pixel Buffer (with fisheye correction) ---
pub fn draw_walls_to_pixels(
    _pixels: &mut [[[u8; 3]; RENDER_X as usize]; RENDER_Y as usize],
    hrays: &[f32; RAYS_COUNT as usize],
    vrays: &[f32; RAYS_COUNT as usize],
    horiz_color: [u8; 3],
    vert_color: [u8; 3],
    background_color: [u8; 3]
) {
    let screen_height = RENDER_Y;
    let screen_width = RENDER_X;
    let proj_plane_dist = (screen_width as f32) / 2.0 / (FOV / 2.0).tan();
    let wall_height_world = 1.0;

    for x in 0..screen_width {
        // Map screen column to ray index (since we may have different ray count vs screen width)
        let ray_index = (((x as f32) * (RAYS_COUNT as f32)) / (screen_width as f32)) as usize;
        let ray_index = ray_index.min((RAYS_COUNT as usize) - 1); // Clamp to array bounds

        let h_dist = hrays[ray_index].max(0.0001);
        let v_dist = vrays[ray_index].max(0.0001);

        // Use the shorter distance for wall height
        let (raw_dist, color) = if h_dist < v_dist {
            (h_dist, horiz_color)
        } else {
            (v_dist, vert_color)
        };

        // --- Better fisheye correction: use screen-space angle calculation ---
        let screen_angle =
            (((x as f32) - (screen_width as f32) / 2.0) / ((screen_width as f32) / 2.0)) *
            (FOV / 2.0);
        let dist = raw_dist * screen_angle.cos();

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
                _pixels[y as usize][x as usize] = background_color; // Ceiling
            } else if (y as i32) >= wall_top && (y as i32) < wall_bottom {
                _pixels[y as usize][x as usize] = color; // Wall
            } else {
                _pixels[y as usize][x as usize] = background_color; // Floor
            }
        }
        // Uncomment for debugging wall heights:
        // println!("x: {}, wall_height: {}", x, wall_height);
    }
}
