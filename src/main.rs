extern crate gl;
extern crate sdl2;

pub mod render_gl;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 700;

const MAP_X: i32 = 8;
const MAP_Y: i32 = 8;
const MAP_S: i32 = 64;

static mut MAP: [i32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

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

    // set up shader program

    use std::ffi::CString;
    let vert_shader = render_gl::Shader
        ::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
        .unwrap();

    let frag_shader = render_gl::Shader
        ::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
        .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up vertex buffer object

    let vertices: Vec<f32> = vec![-1.0, 1.0, 0.0, 0.0, -1.0, 0.0, 1.0, 1.0, 0.0];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, WIDTH, HEIGHT);
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
                3 // number of indices to be rendered
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
    return (pos_y - offset) / offset;
}

fn construct_verticies(player: [f32; 4]) -> Vec<f32> {
    let mut verticies: Vec<f32> = Vec::new();
    for i in 0..7 {
        for ii in 0..7 {
            if MAP[i][ii] == 1 {
                verticies.push(get_x((i as i32 * MAP_S) as f32));
                verticies.push(get_y((ii as i32 * MAP_S) as f32));
            }
        }
    }

    return verticies;
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
        let tl_point = [get_x((index_1 * MAP_S) as f32), get_y((index_2 * MAP_S) as f32), 0.0];
        let tr_point = [
            get_x((index_1 * MAP_S + MAP_S) as f32),
            get_y((index_2 * MAP_S) as f32),
            0.0,
        ];
        let bl_point = [
            get_x((index_1 * MAP_S) as f32),
            get_y((index_2 * MAP_S + MAP_S) as f32),
            0.0,
        ];
        let br_point = [
            get_x((index_1 * MAP_S + MAP_S) as f32),
            get_y((index_2 * MAP_S + MAP_S) as f32),
            0.0,
        ];
        let color = [color.r, color.g, color.b];

        Square { tl_point, tr_point, bl_point, br_point, color }
    }
}