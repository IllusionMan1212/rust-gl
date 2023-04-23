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
        let mut success1 = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success1);
        println!("vertex shader compiled with status: {}", success1);
        // TODO: Check for errors

        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::CreateShader(frag_shader);
        gl::ShaderSource(frag_shader, 1, &frag_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(frag_shader);

        let mut success2 = 0;
        gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut success2);
        println!("frag shader compiled with status: {}", success2);
        // TODO: Check for errors

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(frag_shader);

        Ok(Self {
            program_id: shader_program,
        })
    }

    pub unsafe fn use_shader(&self) {
        gl::UseProgram(self.program_id);
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let c_str = std::ffi::CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let c_str = std::ffi::CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let c_str = std::ffi::CString::new(name).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), value);
        }
    }

    pub fn get_float(&self, name: &str) -> f32 {
        unsafe {
            let c_str = std::ffi::CString::new(name).unwrap();
            let mut value = 0.0;
            gl::GetUniformfv(self.program_id, gl::GetUniformLocation(self.program_id, c_str.as_ptr()), &mut value);
            value
        }
    }

    pub unsafe fn set_mat4fv(&self, name: &str, value: &glm::Mat4) {
        let c_str = std::ffi::CString::new(name).unwrap();
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), 1, gl::FALSE, value.as_array().as_ptr() as *const f32);
    }
}
