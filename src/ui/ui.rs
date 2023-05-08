use glad_gl::gl;
use mint;

use crate::{camera::Camera, model, imgui_glfw_support, imgui_opengl_renderer, mesh, ui};

pub struct State {
    pub camera_coords_shown: bool,
    pub is_cursor_captured: bool,
    pub draw_grid: bool,
    pub first_frame_drawn: bool,
    pub camera: Camera,
    pub objects: Vec<model::Model>,
    pub viewport_size: [f32; 2],
}

impl Default for State {
    fn default() -> Self {
        Self {
            camera_coords_shown: false,
            first_frame_drawn: false,
            is_cursor_captured: false,
            draw_grid: true,
            camera: Camera::new(),
            objects: vec![],
            viewport_size: [0.0, 0.0],
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

fn create_initial_docking(ui: &imgui::Ui, state: &mut State) {
    let flags =
        // No borders etc for top-level window
        imgui::WindowFlags::NO_DECORATION | imgui::WindowFlags::NO_MOVE
        // Show menu bar
        | imgui::WindowFlags::MENU_BAR
        // Don't raise window on focus (as it'll clobber floating windows)
        | imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS | imgui::WindowFlags::NO_NAV_FOCUS
        // Don't want the dock area's parent to be dockable!
        | imgui::WindowFlags::NO_DOCKING
        ;

    let padding = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
    let rounding = ui.push_style_var(imgui::StyleVar::WindowRounding(0.0));

    ui.window("Main Window")
        .flags(flags)
        .position([0.0, 0.0], imgui::Condition::Always)
        .size(ui.io().display_size, imgui::Condition::Always)
        .build(|| {
            // Create top-level docking area, needs to be made early (before docked windows)
            let ui_d = ui::docking::UiDocking {};
            let space = ui_d.dockspace("MainDockArea");

            // Set up splits, docking windows. This can be done conditionally,
            // or calling it every time is also mostly fine
            if !state.first_frame_drawn {
                space.split(
                    imgui::Direction::Right,
                    0.3,
                    |right| {
                        right.dock_window("Objects");
                    },
                    |left| {
                        left.dock_window("Scene");
                    },
                );
            }
        });

    padding.pop();
    rounding.pop();
}

fn draw_viewport(ui: &imgui::Ui, state: &mut State, texture: u32) {
    ui.window("Scene")
        .size(ui.content_region_avail(), imgui::Condition::FirstUseEver)
        .no_decoration()
        .build(|| {
            let size = ui.content_region_avail();
            state.viewport_size = size;

            imgui::Image::new(imgui::TextureId::new(texture.try_into().unwrap()), size)
                // flip the image vertically
                .uv0([0.0, 1.0])
                .uv1([1.0, 0.0])
                .build(ui);
        });
}

pub fn draw_ui(
    imgui: &mut imgui::Context,
    renderer: &imgui_opengl_renderer::Renderer,
    glfw_platform: &imgui_glfw_support::GlfwPlatform,
    window: &mut glfw::Window,
    state: &mut State,
    delta_time: f32,
    last_cursor: &mut Option<imgui::MouseCursor>,
    scene_fb_texture: u32,
) {
    glfw_platform.prepare_frame(imgui.io_mut(), window).expect("Failed to prepare imgui frame");

    let ui = imgui.new_frame();
    create_initial_docking(ui, state);

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
    draw_viewport(ui, state, scene_fb_texture);

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
    state.first_frame_drawn = true;
}
