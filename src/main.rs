use glfw::{Action, Context, Key, Modifiers};
use glad_gl::gl;
use mint;
use anyhow;

use rust_gl::{shader::*, camera::*, imgui_glfw_support, imgui_opengl_renderer, utils};

struct State {
    camera_coords_shown: bool,
    is_cursor_captured: bool,
    draw_grid: bool,
    camera: Camera,
}

impl Default for State {
    fn default() -> Self {
        Self {
            camera_coords_shown: false,
            is_cursor_captured: true,
            draw_grid: true,
            camera: Camera::new(),
        }
    }
}

fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(1200, 800, "rust gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_all_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    let mut state = State::default();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let (mut imgui, glfw_platform, renderer) = init_imgui(&mut window);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Viewport(0, 0, 1200, 800);

        let shader = Shader::new("shaders/vertex.glsl", "shaders/frag.glsl")?;
        let light_shader = Shader::new("shaders/vertex.glsl", "shaders/light_f.glsl")?;
        let grid_shader = Shader::new("shaders/grid_v.glsl", "shaders/grid_f.glsl")?;

        let vertices: [f32; 288] = [
            // positions // normals // texture coords
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
            0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0,
            0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0,
            0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0,
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0,
            0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
            0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
            -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0,
            -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0,
            -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
            -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
            -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0,
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
            0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
            0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
            0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
            0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
            0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0,
            0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0,
            0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0,
            0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
            0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
            -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0
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

        let points_lights: [glm::Vec3; 4] = [
            glm::vec3(0.7, 0.2, 2.0),
            glm::vec3(2.3, -3.3, -4.0),
            glm::vec3(-4.0, 2.0, -12.0),
            glm::vec3(0.0, 0.0, -3.0),
        ];

        let ident_mat = glm::mat4(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.);

        let mut delta_time: f32 = 0.0;
        let mut last_frame: f32 = 0.0;
        let mut last_cursor = None;

        let (w, h) = window.get_size();
        let mut last_x: f32 = w as f32 / 2.0;
        let mut last_y: f32 = h as f32 / 2.0;
        let mut first_mouse: bool = true;

        // Object 1: Cube/container
        //
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * core::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        // position attribute, maps to 'aPos' in vertex shader with location 0
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // normal attribute, maps to 'aNormal' in vertex shader with location 1
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(1);

        // texture attribute, maps to 'aTexCoords' in vertex shader with location 2
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, (6 * std::mem::size_of::<f32>()) as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(2);

        let diffuse_map = utils::load_texture("textures/container2.png")?;
        let specular_map = utils::load_texture("textures/container2_specular.png")?;

        // Object 2: Light source
        let mut light_vao: u32 = 0;
        gl::GenVertexArrays(1, &mut light_vao);

        gl::BindVertexArray(light_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // position attribute, maps to 'aPos' in vertex shader with location 0
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        grid_shader.use_shader();
        grid_shader.set_float("near", 0.01);
        grid_shader.set_float("far", 100.0);

        shader.use_shader();
        shader.set_int("material.diffuse", 0);
        shader.set_int("material.specular", 1);
        shader.set_float("material.shininess", 32.0);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, specular_map);


        // set light uniforms
        for i in 0..points_lights.len() {
            shader.set_3fv(&format!("pointLights[{}].position", i), points_lights[i]);

            shader.set_float(&format!("pointLights[{}].constant", i), 1.0);
            shader.set_float(&format!("pointLights[{}].linear", i), 0.09);
            shader.set_float(&format!("pointLights[{}].quadratic", i), 0.032);

            shader.set_3fv(&format!("pointLights[{}].ambient", i), glm::vec3(0.2, 0.2, 0.2));
            shader.set_3fv(&format!("pointLights[{}].diffuse", i), glm::vec3(0.5, 0.5, 0.5));
            shader.set_3fv(&format!("pointLights[{}].specular", i), glm::vec3(1.0, 1.0, 1.0));
        }
        shader.set_float("spotLight.cutOff", glm::cos(glm::radians(12.5)));
        shader.set_float("spotLight.outerCutOff", glm::cos(glm::radians(15.0)));
        shader.set_3fv("spotLight.ambient", glm::vec3(0.2, 0.2, 0.2));
        shader.set_3fv("spotLight.diffuse", glm::vec3(1.0, 1.0, 1.0));
        shader.set_3fv("spotLight.specular", glm::vec3(1.0, 1.0, 1.0));
        shader.set_float("spotLight.constant", 1.0);
        shader.set_float("spotLight.linear", 0.09);
        shader.set_float("spotLight.quadratic", 0.032);

        shader.set_3fv("dirLight.direction", glm::vec3(-0.2, -1.0, -0.3));
        shader.set_3fv("dirLight.ambient", glm::vec3(0.2, 0.2, 0.2));
        shader.set_3fv("dirLight.diffuse", glm::vec3(0.5, 0.5, 0.5));
        shader.set_3fv("dirLight.specular", glm::vec3(1.0, 1.0, 1.0));

        while !window.should_close() {
            let current_frame = glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            state.camera.update_speed(delta_time);
            state.camera.handle_keyboard(&mut window, state.is_cursor_captured);

            imgui.io_mut().update_delta_time(std::time::Duration::from_secs_f32(delta_time));

            // camera matrices
            let view_mat = glm::ext::look_at(state.camera.position, state.camera.position + state.camera.front, state.camera.up);
            let (win_width, win_height) = window.get_size();
            let projection_mat = glm::ext::perspective(glm::radians(state.camera.fov), win_width as f32 / win_height as f32, 0.01, 100.0);

            for (_, event) in glfw::flush_messages(&events) {
                glfw_platform.handle_event(imgui.io_mut(), &window, &event);
                handle_window_event(&mut window, &event, &shader, &mut state);
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

                        state.camera.handle_mouse_input(xoffset, yoffset, state.is_cursor_captured);
                    }
                    _ => {}
                }
            }

            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // draw scene
            shader.use_shader();

            shader.set_mat4fv("view", &view_mat);
            shader.set_mat4fv("projection", &projection_mat);

            shader.set_3fv("spotLight.position", state.camera.position);
            shader.set_3fv("spotLight.direction", state.camera.front);
            shader.set_3fv("viewPos", state.camera.position);

            for i in 0..cubes.len() {
                let model_mat = glm::ext::translate(&ident_mat, cubes[i]);
                let angle = 20.0 * i as f32;
                let model_mat = glm::ext::rotate(&model_mat, glfw.get_time() as f32 * glm::radians(angle), glm::vec3(1.0, 0.3, 0.5));
                shader.set_mat4fv("model", &model_mat);

                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            light_shader.use_shader();
            light_shader.set_mat4fv("view", &view_mat);
            light_shader.set_mat4fv("projection", &projection_mat);

            for i in 0..points_lights.len() {
                let light_model = glm::ext::translate(&ident_mat, points_lights[i]);
                let light_model = glm::ext::scale(&light_model, glm::vec3(0.2, 0.2, 0.2));
                light_shader.set_mat4fv("model", &light_model);

                gl::BindVertexArray(light_vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }


            if state.draw_grid {draw_grid(&grid_shader, &view_mat, &projection_mat);}

            draw_ui(&mut imgui, &renderer, &glfw_platform, &mut window, &mut state, delta_time, &mut last_cursor);

            glfw.poll_events();
            window.swap_buffers();
        }

        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteVertexArrays(1, &light_vao);
        gl::DeleteBuffers(1, &vbo);
    }

    Ok(())
}

