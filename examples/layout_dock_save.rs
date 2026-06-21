use bevy::prelude::*;
//use bevy::input::mouse::MouseMotion;
//use bevy::ui::RelativeCursorPosition;

use bevy::window::PresentMode;
use bevy_rui::*;
use bevy_rui::widgets::file_dialog::{spawn_file_dialog, FileDialogMode, RuiFileSelected, RuiFileCanceled, FileFilter};
use std::env;

#[derive(Component)]
enum DialogTrigger {
    Open,
    Save,
}

#[derive(Component)]
struct MainUiRoot;

fn main() {
    App::new()
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .insert_resource(UiPickingSettings {
            require_markers: true,
        })
        //.add_plugins(DefaultPlugins)
        .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest()) // Tu configuración actual para pixel art/texturas nítidas
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            // Cambiamos el modo de presentación para evitar el crash en Hyprland
                            present_mode: PresentMode::AutoNoVsync, 
                            ..default()
                        }),
                        ..default()
                    }),
            )
        .add_plugins(MeshPickingPlugin) // Necesario en 0.18.1 para picking 3D
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        //.add_systems(Update, ( drag_cube, auto_rotate_cube))//update_camera_viewport
        .add_systems(Update,  (auto_rotate_cube, handle_buttons, listen_to_file_dialog_messages))
        .run();
}

#[derive(Component)]
struct TestCube;

/* #[derive(Component)]
struct ViewportPanel; */

/* #[derive(Component)]
struct ViewportCamera; */

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Cargar y establecer la fuente global para toda la UI
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));

    // 1. Cámara UI (Debe ser transparente para ver el 3D detrás)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // Se dibuja sobre el 3D
            clear_color: ClearColorConfig::None,
            ..default()
        },
        UiPickingCamera,
    ));

    // 2. Cámara 3D (Se dibuja al fondo, en el orden 0)
    /* commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        MeshPickingCamera, // Requerido para proyectar rayos desde esta cámara
        //ViewportCamera,
        RuiViewportCamera,
    )); */
    let viewport_camera = commands.spawn((
        Camera3d::default(),
        Camera { order: 0,
             ..default() },
        MeshPickingCamera,
        RuiViewportCamera, // <- El componente de la librería
    )).id();

    // 3. Escena 3D (Luz y Cubo Interactivo)
    commands.spawn((
        DirectionalLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(-4.0, 8.0, -4.0), //.looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(StandardMaterial { base_color: Color::srgb(0.87, 0.02, 0.02), ..default() });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, -5.0),
        TestCube,
        Pickable::default(), // PR #17266: Renombrado de PickingBehavior
        Interaction::None, // El motor de picking actualizará esto al hacer clic en el cubo
    )).observe(|_: On<Pointer<Click>>| {
        let msg = "¡Cubo 3D clickeado directamente!".to_string();
        println!("{}", msg);
    });

    // 1. Raíz que cubre la pantalla
    commands.rui_root(|_| {}, |ui| {
        
        // 2. Columna base (VBox) para separar la pantalla en dos filas horizontales (HBox)
        ui.vbox(|_| {}, |vbox| {
            
            let emoji_font = asset_server.load("fonts/openmojicolor.ttf");

            // --- MENÚ SUPERIOR ---
            vbox.menu_bar(|s| { s.flex_shrink = 0.0; }, |menu| {
                menu.submenu("File", None, 0, |_| {}, |file_menu| {
                    file_menu.menu_item("Open", Some(RuiIcon::Emoji("📁".to_string(), Some(emoji_font.clone()))), |_| {}).insert(DialogTrigger::Open);
                    file_menu.menu_item("Save", Some(RuiIcon::Emoji("💾".to_string(), Some(emoji_font.clone()))), |_| {}).insert(DialogTrigger::Save);
                });
                
                menu.submenu("Prefab", None, 0, |_| {}, |prefab_menu| {
                    prefab_menu.submenu("Native", None, 1, |_| {}, |native_menu| {
                        native_menu.menu_item("Capsule3d", None, |_| {});
                    });
                    
                    prefab_menu.submenu("Particle", None, 1, |_| {}, |particle_menu| {
                        particle_menu.menu_item("Particle3d", Some(RuiIcon::Emoji("✨".to_string(), Some(emoji_font.clone()))), |_| {});
                    });
                    
                    prefab_menu.submenu("Environment", None, 1, |_| {}, |env_menu| {
                        env_menu.menu_item("Directional Light", Some(RuiIcon::Emoji("☀️".to_string(), Some(emoji_font.clone()))), |_| {});
                        env_menu.menu_item("Skybox", Some(RuiIcon::Emoji("🌌".to_string(), Some(emoji_font.clone()))), |_| {});
                    });
                });
                
                menu.menu_item("Help", None, |_| {});
            });

            // --- ÁREA DE DOCKING (Ocupa todo el espacio restante) ---
            vbox.dock_split_horizontal(Val::Percent(20.0), 100.0, |_| {}, |left_area| {
                // Dock Izquierdo (Hierarchy)
                left_area.dock_panel(0, |_| {}, |tabs| {
                    tabs.tab("Hierarchy", false, |_, _| {}, |content| {
                        content.label("Contenido de Hierarchy", |_,_|{});
                    });
                    tabs.tab("Entities", true, |_, _| {}, |content| {
                        content.label("Contenido de Entities", |_,_|{});
                    });
                });
            }, |center_right_area| {
                center_right_area.dock_split_horizontal(Val::Percent(75.0), 100.0, |_| {}, |center_area| {
                    // Dock Central (Viewport y Consola inferior)
                    center_area.dock_split_vertical(Val::Percent(75.0), 100.0, |_| {}, |top_area| {
                        // Visor 3D (Arriba)
                        top_area.dock_panel(0, |_| {}, |tabs| {
                            tabs.tab("Scene 3D", false, |_, bg| bg.color = Color::NONE, move |content| {
                                content.viewport(viewport_camera, |s| { s.flex_grow = 1.0; });
                            });
                            tabs.tab("Game", true, |_, _| {}, |content| {
                                content.label("Visor del Juego", |_,_|{});
                            });
                        });
                    }, |bottom_area| {
                        // Consola (Abajo)
                        bottom_area.dock_panel(0, |_| {}, |tabs| {
                            tabs.tab("Console", true, |_, _| {}, |content| {
                                content.label("Logs de consola aquí...", |_,_|{});
                            });
                            tabs.tab("Assets", true, |_, _| {}, |content| {
                                content.label("Archivos", |_,_|{});
                            });
                        });
                    });
                }, |right_area| {
                    // Dock Derecho (Inspector)
                    right_area.dock_panel(0, |_| {}, |tabs| {
                        tabs.tab("Inspector", false, |_, _| {}, |content| {
                            content.label("Propiedades", |_,_|{});
                        });
                        tabs.tab("Settings", true, |_, _| {}, |content| {
                            content.label("Ajustes globales", |_,_|{});
                        });
                    });
                });
            });
        }); // <-- ¡AQUÍ CERRAMOS EL VBOX PRINCIPAL!

    }).insert(MainUiRoot);
}

