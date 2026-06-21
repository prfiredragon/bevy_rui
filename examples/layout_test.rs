use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::ui::RelativeCursorPosition;

use bevy_rui::*;

fn main() {
    App::new()
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .insert_resource(UiPickingSettings {
            require_markers: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin) // Necesario en 0.18.1 para picking 3D
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_test_clicks, update_camera_viewport, drag_cube))
        .run();
}

#[derive(Component)]
struct TestButton;

#[derive(Component)]
struct TestTextBox;

#[derive(Component)]
struct TestCube;

#[derive(Component)]
struct EditableConsole;

#[derive(Component)]
struct ReadonlyConsole;

#[derive(Component)]
struct ViewportPanel;

#[derive(Component)]
struct ViewportCamera;

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
    commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        MeshPickingCamera, // Requerido para proyectar rayos desde esta cámara
        ViewportCamera,
    ));

    // 3. Escena 3D (Luz y Cubo Interactivo)
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
        Pickable::default(), // PR #17266: Renombrado de PickingBehavior
        Interaction::None, // El motor de picking actualizará esto al hacer clic en el cubo
    )).observe(|_: On<Pointer<Click>>, 
                 mut q_ed: Query<&mut RuiTextBox, (With<EditableConsole>, Without<ReadonlyConsole>, Without<TestTextBox>)>, 
                 mut q_ro: Query<&mut RuiTextBox, (With<ReadonlyConsole>, Without<EditableConsole>, Without<TestTextBox>)>| {
        let msg = "¡Cubo 3D clickeado directamente!".to_string();
        println!("{}", msg);
        
        if let Ok(mut textbox) = q_ed.single_mut() {
            textbox.text.push_str(&format!("{}\n", msg));
            textbox.cursor_index = textbox.text.chars().count();
        }
        if let Ok(mut textbox) = q_ro.single_mut() {
            textbox.text.push_str(&format!("{}\n", msg));
            textbox.cursor_index = textbox.text.chars().count();
        }
    });

    // 1. Raíz que cubre la pantalla
    commands.rui_root(|_| {}, |ui| {
        
        // 2. Columna base (VBox) para separar la pantalla en dos filas horizontales (HBox)
        ui.vbox(|_| {}, |vbox| {
            
            let emoji_font = asset_server.load("fonts/openmojicolor.ttf");

            // --- MENÚ SUPERIOR ---
            vbox.menu_bar(|s| { s.flex_shrink = 0.0; }, |menu| {
                menu.submenu("File", None, 0, |_| {}, |file_menu| {
                    file_menu.menu_item("Open", Some(RuiIcon::Emoji("📁".to_string(), Some(emoji_font.clone()))), |_| {});
                    file_menu.menu_item("Save", Some(RuiIcon::Emoji("💾".to_string(), Some(emoji_font.clone()))), |_| {});
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

            // --- FILA SUPERIOR (Ocupa el espacio restante por defecto) ---
            vbox.hbox(|s| { s.flex_grow = 1.0; }, |top_hbox| {
                
                // Dentro de esta primera fila de 80%, colocamos 3 columnas verticales (VBox)
                
                // Columna 1 (Izquierda - 20%)
                top_hbox.vbox(|s| { 
                    s.width = Val::Percent(20.0); 
                    s.justify_content = JustifyContent::Center;
                    s.align_items = AlignItems::Center;
                }, |left_panel| {
                    
                    // ¡AQUÍ ESTÁ EL BOTÓN!
                    left_panel.button(|s| {
                        s.width = Val::Px(160.0);
                        s.height = Val::Px(45.0);
                    }, |btn| {
                        btn.label("¡Hazme Clic!", |font, _color| font.font_size = FontSize::Px(20.0));
                    }).insert(TestButton);

                    // ¡AQUÍ ESTÁ EL NUEVO DROPDOWN!
                    left_panel.dropdown("Elige Clase", &["Guerrero", "Mago", "Arquero", "Ladrón"], |s| {
                        s.width = Val::Px(170.0);
                        s.height = Val::Px(45.0);
                        s.margin = UiRect::top(Val::Px(15.0));
                        s.border = UiRect::all(Val::Px(1.0));
                    });

                    // ¡AQUÍ ESTÁ EL TEXTBOX!
                    left_panel.textbox("Texto a borrar...", |s, font, _color| {
                        s.width = Val::Px(250.0);
                        s.height = Val::Px(45.0);
                        s.margin = UiRect::top(Val::Px(15.0)); // Añadimos un pequeño margen arriba
                        font.font_size = FontSize::Px(18.0);
                    }).insert(TestTextBox);

                    left_panel.textbox("Escribe tu nombre...", |s, font, _color| {
                        s.width = Val::Px(300.0);
                        s.height = Val::Px(45.0);
                        s.margin = UiRect::top(Val::Px(15.0)); 
                        font.font_size = FontSize::Px(22.0);
                    }).insert(RuiTextBox {
                        placeholder: "Escribe tu nombre...".to_string(),
                        max_characters: Some(15),
                        ..default()
                    }).with_tooltip("Escribe tu Nombre. Máximo 15 caracteres.", Color::srgb(0.24, 0.22, 0.22), Color::srgb(1.0, 0.8, 0.0));
                    

                   


                }).insert(BackgroundColor(Color::srgb(0.8, 0.2, 0.2))); // Rojo
                    
                // Columna 2 (Centro - 60%)
                top_hbox.vbox(|s| { 
                    s.width = Val::Percent(60.0); 
                    s.justify_content = JustifyContent::Center;
                    s.align_items = AlignItems::Center;
                    s.row_gap = Val::Px(20.0);
                }, |_center_panel| {
                    _center_panel.label("Visor 3D", |font, color| {
                        font.font_size = FontSize::Px(40.0);
                        color.0 = Color::srgb(1.0, 1.0, 0.0); // Texto amarillo
                    });

                    
                }).insert((ViewportPanel, Pickable::IGNORE)); // Pickable::IGNORE permite que el clic pase al 3D
                    
                // Columna 3 (Derecha - 20%)
                top_hbox.vbox(|s| { 
                    s.width = Val::Percent(20.0); 
                    s.justify_content = JustifyContent::Center;
                    s.align_items = AlignItems::Center;
                }, |right_panel| {
                    right_panel.label("Menú Derecho", |font, _| font.font_size = FontSize::Px(24.0));
                     // Ejemplo rápido de un Panel Derecho tipo Menú con Scroll!
                    right_panel.scrollview(|s| {
                        s.width = Val::Percent(20.0);
                        s.height = Val::Percent(50.0);
                        s.padding = UiRect::all(Val::Px(10.0));
                    }, |scroll| {
                        for i in 1..=20 {
                            scroll.button(|s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Px(40.0);
                                s.margin = UiRect::bottom(Val::Px(5.0)); // Espacio entre botones
                            }, |btn| {
                                btn.label(&format!("Opción {}", i), |_,_| {});
                            });
                        }
                    });
                    right_panel.accordion("Opciones Avanzadas", |s| {
                        s.margin = UiRect::top(Val::Px(15.0));
                    }, |acc| {
                        acc.checkbox("Habilitar Raytracing", false, |_| {});
                        acc.checkbox("Filtro Bilineal", true, |_| {});
                        acc.button(|s| { s.margin = UiRect::top(Val::Px(10.0)); }, |btn| { btn.label("Aplicar Cambios", |_,_| {}); });
                    });

                }).insert(BackgroundColor(Color::srgb(0.2, 0.2, 0.8))); // Azul
            });

            // --- FILA INFERIOR (Ocupa el 20% del alto por defecto) ---
            vbox.hbox(|s| { s.height = Val::Percent(20.0); s.flex_shrink = 0.0; }, |bottom_hbox| {
                
                // Consola 1: Editable
                bottom_hbox.multiline_textbox("Consola Editable...", |s, font, _color| {
                    s.width = Val::Percent(50.0);
                    s.height = Val::Percent(100.0);
                    font.font_size = FontSize::Px(18.0);
                }).insert(EditableConsole);

                // Consola 2: Readonly (Solo lectura)
                bottom_hbox.multiline_textbox("Consola Solo Lectura...", |s, font, color| {
                    s.width = Val::Percent(50.0);
                    s.height = Val::Percent(100.0);
                    font.font_size = FontSize::Px(18.0);
                    color.0 = Color::srgb(0.7, 0.7, 0.7); // Texto más gris para diferenciar
                }).insert(ReadonlyConsole).insert(RuiTextBox {
                    placeholder: "Consola Solo Lectura...".to_string(),
                    mode: RuiTextBoxMode::MultiLine,
                    readonly: true, // ¡NUEVA PROPIEDAD! Bloquea la escritura.
                    ..default()
                });
            });
        }); // <-- ¡AQUÍ CERRAMOS EL VBOX PRINCIPAL!

        // Ventana Flotante (Se añade directo a `ui`, para que flote encima de todo el Layout)
        ui.window("Caja de Herramientas", true, |s| {
            s.left = Val::Px(50.0);
            s.top = Val::Px(50.0);
        }, |win, _| {
            win.label("¡Soy una ventana flotante libre!", |_,_| {});
            win.button(|s| { s.margin = UiRect::top(Val::Px(10.0)); }, |btn| {
                btn.label("Aceptar", |_,_| {});
            });
        });

        // Ventana Flotante (Se añade directo a `ui`, para que flote encima de todo el Layout)
        ui.window("Caja de Herramientas 2", false, |s| {
            s.left = Val::Px(400.0);
            s.top = Val::Px(150.0);
        }, |win, _| {
            win.label("¡Soy una ventana flotante libre!", |_,_| {});
            win.button(|s| { s.margin = UiRect::top(Val::Px(10.0)); }, |btn| {
                btn.label("Aceptar", |_,_| {});
            });
        });

    });
}

fn handle_test_clicks(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TestButton>)>,
    cube_query: Query<&Interaction, (Changed<Interaction>, With<TestCube>)>,
    mut textbox_query: Query<&mut RuiTextBox, (With<TestTextBox>, Without<EditableConsole>, Without<ReadonlyConsole>)>,
    mut editable_console_q: Query<&mut RuiTextBox, (With<EditableConsole>, Without<TestTextBox>, Without<ReadonlyConsole>)>,
    mut readonly_console_q: Query<&mut RuiTextBox, (With<ReadonlyConsole>, Without<TestTextBox>, Without<EditableConsole>)>,
) {
    let mut msg_to_log = None;

    // Detectar clic en el Cubo 3D
    for interaction in &cube_query {
        if *interaction == Interaction::Pressed {
            msg_to_log = Some("¡Cubo 3D clickeado directamente!".to_string());
            println!("{}", msg_to_log.as_ref().unwrap());
        }
    }

    // Detectar clic en el Botón de la UI
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut textbox) = textbox_query.single_mut() {
                if textbox.text.is_empty() {
                    msg_to_log = Some("textbox vacio.".to_string());
                    println!("textbox vacio.");
                } else {
                    msg_to_log = Some(format!("texto borrado: {}", textbox.text));
                    println!("texto borrado: {}", textbox.text);
                    textbox.text.clear();
                    textbox.cursor_index = 0; // Reiniciamos el cursor al principio
                }
            }
        }
    }

    // Si hay un mensaje nuevo (venga del cubo o del botón), lo enviamos a las consolas
    if let Some(msg) = msg_to_log {
        if let Ok(mut editable) = editable_console_q.single_mut() {
            editable.text.push_str(&format!("{}\n", msg));
            editable.cursor_index = editable.text.chars().count();
        }
        if let Ok(mut readonly) = readonly_console_q.single_mut() {
            readonly.text.push_str(&format!("{}\n", msg));
            readonly.cursor_index = readonly.text.chars().count();
        }
    }
}

