use glfw::{Action, Context, Key};
use image;
use std::rc::Rc;
use glow::HasContext;

use rust_gl::{shader::*, camera::*, imgui_glfw_support};

struct State {
    is_bruh_win_open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_bruh_win_open: true,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(800, 600, "rust-gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_all_polling(true);
    window.make_current();

    let mut state = State::default();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let (mut imgui, glfw_platform, mut renderer) = init_imgui(&mut window);

    let tex1 = image::io::Reader::open("textures/container.jpg")?.decode()?;
    let mut tex2 = image::io::Reader::open("textures/awesomeface.png")?.decode()?;
    tex2 = tex2.flipv();

    unsafe {
        renderer.gl_context().enable(glow::DEPTH_TEST);
        renderer.gl_context().viewport(0, 0, 800, 600);

        let shader = Shader::new(renderer.gl_context(), "shaders/vertex.glsl", "shaders/frag.glsl")?;
        let shader2 = Shader::new(renderer.gl_context(), "shaders/grid_v.glsl", "shaders/grid_f.glsl")?;

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
        let vao = renderer.gl_context().create_vertex_array()?;
        let vbo = renderer.gl_context().create_buffer()?;

        renderer.gl_context().bind_vertex_array(Some(vao));

        renderer.gl_context().bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        let vertices_u8: &[u8] = core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<f32>(),
        );

        renderer.gl_context().buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

        // position attribute, maps to 'aPos' in vertex shader with location 0
        renderer.gl_context().vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 5 * std::mem::size_of::<f32>() as i32, 0);
        renderer.gl_context().enable_vertex_attrib_array(0);