fn draw_ui(
    imgui: &mut imgui::Context,
    renderer: &imgui_opengl_renderer::Renderer,
    glfw_platform: &imgui_glfw_support::GlfwPlatform,
    window: &mut glfw::Window,
    state: &mut State,
    delta_time: f32,
    last_cursor: &mut Option<imgui::MouseCursor>
) {
    glfw_platform.prepare_frame(imgui.io_mut(), window).expect("Failed to prepare imgui frame");

    let ui = imgui.new_frame();
    ui.dockspace_over_main_viewport();

    draw_main_menu_bar(ui, state, window, delta_time);
    if state.camera_coords_shown {
        ui.window("Camera Coordinates")
            .size([300.0, 100.0], imgui::Condition::FirstUseEver)
            .opened(&mut state.camera_coords_shown)
            .build(|| {
                ui.text(format!("X: {:.4}\nY: {:.4}\nZ: {:.4}", state.camera.position.x, state.camera.position.y, state.camera.position.z));
            });
    }

    ui.end_frame_early();

    if !state.is_cursor_captured {
        let cursor = ui.mouse_cursor();
        if *last_cursor != cursor {
            *last_cursor = cursor;
            glfw_platform.prepare_render(&ui, window);
        }
    }

    imgui.update_platform_windows();

    renderer.render(imgui);
}

