// Import OpenGL bindings and standard library utilities
use gl;
use std;
use std::ffi::{CStr, CString};

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
                    error.as_ptr() as *mut gl::types::GLchar,
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
                error.as_ptr() as *mut gl::types::GLchar,
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
    unsafe { CString::from_vec_unchecked(buffer) }
}