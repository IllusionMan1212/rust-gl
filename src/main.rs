use glfw::{Action, Context, Key};
use glad_gl::gl;
use image;

use rust_gl::{shader::*, camera::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(800, 600, "rust-gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.make_current();

    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let tex1 = image::io::Reader::open("textures/container.jpg")?.decode()?;
    let mut tex2 = image::io::Reader::open("textures/awesomeface.png")?.decode()?;
    tex2 = tex2.flipv();

    unsafe {
        gl::Viewport(0, 0, 800, 600);

        let shader = Shader::new("shaders/vertex.glsl", "shaders/frag.glsl")?;
        let shader2 = Shader::new("shaders/grid_v.glsl", "shaders/grid_f.glsl")?;

        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5, 0.0, 0.0,
            0.5, -0.5, -0.5, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 1.0,
            -0.5, 0.5, 0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            0.5, -0.5, -0.5, 1.0, 1.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0,
            0.5, 0.5, -0.5, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, 0.5, 0.0, 0.0,
            -0.5, 0.5, -0.5, 0.0, 1.0
                ];

        let cubes: [glm::Vec3; 10] = [
            glm::vec3( 0.0, 0.0, 0.0),
            glm::vec3( 2.0, 5.0, -15.0),
            glm::vec3(-1.5, -2.2, -2.5),
            glm::vec3(-3.8, -2.0, -12.3),
            glm::vec3( 2.4, -0.4, -3.5),
            glm::vec3(-1.7, 3.0, -7.5),
            glm::vec3( 1.3, -2.0, -2.5),
            glm::vec3( 1.5, 2.0, -2.5),
            glm::vec3( 1.5, 0.2, -1.5),
            glm::vec3(-1.3, 1.0, -1.5)
        ];

        // let indices: [u32; 6] = [
        //     0, 1, 2,
        //     1, 2, 3,
        // ];

        let ident_mat = glm::mat4(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.);

        let mut delta_time: f32 = 0.0;
        let mut last_frame: f32 = 0.0;
        let camera_speed: f32 = 10.0;
        let camera_sensitivity: f32 = 0.05;

        let (w, h) = window.get_size();
        let mut last_x: f32 = w as f32 / 2.0;
        let mut last_y: f32 = h as f32 / 2.0;
        let mut first_mouse: bool = true;

        // Object 1: Cube/container
        //
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        // let mut ebo: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

        // position attribute, maps to 'aPos' in vertex shader with location 0
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // texture coords attribute, maps to 'aTexCoords' in vertex shader with location 1
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
        gl::EnableVertexAttribArray(1);

        // gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
        // gl::EnableVertexAttribArray(2);

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

        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);


        // End of Object 1

        // Object 2: Grid
  
        let vertices2: [f32; 6] = [
            -1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
        ];

        let mut vao2: u32 = 0;
        let mut vbo2: u32 = 0;

        gl::GenBuffers(1, &mut vbo2);
        gl::GenVertexArrays(1, &mut vao2);

        gl::BindVertexArray(vao2);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices2.len() * std::mem::size_of::<f32>()) as isize, vertices2.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // End of Object 2

        let mut camera = Camera::new(camera_speed, camera_sensitivity);

        while !window.should_close() {
            let current_frame = glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            camera.update_speed(delta_time);
            camera.handle_keyboard(&mut window);

            // camera matrices
            let view_mat = glm::ext::look_at(camera.position, camera.position + camera.front, camera.up);
            let (win_width, win_height) = window.get_size();
            let projection_mat = glm::ext::perspective(glm::radians(camera.fov), win_width as f32 / win_height as f32, 0.1, 100.0);

            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, &event, &shader, &mut camera);
                match event {
                    glfw::WindowEvent::CursorPos(xpos, ypos) => {
                        if first_mouse {
                            last_x = xpos as f32;
                            last_y = ypos as f32;
                            first_mouse = false;
                        }

                        let xoffset = xpos as f32 - last_x;
                        let yoffset = last_y - ypos as f32;
                        last_x = xpos as f32;
                        last_y = ypos as f32;

                        camera.handle_mouse_input(xoffset, yoffset);
                    }
                    _ => {}
                }
            }

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // drawing objects
            shader.use_shader();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader.set_mat4fv("view", &view_mat);
            shader.set_mat4fv("projection", &projection_mat);

            for i in 0..cubes.len() {
                let model_mat = glm::ext::translate(&ident_mat, cubes[i]);
                let angle = 20.0 * i as f32;
                let model_mat = glm::ext::rotate(&model_mat, glfw.get_time() as f32 * glm::radians(angle), glm::vec3(1.0, 0.3, 0.5));
                shader.set_mat4fv("model", &model_mat);

                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            shader2.use_shader();
            shader2.set_mat4fv("view", &view_mat);
            shader2.set_mat4fv("projection", &projection_mat);

            for i in -100..100 {
                if i == 0 {
                    shader2.set_3fv("color", glm::vec3(1.0, 0.0, 0.0));
                } else {
                    shader2.set_3fv("color", glm::vec3(0.4, 0.4, 0.4));
                }

                let model_mat = glm::ext::translate(&ident_mat, glm::vec3(0.0, 0.0, i as f32));
                let model_mat = glm::ext::scale(&model_mat, glm::vec3(1000.0 * 1000.0, 0.0, 0.0));

                shader2.set_mat4fv("model", &model_mat);

                gl::BindVertexArray(vao2);
                gl::DrawArrays(gl::LINES, 0, 2);

                if i == 0 {
                    shader2.set_3fv("color", glm::vec3(0.0, 1.0, 0.0));
                } else {
                    shader2.set_3fv("color", glm::vec3(0.4, 0.4, 0.4));
                }

                let model_mat = glm::ext::translate(&ident_mat, glm::vec3(i as f32, 0.0, 0.0));
                let model_mat = glm::ext::rotate(&model_mat, glm::radians(90.0), glm::vec3(0.0, 1.0, 0.0));
                let model_mat = glm::ext::scale(&model_mat, glm::vec3(1000.0 * 1000.0, 0.0, 0.0));
                shader2.set_mat4fv("model", &model_mat);

                gl::BindVertexArray(vao2);
                gl::DrawArrays(gl::LINES, 0, 2);
            }

            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            glfw.poll_events();
            window.swap_buffers();
        }

        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteVertexArrays(1, &vao2);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &vbo2);
        // gl::DeleteBuffers(1, &ebo);
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: &glfw::WindowEvent, shader: &rust_gl::shader::Shader, camera: &mut Camera) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            println!("up");
            unsafe {
                shader.use_shader();
                // no worky
                shader.set_float("mixValue", shader.get_float("mixValue") + 0.1);
            }
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            println!("down");
            unsafe {
                shader.use_shader();
                // no worky
                shader.set_float("mixValue", shader.get_float("mixValue") - 0.1);
            }
        }
        glfw::WindowEvent::Scroll(_, yoff) => {
            camera.handle_mouse_scroll(*yoff as f32);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, *width, *height);
            }
        }
        _ => {}
    }
}
