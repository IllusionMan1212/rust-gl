use glfw::{Action, Context, Key};
// use glad_gl::gl;
use image;
use std::time::Instant;
use std::rc::Rc;
use glow::HasContext;

use rust_gl::{shader::*, camera::*, imgui_glfw_support};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(800, 600, "rust-gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_all_polling(true);
    // window.set_key_polling(true);
    // window.set_framebuffer_size_polling(true);
    // window.set_cursor_mode(glfw::CursorMode::Disabled);
    // window.set_cursor_pos_polling(true);
    // window.set_scroll_polling(true);
    window.make_current();

    // gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let mut imgui_ctx = imgui::Context::create();
    imgui_ctx.set_ini_filename(None);
    imgui_ctx.io_mut().config_flags.insert(imgui::ConfigFlags::DOCKING_ENABLE);
    // imgui_ctx.io_mut().config_flags.set(imgui::ConfigFlags::NAV_ENABLE_KEYBOARD, true);

    let mut glfw_platform = imgui_glfw_support::GlfwPlatform::init(&mut imgui_ctx);
    glfw_platform.attach_window(
        imgui_ctx.io_mut(),
        &window,
        imgui_glfw_support::HiDpiMode::Rounded
    );

    imgui_ctx
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_ctx.io_mut().font_global_scale = (1.0 / glfw_platform.hidpi_factor()) as f32;

    let gl = glow_context(&mut window);

    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_ctx).expect("Failed to initialize imgui renderer");

    unsafe {
        glfw_platform.set_clipboard_backend(&mut imgui_ctx, &window);
        // gl::Enable(gl::DEPTH_TEST);
        ig_renderer.gl_context().enable(glow::DEPTH_TEST);
    }

    let tex1 = image::io::Reader::open("textures/container.jpg")?.decode()?;
    let mut tex2 = image::io::Reader::open("textures/awesomeface.png")?.decode()?;
    tex2 = tex2.flipv();

    unsafe {
        // gl::Viewport(0, 0, 800, 600);
        ig_renderer.gl_context().viewport(0, 0, 800, 600);

        let shader = Shader::new(ig_renderer.gl_context(), "shaders/vertex.glsl", "shaders/frag.glsl")?;
        let shader2 = Shader::new(ig_renderer.gl_context(), "shaders/grid_v.glsl", "shaders/grid_f.glsl")?;

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
        let mut last_cursor = None;
        let camera_speed: f32 = 10.0;
        let camera_sensitivity: f32 = 0.05;

        let (w, h) = window.get_size();
        let mut last_x: f32 = w as f32 / 2.0;
        let mut last_y: f32 = h as f32 / 2.0;
        let mut first_mouse: bool = true;

        // Object 1: Cube/container
        //
        // let mut vao: u32 = 0;
        // let mut vbo: u32 = 0;
        // let mut ebo: u32 = 0;
        let vao = ig_renderer.gl_context().create_vertex_array()?;
        let vbo = ig_renderer.gl_context().create_buffer()?;
        // gl::GenVertexArrays(1, &mut vao);
        // gl::GenBuffers(1, &mut vbo);
        // gl::GenBuffers(1, &mut ebo);

        ig_renderer.gl_context().bind_vertex_array(Some(vao));
        // gl::BindVertexArray(vao);

        ig_renderer.gl_context().bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        // gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertices_u8: &[u8] = core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<f32>(),
        );

        ig_renderer.gl_context().buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);
        // gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<u32>()) as isize, indices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

        // position attribute, maps to 'aPos' in vertex shader with location 0
        // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        // gl::EnableVertexAttribArray(0);
        ig_renderer.gl_context().vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 5 * std::mem::size_of::<f32>() as i32, 0);
        ig_renderer.gl_context().enable_vertex_attrib_array(0);

        // texture coords attribute, maps to 'aTexCoords' in vertex shader with location 1
        // gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
        // gl::EnableVertexAttribArray(1);
        ig_renderer.gl_context().vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as i32);
        ig_renderer.gl_context().enable_vertex_attrib_array(1);

        // gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const std::os::raw::c_void);
        // gl::EnableVertexAttribArray(2);

        // let mut texture1: u32 = 0;
        // let mut texture2: u32 = 0;

        // texture 1
        let texture1 = ig_renderer.gl_context().create_texture()?;
        // gl::GenTextures(1, &mut texture1);
        // gl::BindTexture(gl::TEXTURE_2D, texture1);

        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, tex1.width() as i32, tex1.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, tex1.as_bytes().as_ptr() as *const std::ffi::c_void);
        // gl::GenerateMipmap(gl::TEXTURE_2D);

        // // texture 2
        let texture2 = ig_renderer.gl_context().create_texture()?;
        // gl::GenTextures(1, &mut texture2);
        // gl::BindTexture(gl::TEXTURE_2D, texture2);

        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, tex2.width() as i32, tex2.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, tex2.as_bytes().as_ptr() as *const std::ffi::c_void);
        // gl::GenerateMipmap(gl::TEXTURE_2D);

        shader.set_int(ig_renderer.gl_context(), "texture1", 0);
        shader.set_int(ig_renderer.gl_context(), "texture2", 1);


        // End of Object 1

        // Object 2: Grid
  
        let vertices2: [f32; 6] = [
            -1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
        ];

        let vao2 = ig_renderer.gl_context().create_vertex_array()?;
        let vbo2 = ig_renderer.gl_context().create_buffer()?;
        // gl::GenBuffers(1, &mut vbo2);
        // gl::GenVertexArrays(1, &mut vao2);

        ig_renderer.gl_context().bind_vertex_array(Some(vao2));
        // gl::BindVertexArray(vao2);

        ig_renderer.gl_context().bind_buffer(glow::ARRAY_BUFFER, Some(vbo2));
        // gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        let vertices2_u8: &[u8] = core::slice::from_raw_parts(
            vertices2.as_ptr() as *const u8,
            vertices2.len() * core::mem::size_of::<f32>(),
        );

        ig_renderer.gl_context().buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertices2_u8, glow::STATIC_DRAW);
        // gl::BufferData(gl::ARRAY_BUFFER, (vertices2.len() * std::mem::size_of::<f32>()) as isize, vertices2.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);

        ig_renderer.gl_context().vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 3 * std::mem::size_of::<f32>() as i32, 0);
        // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        ig_renderer.gl_context().enable_vertex_attrib_array(0);
        // gl::EnableVertexAttribArray(0);

        // End of Object 2

        let mut camera = Camera::new(camera_speed, camera_sensitivity);

        while !window.should_close() {
            let current_frame = glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            camera.update_speed(delta_time);
            camera.handle_keyboard(&mut window);

            imgui_ctx.io_mut().update_delta_time(std::time::Duration::from_secs_f32(delta_time));

            ig_renderer.gl_context().clear_color(0.3, 0.3, 0.3, 1.0);
            ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // camera matrices
            let view_mat = glm::ext::look_at(camera.position, camera.position + camera.front, camera.up);
            let (win_width, win_height) = window.get_size();
            let projection_mat = glm::ext::perspective(glm::radians(camera.fov), win_width as f32 / win_height as f32, 0.1, 100.0);

            for (_, event) in glfw::flush_messages(&events) {
                glfw_platform.handle_event(imgui_ctx.io_mut(), &window, &event);
                handle_window_event(&mut window, &event, &shader, &mut camera, ig_renderer.gl_context());
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

            // gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // drawing objects
            shader.use_shader(ig_renderer.gl_context());

            ig_renderer.gl_context().active_texture(glow::TEXTURE0);
            ig_renderer.gl_context().bind_texture(glow::TEXTURE_2D, Some(texture1));
            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, texture1);
            ig_renderer.gl_context().active_texture(glow::TEXTURE1);
            ig_renderer.gl_context().bind_texture(glow::TEXTURE_2D, Some(texture2));
            // gl::ActiveTexture(gl::TEXTURE1);
            // gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader.set_mat4fv(ig_renderer.gl_context(), "view", &view_mat);
            shader.set_mat4fv(ig_renderer.gl_context(), "projection", &projection_mat);

            for i in 0..cubes.len() {
                let model_mat = glm::ext::translate(&ident_mat, cubes[i]);
                let angle = 20.0 * i as f32;
                let model_mat = glm::ext::rotate(&model_mat, glfw.get_time() as f32 * glm::radians(angle), glm::vec3(1.0, 0.3, 0.5));
                shader.set_mat4fv(ig_renderer.gl_context(), "model", &model_mat);

                ig_renderer.gl_context().bind_vertex_array(Some(vao));
                ig_renderer.gl_context().draw_arrays(glow::TRIANGLES, 0, 36);
                // gl::BindVertexArray(vao);
                // gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            shader2.use_shader(ig_renderer.gl_context());
            shader2.set_mat4fv(ig_renderer.gl_context(), "view", &view_mat);
            shader2.set_mat4fv(ig_renderer.gl_context(), "projection", &projection_mat);

            for i in -100..100 {
                if i == 0 {
                    shader2.set_3fv(ig_renderer.gl_context(), "color", glm::vec3(1.0, 0.2, 0.2));
                } else {
                    shader2.set_3fv(ig_renderer.gl_context(), "color", glm::vec3(0.4, 0.4, 0.4));
                }

                let model_mat = glm::ext::translate(&ident_mat, glm::vec3(0.0, 0.0, i as f32));
                let model_mat = glm::ext::scale(&model_mat, glm::vec3(1000.0 * 1000.0, 0.0, 0.0));

                shader2.set_mat4fv(ig_renderer.gl_context(), "model", &model_mat);

                ig_renderer.gl_context().bind_vertex_array(Some(vao2));
                ig_renderer.gl_context().draw_arrays(glow::LINES, 0, 2);
                // gl::BindVertexArray(vao2);
                // gl::DrawArrays(gl::LINES, 0, 2);

                if i == 0 {
                    shader2.set_3fv(ig_renderer.gl_context(), "color", glm::vec3(0.0, 1.0, 0.0));
                } else {
                    shader2.set_3fv(ig_renderer.gl_context(), "color", glm::vec3(0.4, 0.4, 0.4));
                }

                let model_mat = glm::ext::translate(&ident_mat, glm::vec3(i as f32, 0.0, 0.0));
                let model_mat = glm::ext::rotate(&model_mat, glm::radians(90.0), glm::vec3(0.0, 1.0, 0.0));
                let model_mat = glm::ext::scale(&model_mat, glm::vec3(1000.0 * 1000.0, 0.0, 0.0));
                shader2.set_mat4fv(ig_renderer.gl_context(), "model", &model_mat);

                ig_renderer.gl_context().bind_vertex_array(Some(vao2));
                ig_renderer.gl_context().draw_arrays(glow::LINES, 0, 2);
                // gl::BindVertexArray(vao2);
                // gl::DrawArrays(gl::LINES, 0, 2);
            }

            glfw_platform.prepare_frame(imgui_ctx.io_mut(), &mut window).expect("Failed to prepare imgui frame");

            let ui = imgui_ctx.new_frame();
            ui.dockspace_over_main_viewport();

            // ui.show_demo_window(&mut true);
            ui.main_menu_bar(|| {
                ui.menu("File", || {
                    if ui.menu_item_config("New Bruh").shortcut("Ctrl + B").build() {
                        ui.text("lol");
                    }
                });
            });
            let mut is_bruh_win_open = true;
            ui.window("##main")
                .opened(&mut is_bruh_win_open)
                .movable(true)
                .build(|| {
                    ui.text("test");
                });

            ui.end_frame_early();

            let cursor = ui.mouse_cursor();
            if last_cursor != cursor {
                last_cursor = cursor;
                glfw_platform.prepare_render(&ui, &mut window);
            }

            imgui_ctx.update_platform_windows();

            let draw_data = imgui_ctx.render();
            ig_renderer.render(draw_data).expect("Failed to render imgui");

            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            glfw.poll_events();
            window.swap_buffers();
        }

        ig_renderer.gl_context().delete_vertex_array(vao);
        ig_renderer.gl_context().delete_vertex_array(vao2);
        ig_renderer.gl_context().delete_buffer(vbo);
        ig_renderer.gl_context().delete_buffer(vbo2);
        // gl::DeleteVertexArrays(1, &vao);
        // gl::DeleteVertexArrays(1, &vao2);
        // gl::DeleteBuffers(1, &vbo);
        // gl::DeleteBuffers(1, &vbo2);
        // gl::DeleteBuffers(1, &ebo);
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: &glfw::WindowEvent, shader: &rust_gl::shader::Shader, camera: &mut Camera, gl: &Rc<glow::Context>) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            println!("up");
            unsafe {
                shader.use_shader(&gl);
                // no worky
                shader.set_float(&gl, "mixValue", shader.get_float(&gl, "mixValue").get(0).unwrap() + 0.1);
            }
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            println!("down");
            unsafe {
                shader.use_shader(&gl);
                // no worky
                shader.set_float(&gl, "mixValue", shader.get_float(&gl, "mixValue").get(0).unwrap() - 0.1);
            }
        }
        glfw::WindowEvent::Scroll(_, yoff) => {
            camera.handle_mouse_scroll(*yoff as f32);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl.viewport(0, 0, *width, *height);
                // gl::Viewport(0, 0, *width, *height);
            }
        }
        _ => {}
    }
}

fn glow_context(window: &mut glfw::Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}