        // texture coords attribute, maps to 'aTexCoords' in vertex shader with location 1
        renderer.gl_context().vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as i32);
        renderer.gl_context().enable_vertex_attrib_array(1);

        // texture 1
        let texture1 = renderer.gl_context().create_texture()?;
        renderer.gl_context().bind_texture(glow::TEXTURE_2D, Some(texture1));

        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

        renderer.gl_context().tex_image_2d(glow::TEXTURE_2D, 0, glow::RGB as i32, tex1.width() as i32, tex1.height() as i32, 0, glow::RGB, glow::UNSIGNED_BYTE, Some(tex1.as_bytes()));
        renderer.gl_context().generate_mipmap(glow::TEXTURE_2D);

        // // texture 2
        let texture2 = renderer.gl_context().create_texture()?;
        renderer.gl_context().bind_texture(glow::TEXTURE_2D, Some(texture2));

        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        renderer.gl_context().tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

        renderer.gl_context().tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, tex2.width() as i32, tex2.height() as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(tex2.as_bytes()));
        renderer.gl_context().generate_mipmap(glow::TEXTURE_2D);

        shader.use_shader(renderer.gl_context());
        shader.set_int(renderer.gl_context(), "texture1", 0);
        shader.set_int(renderer.gl_context(), "texture2", 1);

        // End of Object 1

        // Object 2: Grid
  
        let vertices2: [f32; 6] = [
            -1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
        ];

        let vao2 = renderer.gl_context().create_vertex_array()?;
        let vbo2 = renderer.gl_context().create_buffer()?;

        renderer.gl_context().bind_vertex_array(Some(vao2));

        renderer.gl_context().bind_buffer(glow::ARRAY_BUFFER, Some(vbo2));
        let vertices2_u8: &[u8] = core::slice::from_raw_parts(
            vertices2.as_ptr() as *const u8,
            vertices2.len() * core::mem::size_of::<f32>(),
        );

        renderer.gl_context().buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertices2_u8, glow::STATIC_DRAW);

        renderer.gl_context().vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 3 * std::mem::size_of::<f32>() as i32, 0);
        renderer.gl_context().enable_vertex_attrib_array(0);

        // End of Object 2

        let mut camera = Camera::new(camera_speed, camera_sensitivity);

        while !window.should_close() {
            let current_frame = glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            camera.update_speed(delta_time);
            camera.handle_keyboard(&mut window);

            imgui.io_mut().update_delta_time(std::time::Duration::from_secs_f32(delta_time));

            renderer.gl_context().clear_color(0.3, 0.3, 0.3, 1.0);
            renderer.gl_context().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // camera matrices
            let view_mat = glm::ext::look_at(camera.position, camera.position + camera.front, camera.up);
            let (win_width, win_height) = window.get_size();
            let projection_mat = glm::ext::perspective(glm::radians(camera.fov), win_width as f32 / win_height as f32, 0.1, 100.0);

            for (_, event) in glfw::flush_messages(&events) {
                glfw_platform.handle_event(imgui.io_mut(), &window, &event);
                handle_window_event(&mut window, &event, &shader, &mut camera, renderer.gl_context());
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

            // draw scene
            shader.use_shader(renderer.gl_context());

            renderer.gl_context().active_texture(glow::TEXTURE0);
            renderer.gl_context().bind_texture(glow::TEXTURE_2D, Some(texture1));
            renderer.gl_context().active_texture(glow::TEXTURE1);
            renderer.gl_context().bind_texture(glow::TEXTURE_2D, Some(texture2));

            shader.set_mat4fv(renderer.gl_context(), "view", &view_mat);
            shader.set_mat4fv(renderer.gl_context(), "projection", &projection_mat);

            for i in 0..cubes.len() {
                let model_mat = glm::ext::translate(&ident_mat, cubes[i]);
                let angle = 20.0 * i as f32;
                let model_mat = glm::ext::rotate(&model_mat, glfw.get_time() as f32 * glm::radians(angle), glm::vec3(1.0, 0.3, 0.5));
                shader.set_mat4fv(renderer.gl_context(), "model", &model_mat);

                renderer.gl_context().bind_vertex_array(Some(vao));
                renderer.gl_context().draw_arrays(glow::TRIANGLES, 0, 36);
            }

            shader2.use_shader(renderer.gl_context());
            shader2.set_mat4fv(renderer.gl_context(), "view", &view_mat);
            shader2.set_mat4fv(renderer.gl_context(), "projection", &projection_mat);

            for i in -100..100 {
                if i == 0 {
                    shader2.set_3fv(renderer.gl_context(), "color", glm::vec3(1.0, 0.2, 0.2));
                } else {
                    shader2.set_3fv(renderer.gl_context(), "color", glm::vec3(0.4, 0.4, 0.4));
                }

                let model_mat = glm::ext::translate(&ident_mat, glm::vec3(0.0, 0.0, i as f32));
                let model_mat = glm::ext::scale(&model_mat, glm::vec3(1000.0 * 1000.0, 0.0, 0.0));

                shader2.set_mat4fv(renderer.gl_context(), "model", &model_mat);

                renderer.gl_context().bind_vertex_array(Some(vao2));
                renderer.gl_context().draw_arrays(glow::LINES, 0, 2);

                if i == 0 {
                    shader2.set_3fv(renderer.gl_context(), "color", glm::vec3(0.0, 1.0, 0.0));
                } else {
                    shader2.set_3fv(renderer.gl_context(), "color", glm::vec3(0.4, 0.4, 0.4));
                }

                let model_mat = glm::ext::translate(&ident_mat, glm::vec3(i as f32, 0.0, 0.0));
                let model_mat = glm::ext::rotate(&model_mat, glm::radians(90.0), glm::vec3(0.0, 1.0, 0.0));
                let model_mat = glm::ext::scale(&model_mat, glm::vec3(1000.0 * 1000.0, 0.0, 0.0));
                shader2.set_mat4fv(renderer.gl_context(), "model", &model_mat);

                renderer.gl_context().bind_vertex_array(Some(vao2));
                renderer.gl_context().draw_arrays(glow::LINES, 0, 2);
            }

            // draw imgui ui

            glfw_platform.prepare_frame(imgui.io_mut(), &mut window).expect("Failed to prepare imgui frame");

            let ui = imgui.new_frame();
            ui.dockspace_over_main_viewport();

            draw_main_menu_bar(ui, &mut state);
            if state.is_bruh_win_open {
                ui.window("##main")
                    .opened(&mut state.is_bruh_win_open)
                    .build(|| {
                        ui.text("test");
                    });
            }

            ui.end_frame_early();

            let cursor = ui.mouse_cursor();
            if last_cursor != cursor {
                last_cursor = cursor;
                glfw_platform.prepare_render(&ui, &mut window);
            }

            imgui.update_platform_windows();

            let draw_data = imgui.render();
            renderer.render(draw_data).expect("Failed to render imgui");

            glfw.poll_events();
            window.swap_buffers();
        }

        renderer.gl_context().delete_vertex_array(vao);
        renderer.gl_context().delete_vertex_array(vao2);
        renderer.gl_context().delete_buffer(vbo);
        renderer.gl_context().delete_buffer(vbo2);
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: &glfw::WindowEvent, shader: &rust_gl::shader::Shader, camera: &mut Camera, gl: &Rc<glow::Context>) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            unsafe {
                shader.use_shader(&gl);
                shader.set_float(&gl, "mixValue", shader.get_float(&gl, "mixValue").get(0).unwrap() + 0.1);
            }
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            unsafe {
                shader.use_shader(&gl);
                shader.set_float(&gl, "mixValue", shader.get_float(&gl, "mixValue").get(0).unwrap() - 0.1);
            }
        }
        glfw::WindowEvent::Scroll(_, yoff) => {
            camera.handle_mouse_scroll(*yoff as f32);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl.viewport(0, 0, *width, *height);
            }
        }
        _ => {}
    }
}

fn draw_main_menu_bar(ui: &imgui::Ui, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if ui.menu_item_config("Import model(s)").shortcut("Ctrl+O").build() {
                // TODO: open file dialog
            }
            if ui.menu_item_config("Quit").shortcut("Ctrl+Q").build() {
                // TODO: quit the application
            }
        });
        ui.menu("View", || {
            // TODO: add menu items
            if ui.menu_item("Reset Camera") {
                // TODO: reset camera
            }
            if ui.menu_item("Show Bruh Window") {
                state.is_bruh_win_open = !state.is_bruh_win_open;
            }
        })
    });
}

fn init_imgui(window: &mut glfw::Window) -> (imgui::Context, imgui_glfw_support::GlfwPlatform, imgui_glow_renderer::AutoRenderer) {
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    imgui.io_mut().config_flags.insert(imgui::ConfigFlags::DOCKING_ENABLE);
    imgui.io_mut().config_flags.set(imgui::ConfigFlags::NAV_ENABLE_KEYBOARD, true);

    let mut glfw_platform = imgui_glfw_support::GlfwPlatform::init(&mut imgui);
    glfw_platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_glfw_support::HiDpiMode::Rounded
    );

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui.io_mut().font_global_scale = (1.0 / glfw_platform.hidpi_factor()) as f32;

    let gl = glow_context(window);

    let ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui).expect("Failed to initialize imgui renderer");
    glfw_platform.set_clipboard_backend(&mut imgui, &window);

    (imgui, glfw_platform, ig_renderer)
}

fn glow_context(window: &mut glfw::Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}
