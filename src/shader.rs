use glad_gl::gl;

pub struct Shader {
    pub program_id: gl::GLuint,
}

impl Shader {
    pub unsafe fn new(vertex_path: &str, frag_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut vertex_shader_source = std::fs::read_to_string(vertex_path)?;
        vertex_shader_source.push('\0');
        let vertex_shader_source = std::ffi::CStr::from_bytes_with_nul(vertex_shader_source.as_bytes())?;

        let mut frag_shader_source = std::fs::read_to_string(frag_path)?;
        frag_shader_source.push('\0');
        let frag_shader_source = std::ffi::CStr::from_bytes_with_nul(frag_shader_source.as_bytes())?;

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::CreateShader(vertex_shader);
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);
        // TODO: Check for errors

        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::CreateShader(frag_shader);
        gl::ShaderSource(frag_shader, 1, &frag_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(frag_shader);
        // TODO: Check for errors

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);

        Ok(Self {
            program_id: shader_program,
        })
    }

    pub unsafe fn use_shader(&self) {
        gl::UseProgram(self.program_id);
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8), value);
        }
    }

}
