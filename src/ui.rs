use glad_gl::gl;
use mint;

use crate::{camera::Camera, model, imgui_glfw_support, imgui_opengl_renderer, mesh};

pub struct State {
    pub camera_coords_shown: bool,
    pub is_cursor_captured: bool,
    pub draw_grid: bool,
    pub camera: Camera,
    pub objects: Vec<model::Model>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            camera_coords_shown: false,
            is_cursor_captured: true,
            draw_grid: true,
            camera: Camera::new(),
            objects: vec![],
        }
    }
}

pub fn init_imgui(window: &mut glfw::Window) -> (imgui::Context, imgui_glfw_support::GlfwPlatform, imgui_opengl_renderer::Renderer) {
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

pub fn draw_main_menu_bar(ui: &imgui::Ui, state: &mut State, window: &mut glfw::Window, delta_time: f32) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if ui.menu_item_config("Import Model(s)").shortcut("Ctrl+O").build() {
                let models = match rfd::FileDialog::new()
                    .set_title("Import Model(s)")
                    .set_directory("./")
                    .add_filter("All supported files", &["obj", "fbx", "gltf", "glb"])
                    .add_filter("Wavefront OBJ (.obj)", &["obj"])
                    .add_filter("FBX (.fbx)", &["fbx"])
                    .add_filter("glTF (.gltf, .glb)", &["gltf", "glb"])
                    .pick_files() {
                        Some(m) => m,
                        None => return,
                    };
                for model_path in &models {
                    let model = model::Model::new(model_path.to_str().unwrap());
                    match model {
                        Ok(m) => state.objects.push(m),
                        Err(e) => {
                            println!("Error loading model: {}", e);
                            // TODO: show error in imgui overlay
                        },
                    }
                }
            }
            if ui.menu_item_config("Quit").shortcut("Ctrl+Q").build() {
                // TODO: cleanup stuff
                window.set_should_close(true);
            }
        });
        ui.menu("View", || {
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

// TODO: finish this
// fn draw_error_overlay(ui: &imgui::Ui) {
//     const DISTANCE: f32 = 10.0;
//     let window_pos = [DISTANCE, DISTANCE];
//     let style = ui.push_style_color(imgui::StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.5]);
//     ui.window("Error Overlay")
//         .opened(opened)
//         .position(window_pos, imgui::Condition::Always)
//         .title_bar(false)
//         .resizable(false)
//         .always_auto_resize(true)
//         .movable(false)
//         .save_settings(false)
//         .build(|| {
//             ui.text(
//                 "Simple overlay\nin the corner of the screen.",
//             );
//         });
//     style.pop();
// }

fn draw_transformations(ui: &imgui::Ui, mesh: &mut mesh::Mesh) {
    imgui::Drag::new("###XPos")
        .range(f32::NEG_INFINITY, f32::INFINITY)
        .speed(0.1)
        .display_format("X: %.3f")
        .build(ui, &mut mesh.position.x);
    imgui::Drag::new("###YPos")
        .range(f32::NEG_INFINITY, f32::INFINITY)
        .speed(0.1)
        .display_format("Y: %.3f")
        .build(ui, &mut mesh.position.y);
    imgui::Drag::new("###ZPos")
        .range(f32::NEG_INFINITY, f32::INFINITY)
        .speed(0.1)
        .display_format("Z: %.3f")
        .build(ui, &mut mesh.position.z);
    imgui::Drag::new("Scale")
        .range(f32::NEG_INFINITY, f32::INFINITY)
        .speed(0.1)
        .display_format("%.7f")
        .build_array(ui, mesh.scale.as_array_mut());
    imgui::Drag::new("Rotation")
        .range(f32::NEG_INFINITY, f32::INFINITY)
        .speed(1.0)
        .display_format("%.2f")
        .build_array(ui, mesh.rotation.as_array_mut());
}

fn draw_mesh_hierarchy(ui: &imgui::Ui, mesh: &mut mesh::Mesh, i: usize) {
    ui.tree_node_config(format!("{}###{}", mesh.name.as_str(), i)).build(|| {
        ui.text(format!("Vertices: {}", mesh.vertices.len()));
        ui.text(format!("Textures: {}", mesh.textures.len()));
        ui.tree_node_config(mesh.material.name.as_str()).build(|| {
            ui.text(format!("{}", mesh.material));
        });
        ui.tree_node_config("Transformations").build(|| {
            draw_transformations(ui, mesh);
        })
    });
}

fn draw_object_hierarchy(ui: &imgui::Ui, state: &mut State, idx: usize) -> bool {
    let object = &mut state.objects[idx];
    if let Some(_t) = ui.begin_table_with_sizing("Objects Table", 2, imgui::TableFlags::SIZING_STRETCH_PROP, [0.0, 0.0], 0.0) {
        ui.table_next_row();
        ui.table_next_column();
        ui.tree_node_config(format!("{}###{}", object.name.as_str(), idx))
            .build(|| {
                for (j, mesh) in &mut object.meshes.iter_mut().enumerate() {
                    draw_mesh_hierarchy(ui, mesh, j);
                }
            });

        ui.table_next_column();
        if ui.small_button(format!("X###{}-{}", object.name.as_str(), idx)) {
            println!("Removing object {}", object.name);
            return true;
        }
    }

    return false;
}

fn draw_objects_window(ui: &imgui::Ui, state: &mut State) {
    ui.window("Objects")
        .size([500.0, 200.0], imgui::Condition::FirstUseEver)
        .build(|| {
            let mut i = 0;

            while i < state.objects.len() {
                if draw_object_hierarchy(ui, state, i) {
                    state.objects.remove(i);
                    continue;
                }

                i = i + 1;
            }
        });
}

pub fn draw_ui(
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

    draw_objects_window(ui, state);

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
