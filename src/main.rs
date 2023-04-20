use glfw::{Action, Context, Key};
use glad_gl::gl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(1200, 800, "rust-gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 1200, 800);

        let mut vertex_shader_source = std::fs::read_to_string("shaders/vertex.glsl")?;
        vertex_shader_source.push('\0');
        let vertex_shader_source = std::ffi::CStr::from_bytes_with_nul(vertex_shader_source.as_bytes())?;

        let mut frag_shader_source = std::fs::read_to_string("shaders/frag.glsl")?;
        frag_shader_source.push('\0');
        let frag_shader_source = std::ffi::CStr::from_bytes_with_nul(frag_shader_source.as_bytes())?;

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::CreateShader(vertex_shader);
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::CreateShader(frag_shader);
        gl::ShaderSource(frag_shader, 1, &frag_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(frag_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(frag_shader);


        while !window.should_close() {
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let vertices: [f32; 18] = [
                -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // left, red
                0.0,  0.5, 0.0, 0.0, 1.0, 0.0, // top, green
                0.5, -0.5, 0.0, 0.0, 0.0, 1.0, // right, blue
            ];
            let triangle_vao = prepare_triangle_vao(vertices);
            // let rect_vao = prepare_rect_vao();

            gl::UseProgram(shader_program);
            gl::BindVertexArray(triangle_vao);

            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            glfw.poll_events();
            window.swap_buffers();
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {}
    }
}

unsafe fn prepare_triangle_vao(vertices: [f32; 18]) -> u32 {
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

    // set the vertex attribute 'aPos' and enable it
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as i32, std::ptr::null());
    gl::EnableVertexAttribArray(0);

    // set the vertex attribute 'aColor' and enable it
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
    gl::EnableVertexAttribArray(1);

    // unbind the VBO and VAO
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    return vao;
}

unsafe fn prepare_rect_vao() -> u32 {
    let vertices: [f32; 12] = [
         0.5, 0.5, 0.0, // top right
         -0.5, 0.5, 0.0, // top left
         0.5, -0.5, 0.0, // bottom right
         -0.5, -0.5, 0.0, // bottom left
    ];

    let indices: [u32; 6] = [
        0, 1, 2,
        1, 2, 3,
    ];

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
    gl::EnableVertexAttribArray(0);

    // unbind VBO, EBO and VAO
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
    // EBO should NOT be unbound while a VAO is bound as the bound element buffer object IS stored in the VAO
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

    return vao;
}
