use glfw::{Action, Context, Key};
use glad_gl::gl;
use rust_gl::shader;
use image;

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

    let tex1 = image::io::Reader::open("textures/container.jpg")?.decode()?;
    let mut tex2 = image::io::Reader::open("textures/awesomeface.png")?.decode()?;
    tex2 = tex2.flipv();

    unsafe {
        gl::Viewport(0, 0, 1200, 800);

        let shader = shader::Shader::new("shaders/vertex.glsl", "shaders/frag.glsl")?;

        let vertices: [f32; 32] = [
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
            -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top left
            0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, // bottom left
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

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
        gl::EnableVertexAttribArray(2);

        let mut texture1: u32 = 0;
        let mut texture2: u32 = 0;

        // texture 1
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, tex1.width() as i32, tex1.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, tex1.as_bytes().as_ptr() as *const std::ffi::c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // texture 2
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, tex2.width() as i32, tex2.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, tex2.as_bytes().as_ptr() as *const std::ffi::c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader.use_shader();
        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);

        while !window.should_close() {
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event, &shader);
            }

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader.use_shader();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            glfw.poll_events();
            window.swap_buffers();
        }

        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, shader: &rust_gl::shader::Shader) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            shader.set_float("mixValue", shader.get_float("mixValue") + 0.1);
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            shader.set_float("mixValue", shader.get_float("mixValue") - 0.1);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {}
    }
}