/* /// Mueve el cubo cuando se arrastra el ratón dentro del área del visor
fn drag_cube(
    mut mouse_motion: MessageReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    viewport_query: Query<(&RelativeCursorPosition, &ViewVisibility), With<ViewportPanel>>,
    mut cube_query: Query<&mut Transform, With<TestCube>>,
) {
    // Solo rotamos si el botón izquierdo está presionado
    if !mouse_button.pressed(MouseButton::Left) { return; }

    // Verificamos que el ratón esté dentro del visor 3D
    let Ok((rel_pos, view_vis)) = viewport_query.single() else { return };
    if !view_vis.get() || !rel_pos.cursor_over() { return; }

    let Ok(mut transform) = cube_query.single_mut() else { return };

    for event in mouse_motion.read() {
        if mouse_button.pressed(MouseButton::Left) {
            // Rotación con Click Izquierdo
            transform.rotate_local_y(event.delta.x * 0.005);
            transform.rotate_local_x(event.delta.y * 0.005);
        } else if mouse_button.pressed(MouseButton::Right) {
            // Movimiento con Click Derecho
            transform.translation.x += event.delta.x * 0.01;
            transform.translation.y -= event.delta.y * 0.01;
        }
    }
}*/

/// Rota el cubo automáticamente para darle dinamismo a la escena 3D y probar el renderizado
fn auto_rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<TestCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(time.delta_secs() * 0.5);
        transform.rotate_y(time.delta_secs() * 1.0);
    }
}

/// Escucha los clics de nuestros botones de prueba e invoca al File Dialog.
fn handle_buttons(
    mut commands: Commands,
    q_interactions: Query<(&Interaction, &DialogTrigger), Changed<Interaction>>,
    q_root: Query<Entity, With<MainUiRoot>>,
) {
    let root_entity = if let Some(e) = q_root.iter().next() { e } else { return; };
    
    for (interaction, trigger) in &q_interactions {
        if *interaction == Interaction::Pressed {
            commands.entity(root_entity).with_children(|parent| {
                let start_path = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")).join("assets");
                
                let filters = vec![
                    FileFilter { name: "All Files (*.*)".to_string(), extensions: vec!["*.*".to_string()] },
                    FileFilter { name: "Text File (*.txt)".to_string(), extensions: vec!["*.txt".to_string()] },
                    FileFilter { name: "Tar Archive (*.tar.gz)".to_string(), extensions: vec!["*.tar.gz".to_string()] },
                ];
                
                match trigger {
                    DialogTrigger::Open => {
                        spawn_file_dialog(parent, "Open File", FileDialogMode::Open, start_path.clone(), filters.clone());
                    }
                    DialogTrigger::Save => {
                        spawn_file_dialog(parent, "Save File", FileDialogMode::Save, start_path.clone(), filters.clone());
                    }
                }
            });
        }
    }
}

/// Lee los eventos generados por el File Dialog y muestra el resultado en la consola.
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

/* 
/// Mantiene el Viewport de la Cámara 3D sincronizado con el tamaño y posición del Panel Central
fn update_camera_viewport(
    ui_query: Query<(&GlobalTransform, &ComputedNode, &ViewVisibility), With<ViewportPanel>>,
    mut camera_query: Query<&mut Camera, With<ViewportCamera>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Ok((transform, node, view_vis)) = ui_query.single() else { return };
    let Ok(mut camera) = camera_query.single_mut() else { return };
    let Ok(window) = window_query.single() else { return };

    let size = node.size();
    if size.x <= 0.0 || size.y <= 0.0 || !view_vis.get() {
        camera.is_active = false;
        return;
    }
    camera.is_active = true;

    // Convertimos la posición de la UI (basada en el centro) a coordenadas físicas (top-left) de la pantalla
    let scale_factor = window.scale_factor();
    let pos = transform.translation();
    
    let physical_size = UVec2::new((size.x * scale_factor) as u32, (size.y * scale_factor) as u32);
    let physical_pos = UVec2::new(
        ((pos.x - size.x / 2.0) * scale_factor).max(0.0) as u32,
        ((pos.y - size.y / 2.0) * scale_factor).max(0.0) as u32,
    );

    // Le aplicamos el candado a la visión de la cámara
    camera.viewport = Some(bevy::camera::Viewport {
        physical_position: physical_pos,
        physical_size,
        ..default()
    });
} */