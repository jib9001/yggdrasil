extern crate gl;

pub struct BufferArrayBinder {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
}

impl BufferArrayBinder {
    pub fn new(vao: gl::types::GLuint, vbo: gl::types::GLuint) -> BufferArrayBinder {
        let vao = vao;
        let vbo = vbo;

        BufferArrayBinder { vao, vbo }
    }

    pub fn set_buffers(&mut self, vertices: &Vec<f32>) {
        unsafe {
            gl::GenBuffers(1, &mut self.vbo);

            // bind the buffer to the opengl context
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            // tell opengl how to interpret the buffer data
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
            gl::GenVertexArrays(1, &mut self.vao);
            // bind the vertex array to the buffer
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn set_vertex_attribs(&mut self, vertex_size: usize, stride: usize, frag_size: usize) {
        unsafe {
            // enables vertex attributes in the shader
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute ("layout (location = 0)")
                vertex_size as i32, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );
            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1, // index of the generic vertex attribute ("layout (location = 1)")
                frag_size as i32, // the number of components per generic vertex attributes
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (vertex_size * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
        }
    }

    pub fn draw_arrays(&self, mode: gl::types::GLenum, num_of_indicies: i32, start: i32, end: i32) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                mode, // mode
                0, // starting index in the enabled arrays
                ((end - start) / num_of_indicies) as i32 // number of indices to be rendered
            );
        }
    }
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

    pub fn get_colors(&self) -> [f32; 3] {
        return [self.r, self.g, self.b];
    }
}

pub fn get_x(pos_x: f32, width: u32) -> f32 {
    let offset: f32 = (width as f32) / 2.0;
    return (pos_x - offset) / offset;
}

pub fn get_y(pos_y: f32, height: u32) -> f32 {
    let offset: f32 = (height as f32) / 2.0;
    return ((pos_y - offset) / offset) * -1.0;
}