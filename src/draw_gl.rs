// Import OpenGL bindings
extern crate gl;

// Struct to manage OpenGL buffer and vertex array objects
pub struct BufferArrayBinder {
    vao: gl::types::GLuint, // Vertex Array Object (VAO)
    vbo: gl::types::GLuint, // Vertex Buffer Object (VBO)
}

impl BufferArrayBinder {
    // Constructor to create a new BufferArrayBinder
    pub fn new(vao: gl::types::GLuint, vbo: gl::types::GLuint) -> BufferArrayBinder {
        BufferArrayBinder { vao, vbo }
    }

    // Set up the buffer with vertex data
    pub fn set_buffers(&mut self, vertices: &Vec<f32>) {
        unsafe {
            // Generate a buffer for the VBO
            gl::GenBuffers(1, &mut self.vbo);

            // Bind the buffer to the OpenGL context
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // Upload the vertex data to the buffer
            gl::BufferData(
                gl::ARRAY_BUFFER, // Target buffer type
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // Size of the data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // Pointer to the vertex data
                gl::STATIC_DRAW // Usage hint (data will not change often)
            );

            // Generate a Vertex Array Object (VAO)
            gl::GenVertexArrays(1, &mut self.vao);

            // Bind the VAO to the buffer
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    // Configure vertex attributes for the shader
    pub fn set_vertex_attribs(&mut self, vertex_size: usize, stride: usize, frag_size: usize) {
        unsafe {
            // Enable the vertex attribute for position (layout location 0 in the shader)
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, // Index of the vertex attribute
                vertex_size as i32, // Number of components per vertex attribute
                gl::FLOAT, // Data type of each component
                gl::FALSE, // Normalize the data (false for floats)
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // Stride (byte offset between consecutive attributes)
                std::ptr::null() // Offset of the first component
            );

            // Enable the vertex attribute for color (layout location 1 in the shader)
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, // Index of the vertex attribute
                frag_size as i32, // Number of components per vertex attribute
                gl::FLOAT, // Data type of each component
                gl::FALSE, // Normalize the data (false for floats)
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // Stride (byte offset between consecutive attributes)
                (vertex_size * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // Offset of the first component
            );
        }
    }

    // Draw the vertex data as arrays
    pub fn draw_arrays(&self, mode: gl::types::GLenum, num_of_indicies: i32, start: i32, end: i32) {
        unsafe {
            // Bind the VAO
            gl::BindVertexArray(self.vao);

            // Draw the arrays
            gl::DrawArrays(
                mode, // Drawing mode (e.g., GL_TRIANGLES, GL_LINES)
                start / num_of_indicies, // Starting index in the enabled arrays
                ((end - start) / num_of_indicies) as i32 // Number of indices to render
            );
        }
        //print!("Drawing from {} to {}.\n", start, end); // Print the range of indices being drawn
    }
}

// --- Vertex Array Wrapper Struct and Methods ---
pub struct VertexArrayWrapper {
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

// Struct to represent a color (RGB)
pub struct Color {
    r: f32, // Red component
    g: f32, // Green component
    b: f32, // Blue component
}

impl Color {
    // Constructor to create a new Color
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { r: red, g: green, b: blue }
    }

    // Get the color as an array
    pub fn get_colors(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

// Convert an x-coordinate to normalized OpenGL space
pub fn get_x(pos_x: f32, width: u32) -> f32 {
    let offset: f32 = (width as f32) / 2.0; // Calculate the center of the screen
    (pos_x - offset) / offset // Normalize the x-coordinate
}

// Convert a y-coordinate to normalized OpenGL space
pub fn get_y(pos_y: f32, height: u32) -> f32 {
    let offset: f32 = (height as f32) / 2.0; // Calculate the center of the screen
    ((pos_y - offset) / offset) * -1.0 // Normalize the y-coordinate and invert it
}

pub struct TextureManager {
    pub id: gl::types::GLuint, // OpenGL ID for the texture
}
impl TextureManager {
    // Constructor to create a new TextureManager
    pub fn new() -> TextureManager {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id); // Generate a texture ID
        }
        TextureManager { id }
    }

    // Load a texture from a file
    pub fn load_texture(&self, pixels: [[[u8; 3]; 60]; 60]) -> Result<(), String> {
        let mut flat_pixels = Vec::with_capacity(60 * 60 * 3);
        for row in pixels {
            for pixel in row {
                flat_pixels.extend_from_slice(&pixel);
            }
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // Upload the pixel data
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                60,
                60,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                flat_pixels.as_ptr() as *const _
            );
        }
        Ok(())
    }
}