/// Mueve el cubo cuando se arrastra el ratón dentro del área del visor
fn drag_cube(
    mut mouse_motion: MessageReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    viewport_query: Query<&RelativeCursorPosition, With<ViewportPanel>>,
    mut cube_query: Query<&mut Transform, With<TestCube>>,
) {
    // Solo rotamos si el botón izquierdo está presionado
    if !mouse_button.pressed(MouseButton::Left) { return; }

    // Verificamos que el ratón esté dentro del visor 3D
    let Ok(rel_pos) = viewport_query.single() else { return };
    if !rel_pos.cursor_over() { return; }

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
}

/// Mantiene el Viewport de la Cámara 3D sincronizado con el tamaño y posición del Panel Central
fn update_camera_viewport(
    ui_query: Query<(&GlobalTransform, &ComputedNode), With<ViewportPanel>>,
    mut camera_query: Query<&mut Camera, With<ViewportCamera>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Ok((transform, node)) = ui_query.single() else { return };
    let Ok(mut camera) = camera_query.single_mut() else { return };
    let Ok(window) = window_query.single() else { return };

    let size = node.size();
    if size.x <= 0.0 || size.y <= 0.0 {
        return;
    }

    // Convertimos la posición de la UI (basada en el centro) a coordenadas físicas (top-left) de la pantalla
    let scale_factor = window.scale_factor();
    let pos = transform.translation();
    
    let physical_size = UVec2::new((size.x * scale_factor) as u32, (size.y * scale_factor) as u32);
    let physical_pos = UVec2::new(
        ((pos.x - size.x / 2.0) * scale_factor) as u32,
        ((pos.y - size.y / 2.0) * scale_factor) as u32,
    );

    // Le aplicamos el candado a la visión de la cámara
    camera.viewport = Some(bevy::camera::Viewport {
        physical_position: physical_pos,
        physical_size,
        ..default()
    });
}