use glad_gl::gl;
use glow::HasContext;

pub struct Shader {
    pub program: glow::NativeProgram,
}

impl Shader {
    pub unsafe fn new(gl: &glow::Context, vertex_path: &str, frag_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut vertex_shader_source = std::fs::read_to_string(vertex_path)?;
        vertex_shader_source.push('\0');
        // let vertex_shader_source = std::ffi::CStr::from_bytes_with_nul(vertex_shader_source.as_bytes())?;

        let mut frag_shader_source = std::fs::read_to_string(frag_path)?;
        frag_shader_source.push('\0');
        // let frag_shader_source = std::ffi::CStr::from_bytes_with_nul(frag_shader_source.as_bytes())?;

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)?;
        gl.shader_source(vertex_shader, &vertex_shader_source);
        gl.compile_shader(vertex_shader);
        let vert_status = gl.get_shader_compile_status(vertex_shader);
        println!("vertex shader {:?} compiled with status: {}",
            std::path::Path::new(vertex_path).file_name().unwrap(),
            vert_status
        );

        // let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        // gl::CreateShader(vertex_shader);
        // gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
        // gl::CompileShader(vertex_shader);
        // let mut success1 = 0;
        // gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success1);
        // println!("vertex shader {:?} compiled with status: {}",
        //     std::path::Path::new(vertex_path).file_name().unwrap(),
        //     success1
        // );
        // TODO: Check for errors

        let frag_shader = gl.create_shader(glow::FRAGMENT_SHADER)?;
        gl.shader_source(frag_shader, &frag_shader_source);
        gl.compile_shader(frag_shader);
        let frag_status = gl.get_shader_compile_status(frag_shader);
        println!("frag shader {:?} compiled with status: {}",
            std::path::Path::new(frag_path).file_name().unwrap(),
            frag_status
        );

        // let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        // gl::CreateShader(frag_shader);
        // gl::ShaderSource(frag_shader, 1, &frag_shader_source.as_ptr(), std::ptr::null());
        // gl::CompileShader(frag_shader);

        // let mut success2 = 0;
        // gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut success2);
        // println!("frag shader {:?} compiled with status: {}",
        //     std::path::Path::new(frag_path).file_name().unwrap(),
        //     success2
        // );
        // TODO: Check for errors

        let shader_program = gl.create_program()?;
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, frag_shader);
        gl.link_program(shader_program);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(frag_shader);
        // let shader_program = gl::CreateProgram();
        // gl::AttachShader(shader_program, vertex_shader);
        // gl::AttachShader(shader_program, frag_shader);
        // gl::LinkProgram(shader_program);

        // gl::DeleteShader(vertex_shader);
        // gl::DeleteShader(frag_shader);

        Ok(Self {
            program: shader_program,
        })
    }

    pub unsafe fn use_shader(&self, gl: &glow::Context) {
        gl.use_program(Some(self.program));
        // gl::UseProgram(self.program);
    }

    pub fn set_bool(&self, gl: &glow::Context, name: &str, value: bool) {
        unsafe {
            // let c_str = std::ffi::CString::new(name).unwrap();
            let loc = gl.get_uniform_location(self.program, name).unwrap();
            gl.uniform_1_i32(Some(&loc), value as i32)
            // gl::Uniform1i(gl::GetUniformLocation(self.program, c_str.as_ptr()), value as i32);
        }
    }

    pub fn set_int(&self, gl: &glow::Context, name: &str, value: i32) {
        unsafe {
            // let c_str = std::ffi::CString::new(name).unwrap();
            let loc = gl.get_uniform_location(self.program, name).unwrap();
            gl.uniform_1_i32(Some(&loc), value as i32);
            // gl::Uniform1i(gl::GetUniformLocation(self.program, c_str.as_ptr()), value);
        }
    }

    pub fn set_float(&self, gl: &glow::Context, name: &str, value: f32) {
        unsafe {
            // let c_str = std::ffi::CString::new(name).unwrap();
            let loc = gl.get_uniform_location(self.program, name).unwrap();
            gl.uniform_1_f32(Some(&loc), value);
            // gl::Uniform1f(gl::GetUniformLocation(self.program, c_str.as_ptr()), value);
        }
    }

    pub fn get_float(&self, gl: &glow::Context, name: &str) -> [f32; 1] {
        unsafe {
            // let c_str = std::ffi::CString::new(name).unwrap();
            let loc = gl.get_uniform_location(self.program, name).unwrap();
            let mut value: f32 = 0.0;
            let slice_val = std::slice::from_mut(&mut value);
            gl.get_uniform_f32(self.program, &loc, slice_val);
            // gl::GetUniformfv(self.program, gl::GetUniformLocation(self.program, c_str.as_ptr()), &mut value);
            return [value];
        }
    }

    pub unsafe fn set_mat4fv(&self, gl: &glow::Context, name: &str, value: &glm::Mat4) {
        // let c_str = std::ffi::CString::new(name).unwrap();
        let loc = gl.get_uniform_location(self.program, name).unwrap();
        let slice_val = std::slice::from_raw_parts(value.as_array().as_ptr() as *const f32, 16);
        gl.uniform_matrix_4_f32_slice(Some(&loc), false, slice_val);
        // gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, c_str.as_ptr()), 1, gl::FALSE, value.as_array().as_ptr() as *const f32);
    }

    pub unsafe fn set_3fv(&self, gl: &glow::Context, name: &str, value: glm::Vec3) {
        // let c_str = std::ffi::CString::new(name).unwrap();
        let loc = gl.get_uniform_location(self.program, name).unwrap();
        gl.uniform_3_f32(Some(&loc), value.x, value.y, value.z);
        // gl::Uniform3fv(gl::GetUniformLocation(self.program, c_str.as_ptr()), 1, value.as_array() as *const f32);
    }
}
