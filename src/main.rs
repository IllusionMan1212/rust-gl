use glfw::{Action, Context, Key, Modifiers};
use glad_gl::gl;
use anyhow;

use rust_gl::{shader, model, ui::ui};

fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(1200, 800, "rust gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_all_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    let mut state = ui::State::default();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let (mut imgui, glfw_platform, renderer) = ui::init_imgui(&mut window);

    let mesh_shader = shader::Shader::new("shaders/vertex.glsl", "shaders/frag.glsl")?;
    let light_shader = shader::Shader::new("shaders/vertex.glsl", "shaders/light_f.glsl")?;
    let grid_shader = shader::Shader::new("shaders/grid_v.glsl", "shaders/grid_f.glsl")?;

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

    unsafe {
        // Object 1: Cube/container
        //
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);

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
        grid_shader.set_float("far", 200.0);

        mesh_shader.use_shader();

        // set light uniforms
        for i in 0..points_lights.len() {
            mesh_shader.set_3fv(&format!("pointLights[{}].position", i), points_lights[i]);

            mesh_shader.set_float(&format!("pointLights[{}].constant", i), 1.0);
            mesh_shader.set_float(&format!("pointLights[{}].linear", i), 0.09);
            mesh_shader.set_float(&format!("pointLights[{}].quadratic", i), 0.032);

            mesh_shader.set_3fv(&format!("pointLights[{}].ambient", i), glm::vec3(0.2, 0.2, 0.2));
            mesh_shader.set_3fv(&format!("pointLights[{}].diffuse", i), glm::vec3(0.5, 0.5, 0.5));
            mesh_shader.set_3fv(&format!("pointLights[{}].specular", i), glm::vec3(1.0, 1.0, 1.0));
        }
        mesh_shader.set_float("spotLight.cutOff", glm::cos(glm::radians(12.5)));
        mesh_shader.set_float("spotLight.outerCutOff", glm::cos(glm::radians(15.0)));
        mesh_shader.set_3fv("spotLight.ambient", glm::vec3(0.2, 0.2, 0.2));
        mesh_shader.set_3fv("spotLight.diffuse", glm::vec3(0.2, 0.2, 0.2));
        mesh_shader.set_3fv("spotLight.specular", glm::vec3(1.0, 1.0, 1.0));
        mesh_shader.set_float("spotLight.constant", 1.0);
        mesh_shader.set_float("spotLight.linear", 0.09);
        mesh_shader.set_float("spotLight.quadratic", 0.032);

        mesh_shader.set_3fv("dirLight.direction", glm::vec3(-0.2, -1.0, -0.3));
        mesh_shader.set_3fv("dirLight.ambient", glm::vec3(0.2, 0.2, 0.2));
        mesh_shader.set_3fv("dirLight.diffuse", glm::vec3(0.5, 0.5, 0.5));
        mesh_shader.set_3fv("dirLight.specular", glm::vec3(1.0, 1.0, 1.0));

        let lantern = model::Model::new("models/lantern/Lantern.gltf", &mut state)?;
        state.objects.push(lantern);

        let scene_fb = create_scene_framebuffer();

        // main loop
        while !window.should_close() {
            let current_frame = glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            state.camera.update_speed(delta_time);
            state.camera.handle_keyboard(&mut window, state.is_cursor_captured);

            imgui.io_mut().update_delta_time(std::time::Duration::from_secs_f32(delta_time));

            // camera matrices
            let view_mat = glm::ext::look_at(state.camera.position, state.camera.position + state.camera.front, state.camera.up);
            let projection_mat = glm::ext::perspective(glm::radians(state.camera.fov), state.viewport_size[0] / state.viewport_size[1], 0.01, 200.0);

            for (_, event) in glfw::flush_messages(&events) {
                if !state.is_cursor_captured {
                    glfw_platform.handle_event(imgui.io_mut(), &window, &event);
                }
                handle_window_event(&mut window, &event, &mut state);
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

            //
            // draw scene to framebuffer
            //
            let (scene_texture, rbo) = create_scene_texture_and_renderbuffer(&window, scene_fb);

            gl::BindFramebuffer(gl::FRAMEBUFFER, scene_fb);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            mesh_shader.use_shader();

            mesh_shader.set_mat4fv("view", &view_mat);
            mesh_shader.set_mat4fv("projection", &projection_mat);

            mesh_shader.set_3fv("spotLight.position", state.camera.position);
            mesh_shader.set_3fv("spotLight.direction", state.camera.front);
            mesh_shader.set_3fv("viewPos", state.camera.position);

            for obj in &state.objects {
                if state.wireframe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                } else {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
                obj.draw(&mesh_shader);
            }
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

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

            // draw grid
            if state.draw_grid {draw_grid(&grid_shader, &view_mat, &projection_mat);}

            //
            // draw ui
            //
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
            ui::draw_ui(&mut imgui, &renderer, &glfw_platform, &mut window, &mut state, delta_time, &mut last_cursor, scene_texture);

            glfw.poll_events();
            window.swap_buffers();

            gl::DeleteTextures(1, &scene_texture);
            gl::DeleteRenderbuffers(1, &rbo);
        }

        gl::DeleteVertexArrays(1, &light_vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteFramebuffers(1, &scene_fb);
    }

    Ok(())
}

fn draw_grid(shader: &rust_gl::shader::Shader, view_mat: &glm::Mat4, projection_mat: &glm::Mat4) {
    shader.use_shader();
    shader.set_mat4fv("view", &view_mat);
    shader.set_mat4fv("projection", &projection_mat);
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: &glfw::WindowEvent, state: &mut ui::State) {
    match event {
        glfw::WindowEvent::Key(Key::Q, _, Action::Press, Modifiers::Control) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => {
            state.camera.speed *= 5.0;
        }
        glfw::WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => {
            state.camera.speed /= 5.0;
        }
        glfw::WindowEvent::Key(Key::GraveAccent, _, Action::Press, _) => {
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
        glfw::WindowEvent::FramebufferSize(w, h) => {
            unsafe {
                gl::Viewport(0, 0, *w, *h);
            }
        }
        _ => {}
    }
}

fn create_scene_framebuffer() -> u32 {
    let mut fb: u32 = 0;

    unsafe {
        gl::GenFramebuffers(1, &mut fb);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
    }

    return fb;
}

fn create_scene_texture_and_renderbuffer(window: &glfw::Window, fbo: u32) -> (u32, u32) {
    let mut fb_texture: u32 = 0;
    let mut rbo: u32 = 0;

    let (w, h) = window.get_size();

    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        // texture
        gl::GenTextures(1, &mut fb_texture);
        gl::BindTexture(gl::TEXTURE_2D, fb_texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, w, h, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, fb_texture, 0);

        // renderbuffer for depth
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, w, h);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
    }

    return (fb_texture, rbo);
}