fn draw_grid(shader: &rust_gl::shader::Shader, view_mat: &glm::Mat4, projection_mat: &glm::Mat4) {
    shader.use_shader();
    shader.set_mat4fv("view", &view_mat);
    shader.set_mat4fv("projection", &projection_mat);
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: &glfw::WindowEvent, shader: &rust_gl::shader::Shader, state: &mut State) {
    match event {
        glfw::WindowEvent::Key(Key::Q, _, Action::Press, Modifiers::Control) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            shader.use_shader();
            shader.set_float("mixValue", shader.get_float("mixValue") + 0.1);
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            shader.use_shader();
            shader.set_float("mixValue", shader.get_float("mixValue") - 0.1);
        }
        glfw::WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => {
            state.camera.speed *= 5.0;
        }
        glfw::WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => {
            state.camera.speed /= 5.0;
        }
        glfw::WindowEvent::Key(Key::Tab, _, Action::Press, _) => {
            state.is_cursor_captured = !state.is_cursor_captured;
            if state.is_cursor_captured {
                window.set_cursor_mode(glfw::CursorMode::Disabled);
            } else {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            }
        }
        glfw::WindowEvent::Scroll(_, yoff) => {
            state.camera.handle_mouse_scroll(*yoff as f32, state.is_cursor_captured);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, *width, *height);
            }
        }
        _ => {}
    }
}

fn draw_main_menu_bar(ui: &imgui::Ui, state: &mut State, window: &mut glfw::Window, delta_time: f32) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if ui.menu_item_config("Import Model(s)").shortcut("Ctrl+O").build() {
                // TODO: open file dialog
            }
            if ui.menu_item_config("Quit").shortcut("Ctrl+Q").build() {
                // TODO: cleanup stuff
                window.set_should_close(true);
            }
        });
        ui.menu("View", || {
            // TODO: add menu items
            if ui.menu_item("Reset Camera") {
                state.camera.reset();
            }
            if ui.menu_item_config("Show Camera Coords").selected(state.camera_coords_shown).build() {
                state.camera_coords_shown = !state.camera_coords_shown;
            }
            if ui.menu_item_config("Toggle grid").selected(state.draw_grid).build() {
                state.draw_grid = !state.draw_grid;
            }
        });
        let mut avail_size = mint::Vector2 { x: 0.0, y: 0.0 };
        avail_size.x = *ui.content_region_avail().get(0).unwrap() - ui.calc_text_size(format!("FPS: {:.1}", 1.0 / delta_time))[0];
        ui.dummy(avail_size);
        ui.text(format!("FPS: {:.1}", 1.0 / delta_time));
    });
}

fn init_imgui(window: &mut glfw::Window) -> (imgui::Context, imgui_glfw_support::GlfwPlatform, imgui_opengl_renderer::Renderer) {
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

    gl::load(|e| window.get_proc_address(e) as *const std::os::raw::c_void);

    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui);
    glfw_platform.set_clipboard_backend(&mut imgui, &window);

    (imgui, glfw_platform, renderer)
}

