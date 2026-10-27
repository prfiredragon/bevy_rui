use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::ui_widgets::Activate;
use bevy_rui::*;
use bevy_rui::widgets::file_dialog::{spawn_file_dialog, FileDialogMode, RuiFileSelected, RuiFileCanceled, FileFilter};
use std::env;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum DialogTrigger {
    Open,
    Save,
}

#[derive(Component)]
struct MainUiRoot;

fn main() {
    App::new()
        .insert_resource(MeshPickingSettings {
            require_markers: false,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .insert_resource(UiPickingSettings {
            require_markers: false,
        })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync, 
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(MeshPickingPlugin)
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (auto_rotate_cube, listen_to_file_dialog_messages))
        .run();
}

#[derive(Component)]
struct TestCube;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut theme: ResMut<bevy_rui::theme::RuiTheme>,
) {
    theme.color_panel = Color::srgb_u8(37, 37, 38);
    theme.border_panel = UiRect::all(Val::Px(0.0));
    theme.color_text = Color::srgb_u8(204, 204, 204);
    theme.color_window_header = Color::srgb_u8(51, 51, 51);
    theme.color_window_header_text = Color::srgb_u8(204, 204, 204);
    theme.border_window = UiRect::all(Val::Px(1.0));
    theme.border_color_window = Color::BLACK;
    theme.border_radius_window = BorderRadius::all(Val::Px(4.0));
    theme.color_textbox_bg = Color::srgb_u8(60, 60, 60);
    theme.border_textbox = UiRect::all(Val::Px(1.0));
    theme.border_color_textbox = Color::srgb_u8(69, 69, 69);
    theme.border_radius_textbox = BorderRadius::all(Val::Px(2.0));
    theme.color_button_normal = theme.color_panel;
    theme.border_button = UiRect::all(Val::Px(1.0));
    theme.border_color_button = Color::srgb_u8(69, 69, 69);
    theme.border_radius_button = BorderRadius::all(Val::Px(2.0));
    theme.color_button_hover = Color::srgb_u8(17, 119, 187);
    theme.color_button_pressed = Color::srgb_u8(9, 71, 113);
    theme.color_button_text = Color::WHITE;
    theme.color_list_item_normal = Color::NONE;
    theme.color_list_item_hover = Color::srgb_u8(42, 45, 46); 
    theme.color_list_item_pressed = Color::srgb_u8(9, 71, 113);
    theme.border_list_item = UiRect::all(Val::Px(1.0));
    theme.border_color_list_item = Color::NONE;
    theme.border_radius_list_item = BorderRadius::all(Val::Px(2.0)); 

    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));

    commands.spawn((
        Camera2d,
        Camera { order: 1, clear_color: ClearColorConfig::None, ..default() },
        UiPickingCamera,
    ));

    let viewport_camera = commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        MeshPickingCamera,
        RuiViewportCamera,
    )).id();

    commands.spawn((
        DirectionalLight { shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(-4.0, 8.0, -4.0),
    ));

    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(StandardMaterial { base_color: Color::srgb(0.87, 0.02, 0.02), ..default() });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, -5.0),
        TestCube,
        Pickable::default(),
    )).observe(|_: On<PointerClick>| {
        println!("¡Cubo 3D clickeado directamente de forma nativa!");
    });
    let root_entity = commands.rui_root(|_| {}, |ui| {
        let mut root_ui = ui.reborrow();
        root_ui.vbox(|_| {}, |vbox| {
            let mut vb = vbox.reborrow();
            let emoji_font = asset_server.load("fonts/openmojicolor.ttf");

            // --- MENÚ SUPERIOR ---
            let mut bar_bind = vb.reborrow();
            bar_bind.menu_bar(|s| { s.flex_shrink = 0.0; }, |menu| {
                let mut m = menu.reborrow();
                
                let mut file_bind = m.reborrow();
                file_bind.submenu("File", None, 0, |_| {}, |file_menu| {
                    let mut fm = file_menu.reborrow();
                    fm.reborrow().menu_item("Open", Some(RuiIcon::Emoji("📁".to_string(), Some(emoji_font.clone()))), |_| {}).insert(DialogTrigger::Open);
                    fm.reborrow().menu_item("Save", Some(RuiIcon::Emoji("💾".to_string(), Some(emoji_font.clone()))), |_| {}).insert(DialogTrigger::Save);
                });
                
                let mut prefab_bind = m.reborrow();
                prefab_bind.submenu("Prefab", None, 0, |_| {}, |prefab_menu| {
                    let mut pm = prefab_menu.reborrow();
                    
                    let mut native_bind = pm.reborrow();
                    native_bind.submenu("Native", None, 1, |_| {}, |native_menu| {
                        native_menu.reborrow().menu_item("Capsule3d", None, |_| {});
                    });
                    
                    let mut particle_bind = pm.reborrow();
                    particle_bind.submenu("Particle", None, 1, |_| {}, |particle_menu| {
                        particle_menu.reborrow().menu_item("Particle3d", Some(RuiIcon::Emoji("✨".to_string(), Some(emoji_font.clone()))), |_| {});
                    });
                    
                    let mut env_bind = pm.reborrow();
                    env_bind.submenu("Environment", None, 1, |_| {}, |env_menu| {
                        let mut em = env_menu.reborrow();
                        let mut light_bind = em.reborrow();
                        light_bind.menu_item("Directional Light", Some(RuiIcon::Emoji("☀️".to_string(), Some(emoji_font.clone()))), |_| {});
                        let mut sky_bind = em.reborrow();
                        sky_bind.menu_item("Skybox", Some(RuiIcon::Emoji("🌌".to_string(), Some(emoji_font.clone()))), |_| {});
                    });
                });
                
                let mut help_bind = m.reborrow();
                help_bind.menu_item("Help", None, |_| {});
            });

            // --- ÁREA DE DOCKING (CORRECCIÓN E0599: Invocamos de forma directa sobre tabs sin .reborrow()) ---
            let mut dock_bind = vb.reborrow();
            dock_bind.dock_split_horizontal(Val::Percent(20.0), 100.0, |_| {}, |left_area| {
                let mut la = left_area.reborrow();
                la.dock_panel(0, |_| {}, |tabs| {
                    tabs.tab("Hierarchy", false, |_, _| {}, |content| { content.reborrow().label("Contenido de Hierarchy", |_,_|{}); });
                    tabs.tab("Entities", true, |_, _| {}, |content| { content.reborrow().label("Contenido de Entities", |_,_|{}); });
                });
            }, |center_right_area| {
                let mut cra = center_right_area.reborrow();
                cra.dock_split_horizontal(Val::Percent(75.0), 100.0, |_| {}, |center_area| {
                    let mut ca = center_area.reborrow();
                    ca.dock_split_vertical(Val::Percent(75.0), 100.0, |_| {}, |top_area| {
                        let mut ta = top_area.reborrow();
                        ta.dock_panel(0, |_| {}, |tabs| {
                            tabs.tab("Scene 3D", false, |_, bg| bg.color = Color::NONE, move |content| {
                                content.reborrow().viewport(viewport_camera, |s| { s.flex_grow = 1.0; });
                            });
                            tabs.tab("Game", true, |_, _| {}, |content| { content.reborrow().label("Visor del Juego", |_,_|{}); });
                        });
                    }, |bottom_area| {
                        let mut ba = bottom_area.reborrow();
                        ba.dock_panel(0, |_| {}, |tabs| {
                            tabs.tab("Console", true, |_, _| {}, |content| { content.reborrow().label("Logs de consola aquí...", |_,_|{}); });
                            tabs.tab("Assets", true, |_, _| {}, |content| { content.reborrow().label("Archivos", |_,_|{}); });
                        });
                    });
                }, |right_area| {
                    let mut ra = right_area.reborrow();
                    ra.dock_panel(0, |_| {}, |tabs| {
                        tabs.tab("Inspector", false, |_, _| {}, |content| {
                            content.reborrow().vbox(|s| { s.width = Val::Percent(100.0); s.padding = UiRect::all(Val::Px(10.0)); }, |v| {
                                let mut vb = v.reborrow();
                                vb.reborrow().label("Propiedades", |_,_|{});
                                vb.reborrow().hbox(|s| { s.width = Val::Percent(100.0); s.margin = UiRect::top(Val::Px(10.0)); s.align_items = AlignItems::Center; }, |h| {
                                    let mut hb = h.reborrow();
                                    hb.reborrow().label("Color: ", |_,_|{});
                                    hb.reborrow().color_picker(Color::srgb(1.0, 0.0, 0.0), |s| { s.margin = UiRect::left(Val::Px(10.0)); });
                                });
                            });
                        });
                        tabs.tab("Settings", true, |_, _| {}, |content| { content.reborrow().label("Ajustes globales", |_,_|{}); });
                    });
                });
            });
        });
    }).id();

    commands.entity(root_entity).insert(MainUiRoot);

    // OBSERVADOR REACIVO ATÓMICO GLOBAL INTERACTIVO
    commands.add_observer(|trigger: On<Activate>, q_triggers: Query<&DialogTrigger>, q_root: Query<Entity, With<MainUiRoot>>, mut commands: Commands| {
        if let Ok(dialog_mode) = q_triggers.get(trigger.entity) {
            if let Ok(root_e) = q_root.single() {
                let start_path = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")).join("assets");
                let filters = vec![
                    FileFilter { name: "All Files (*.*)".to_string(), extensions: vec!["*.*".to_string()] },
                    FileFilter { name: "Text File (*.txt)".to_string(), extensions: vec!["*.txt".to_string()] },
                ];
                
                commands.entity(root_e).with_children(|ui| {
                    let mut local_ui = ui.reborrow();
                    match dialog_mode {
                        DialogTrigger::Open => { spawn_file_dialog(&mut local_ui, "Open File", FileDialogMode::Open, start_path, None, filters); }
                        DialogTrigger::Save => { spawn_file_dialog(&mut local_ui, "Save File", FileDialogMode::Save, start_path.clone(), Some(start_path), filters); }
                    }
                });
            }
        }
    });
}
fn auto_rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<TestCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(time.delta_secs() * 0.5);
        transform.rotate_y(time.delta_secs() * 1.0);
    }
}

fn listen_to_file_dialog_messages(
    mut ev_selected: MessageReader<RuiFileSelected>,
    mut ev_canceled: MessageReader<RuiFileCanceled>,
) {
    for ev in ev_selected.read() {
        println!("¡Archivo seleccionado! Ruta: {:?}", ev.path);
    }
    for _ in ev_canceled.read() {
        println!("Selección de archivo cancelada.");
    }
}

pub fn handle_buttons() {}
