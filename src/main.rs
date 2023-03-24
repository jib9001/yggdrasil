extern crate gl;
extern crate sdl2;

pub mod render_gl;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;

const MAP_X: i32 = 8;
const MAP_Y: i32 = 8;
const MAP_S: i32 = 64;

static MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

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

    // set up shader program

    use std::ffi::CString;

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

    //create player
    let player: Player = Player::new(200.0, 200.0);

    // set up vertex buffer object
    let vertices: Vec<f32> = construct_vertices(player);

    for num in &vertices {
        println!("{}", num);
    }

    // create vertex buffer object
    let mut vbo: gl::types::GLuint = 0;
    // create opengl buffer from buffer object
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        // bind the buffer to the opengl context
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // tell opengl how to interpret the buffer data
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW // usage
        );
        // I don't know why we do this, but it might need an additional buffer?
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object
    // create vertex array object
    let mut vao: gl::types::GLuint = 0;
    // create vertex array in opengl context from vao
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        // bind the vertex array to.... something in opengl
        gl::BindVertexArray(vao);
        // do more buffer binding, have to look into what this does to find out
        // why so many BindBuffer calls are necessary, if they are
        // maybe this sets the array buffer back to our object so we can make changes if necessary
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // enables vertex attributes in the shader
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        // bind the buffer to location 0, which I believe to be our shader program
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // do the same with the vertex array
        gl::BindVertexArray(0);
    }

    // set up shared state for window
    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                (vertices.len() / 6) as i32 // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}

fn get_x(pos_x: f32) -> f32 {
    let offset: f32 = (WIDTH as f32) / 2.0;
    return (pos_x - offset) / offset;
}

fn get_y(pos_y: f32) -> f32 {
    let offset: f32 = (HEIGHT as f32) / 2.0;
    return ((pos_y - offset) / offset) * -1.0;
}

fn construct_vertices(player: Player) -> Vec<f32> {
    let mut vertices: Vec<f32> = Vec::new();
    for i in 0..=7 {
        for ii in 0..=7 {
            if MAP[i][ii] == 1 {
                vertices = push_square_vertices(
                    vertices,
                    Square::new(i as i32, ii as i32, Color::new(1.0, 1.0, 1.0))
                );
            } else {
                vertices = push_square_vertices(
                    vertices,
                    Square::new(i as i32, ii as i32, Color::new(0.0, 0.0, 0.0))
                );
            }
        }
    }
    vertices = push_player_vertices(vertices, player);

    return vertices;
}

fn push_square_vertices<f32>(vertices: Vec<f32>, square: Square) -> Vec<f32> {
    for num in square.tl_point {
        vertices.push(num);
    }
    for num in square.color {
        vertices.push(num);
    }
    for num in square.tr_point {
        vertices.push(num);
    }
    for num in square.color {
        vertices.push(num);
    }
    for num in square.bl_point {
        vertices.push(num);
    }
    for num in square.color {
        vertices.push(num);
    }
    for num in square.bl_point {
        vertices.push(num);
    }
    for num in square.color {
        vertices.push(num);
    }
    for num in square.br_point {
        vertices.push(num);
    }
    for num in square.color {
        vertices.push(num);
    }
    for num in square.tr_point {
        vertices.push(num);
    }
    for num in square.color {
        vertices.push(num);
    }

    return vertices;
}

fn push_player_vertices<T>(vertices: Vec<f32>, player: Player) -> Vec<f32> {
    for num in player.tl_point {
        vertices.push(num);
    }
    for num in player.color {
        vertices.push(num);
    }
    for num in player.tr_point {
        vertices.push(num);
    }
    for num in player.color {
        vertices.push(num);
    }
    for num in player.bl_point {
        vertices.push(num);
    }
    for num in player.color {
        vertices.push(num);
    }
    for num in player.bl_point {
        vertices.push(num);
    }
    for num in player.color {
        vertices.push(num);
    }
    for num in player.br_point {
        vertices.push(num);
    }
    for num in player.color {
        vertices.push(num);
    }
    for num in player.tr_point {
        vertices.push(num);
    }
    for num in player.color {
        vertices.push(num);
    }

    return vertice;
}

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        let r = red;
        let g = green;
        let b = blue;

        Color { r, g, b }
    }
}

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
            get_x((index_1 * MAP_S + 1) as f32),
            get_y((index_2 * MAP_S + 1) as f32),
            0.0,
        ];
        let tr_point = [
            get_x((index_1 * MAP_S + MAP_S - 1) as f32),
            get_y((index_2 * MAP_S + 1) as f32),
            0.0,
        ];
        let bl_point = [
            get_x((index_1 * MAP_S + 1) as f32),
            get_y((index_2 * MAP_S + MAP_S - 1) as f32),
            0.0,
        ];
        let br_point = [
            get_x((index_1 * MAP_S + MAP_S - 1) as f32),
            get_y((index_2 * MAP_S + MAP_S - 1) as f32),
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

pub struct Player {
    x_pos: f32,
    y_pos: f32,
    tl_point: [f32; 3],
    tr_point: [f32; 3],
    bl_point: [f32; 3],
    br_point: [f32; 3],
    color: [f32; 3],
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        let x_pos = x;
        let y_pos = y;
        let tl_point = [get_x(x), get_y(y), 0.0];
        let tr_point = [get_x(x + 8.0), get_y(y), 0.0];
        let bl_point = [get_x(x), get_y(y + 8.0), 0.0];
        let br_point = [get_x(x + 8.0), get_y(y + 8.0), 0.0];
        let color = [0.0, 0.0, 1.0];

        Player {
            x_pos,
            y_pos,
            tl_point,
            tr_point,
            bl_point,
            br_point,
            color,
        }
    }
}