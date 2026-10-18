use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::ui::RelativeCursorPosition;
use bevy::ui_widgets::Activate; 

use bevy_rui::*;

fn main() {
    App::new()
        // Configuración nativa unificada de interacción para Bevy 0.20
        .insert_resource(MeshPickingSettings {
            require_markers: false,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .insert_resource(UiPickingSettings {
            require_markers: false,
        })
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // SANEAMIENTO DE LOGS: Forzamos nivel INFO para silenciar los logs ruidosos de la GPU en la consola
            level: bevy::log::Level::INFO,
            ..default()
        }))
        .add_plugins(MeshPickingPlugin) 
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_camera_viewport)
        .add_systems(Update, drag_cube)
        .run();
}

#[derive(Component)] struct TestButton;
#[derive(Component)] struct TestTextBox;
#[derive(Component)] struct TestCube;
#[derive(Component)] struct EditableConsole;
#[derive(Component)] struct ReadonlyConsole;
#[derive(Component)] struct ViewportPanel;
#[derive(Component)] struct ViewportCamera;


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font_handle = asset_server.load("fonts/afacad.ttf");
    commands.insert_resource(RuiDefaultFont(font_handle));

    // 1. Cámara UI
    commands.spawn((
        Camera2d,
        Camera { order: 1, clear_color: ClearColorConfig::None, ..default() },
        UiPickingCamera,
    ));

    // 2. Cámara 3D
    commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        MeshPickingCamera, 
        ViewportCamera,
    ));

    // 3. Escena 3D
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
    )).observe(|_: On<Activate>, 
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
    let emoji_font = asset_server.load("fonts/openmojicolor.ttf");

    commands.rui_root(|_| {}, |ui| {
        let mut root_spawner = ui.reborrow();
        
        root_spawner.vbox(|_| {}, |vbox| {
            let mut vb = vbox.reborrow();
            
            // --- MENÚ SUPERIOR ---
            let mut menu_bar_binding = vb.reborrow();
            menu_bar_binding.menu_bar(|s| { s.flex_shrink = 0.0; }, |menu| {
                let mut m = menu.reborrow();
                
                let mut sub1 = m.reborrow();
                sub1.submenu("File", None, 0, |_| {}, |file_menu| {
                    let mut fm = file_menu.reborrow();
                    let mut item1 = fm.reborrow();
                    item1.menu_item("Open", Some(RuiIcon::Emoji("📁".to_string(), Some(emoji_font.clone()))), |_| {});
                    let mut item2 = fm.reborrow();
                    item2.menu_item("Save", Some(RuiIcon::Emoji("💾".to_string(), Some(emoji_font.clone()))), |_| {});
                });
                
                let mut sub2 = m.reborrow();
                sub2.submenu("Prefab", None, 0, |_| {}, |prefab_menu| {
                    let mut pm = prefab_menu.reborrow();
                    
                    let mut p_sub1 = pm.reborrow();
                    p_sub1.submenu("Native", None, 1, |_| {}, |native_menu| {
                        let mut nm = native_menu.reborrow();
                        nm.menu_item("Capsule3d", None, |_| {});
                    });
                    
                    let mut p_sub2 = pm.reborrow();
                    p_sub2.submenu("Particle", None, 1, |_| {}, |particle_menu| {
                        let mut ptm = particle_menu.reborrow();
                        ptm.menu_item("Particle3d", Some(RuiIcon::Emoji("✨".to_string(), Some(emoji_font.clone()))), |_| {});
                    });
                    
                    let mut p_sub3 = pm.reborrow();
                    p_sub3.submenu("Environment", None, 1, |_| {}, |env_menu| {
                        let mut em = env_menu.reborrow();
                        let mut item1 = em.reborrow();
                        item1.menu_item("Directional Light", Some(RuiIcon::Emoji("☀️".to_string(), Some(emoji_font.clone()))), |_| {});
                        let mut item2 = em.reborrow();
                        item2.menu_item("Skybox", Some(RuiIcon::Emoji("🌌".to_string(), Some(emoji_font.clone()))), |_| {});
                    });
                });
                
                let mut sub3 = m.reborrow();
                sub3.menu_item("Help", None, |_| {});
            });

            // --- FILA CENTRAL (Paneles del Editor) ---
            let mut hbox_binding = vb.reborrow();
            hbox_binding.hbox(|s| { s.flex_grow = 1.0; }, |top_hbox| {
                let mut hbox = top_hbox.reborrow();
                
                // Columna Izquierda (20%)
                let mut left_binding = hbox.reborrow();
                left_binding.vbox(|s| { 
                    s.width = Val::Percent(20.0); s.justify_content = JustifyContent::Center; s.align_items = AlignItems::Center; s.overflow = Overflow::visible();
                }, |left_panel| {
                    let mut lp = left_panel.reborrow();
                    
                    let mut btn_binding = lp.reborrow();
                    let mut btn_cmd = btn_binding.button(|s| { s.width = Val::Px(160.0); s.height = Val::Px(45.0); }, |btn| {
                        let mut b = btn.reborrow(); b.label("¡Hazme Clic!", |font, _| font.font_size = FontSize::Px(20.0));
                    });
                    
                    btn_cmd.insert(TestButton).observe(|_: On<Activate>,
                                                         mut textbox_query: Query<&mut RuiTextBox, (With<TestTextBox>, Without<EditableConsole>, Without<ReadonlyConsole>)>,
                                                         mut editable_console_q: Query<&mut RuiTextBox, (With<EditableConsole>, Without<TestTextBox>, Without<ReadonlyConsole>)>,
                                                         mut readonly_console_q: Query<&mut RuiTextBox, (With<ReadonlyConsole>, Without<TestTextBox>, Without<EditableConsole>)>| {
                        if let Ok(mut textbox) = textbox_query.single_mut() {
                            let msg_to_log; // SANEAMIENTO ADVERTENCIA: Eliminamos asignación inicial redundante
                            if textbox.text.is_empty() { 
                                msg_to_log = Some("textbox vacio.".to_string()); 
                            } else {
                                msg_to_log = Some(format!("texto borrado: {}", textbox.text));
                                textbox.text.clear(); textbox.cursor_index = 0;
                            }
                            if let Some(msg) = msg_to_log {
                                if let Ok(mut editable) = editable_console_q.single_mut() { editable.text.push_str(&format!("{}\n", msg)); editable.cursor_index = editable.text.chars().count(); }
                                if let Ok(mut readonly) = readonly_console_q.single_mut() { readonly.text.push_str(&format!("{}\n", msg)); readonly.cursor_index = readonly.text.chars().count(); }
                            }
                        }
                    });
                    let mut ctx_binding = lp.reborrow();
                    ctx_binding.vbox(|s| {
                        s.width = Val::Px(160.0); s.height = Val::Px(45.0); s.margin = UiRect::top(Val::Px(15.0));
                        s.border = UiRect::all(Val::Px(1.0)); s.justify_content = JustifyContent::Center; s.align_items = AlignItems::Center;
                    }, |area| {
                        let mut a = area.reborrow();
                        let mut l_bind = a.reborrow();
                        l_bind.label("Right Click Me!", |font, _color| font.font_size = FontSize::Px(16.0));
                        let mut menu_bind = a.reborrow();
                        menu_bind.context_menu(|menu| {
                            let mut m = menu.reborrow();
                            let mut i1 = m.reborrow(); i1.context_menu_item("Action 1", None, |_|{});
                            let mut i2 = m.reborrow(); i2.context_menu_item("Action 2", None, |_|{});
                            let mut i3 = m.reborrow(); i3.context_menu_item("Delete", None, |_|{});
                        });
                    });

                    let mut drop_binding = lp.reborrow();
                    drop_binding.dropdown("Elige Clase", &["Guerrero", "Mago", "Arquero", "Ladrón"], |s| {
                        s.width = Val::Px(170.0); s.height = Val::Px(45.0); s.margin = UiRect::top(Val::Px(15.0)); s.border = UiRect::all(Val::Px(1.0));
                    });

                    let mut txt_binding1 = lp.reborrow();
                    txt_binding1.textbox("Texto a borrar...", |s, font, _| {
                        s.width = Val::Px(250.0); s.height = Val::Px(45.0); s.margin = UiRect::top(Val::Px(15.0)); font.font_size = FontSize::Px(18.0);
                    }).insert(TestTextBox);

                    let mut txt_binding2 = lp.reborrow();
                    txt_binding2.textbox("Escribe tu nombre...", |s, font, _| {
                        s.width = Val::Px(300.0); s.height = Val::Px(45.0); s.margin = UiRect::top(Val::Px(15.0)); font.font_size = FontSize::Px(22.0);
                    }).insert(RuiTextBox {
                        placeholder: "Escribe tu nombre...".to_string(), max_characters: Some(15), ..default()
                    }).with_tooltip("Escribe tu Nombre. Máximo 15 caracteres.", Color::srgb(0.24, 0.22, 0.22), Color::srgb(1.0, 0.8, 0.0));
                }).insert(BackgroundColor(Color::srgb(0.8, 0.2, 0.2)));

                // Columna Central (60% - Visor 3D)
                let mut center_binding = hbox.reborrow();
                center_binding.vbox(|s| { 
                    s.width = Val::Percent(60.0); s.justify_content = JustifyContent::Center; s.align_items = AlignItems::Center; s.row_gap = Val::Px(20.0);
                }, |center_panel| {
                    let mut cp = center_panel.reborrow();
                    cp.label("Visor 3D", |font, color| { font.font_size = FontSize::Px(40.0); color.0 = Color::srgb(1.0, 1.0, 0.0); });
                }).insert((ViewportPanel, Pickable::IGNORE));

                // Columna Derecha (20%)
                let mut right_binding = hbox.reborrow();
                right_binding.vbox(|s| { 
                    s.width = Val::Percent(20.0); s.justify_content = JustifyContent::Center; s.align_items = AlignItems::Center;
                }, |right_panel| {
                    let mut rp = right_panel.reborrow();
                    let mut l_bind = rp.reborrow(); l_bind.label("Menú Derecho", |font, _| font.font_size = FontSize::Px(24.0));
                    
                    let mut scr_bind = rp.reborrow();
                    scr_bind.scrollview(|s| {
                        s.width = Val::Percent(20.0); s.height = Val::Percent(50.0); s.padding = UiRect::all(Val::Px(10.0));
                    }, |scroll| {
                        let mut sc = scroll.reborrow();
                        for i in 1..=20 {
                            let mut item_bind = sc.reborrow();
                            item_bind.button(|s| { s.width = Val::Percent(100.0); s.height = Val::Px(40.0); s.margin = UiRect::bottom(Val::Px(5.0)); }, |btn| {
                                let mut b = btn.reborrow(); b.label(&format!("Opción {}", i), |_,_| {});
                            });
                        }
                    });
                    
                    let mut acc_bind = rp.reborrow();
                    acc_bind.accordion("Opciones Avanzadas", |s| { s.margin = UiRect::top(Val::Px(15.0)); }, |acc| {
                        let mut ac = acc.reborrow();
                        let mut chk1 = ac.reborrow(); chk1.checkbox("Habilitar Raytracing", false, |_| {});
                        let mut chk2 = ac.reborrow(); chk2.checkbox("Filtro Bilineal", true, |_| {});
                        let mut btn1 = ac.reborrow(); btn1.button(|s| { s.margin = UiRect::top(Val::Px(10.0)); }, |btn| { let mut b = btn.reborrow(); b.label("Aplicar Cambios", |_,_| {}); });
                    });
                }).insert(BackgroundColor(Color::srgb(0.2, 0.2, 0.8)));
            });

            // --- FILA INFERIOR (Consolas del Editor) ---
            let mut bot_binding = vb.reborrow();
            bot_binding.hbox(|s| { s.height = Val::Percent(20.0); s.flex_shrink = 0.0; }, |bottom_hbox| {
                let mut bhbox = bottom_hbox.reborrow();
                let mut txt1 = bhbox.reborrow(); txt1.multiline_textbox("Consola Editable...", |s, font, _| { s.width = Val::Percent(50.0); s.height = Val::Percent(100.0); font.font_size = FontSize::Px(18.0); }).insert(EditableConsole);
                let mut txt2 = bhbox.reborrow(); txt2.multiline_textbox("Consola Solo Lectura...", |s, font, color| { s.width = Val::Percent(50.0); s.height = Val::Percent(100.0); font.font_size = FontSize::Px(18.0); color.0 = Color::srgb(0.7, 0.7, 0.7); }).insert((ReadonlyConsole, RuiTextBox { placeholder: "Consola Solo Lectura...".to_string(), mode: RuiTextBoxMode::MultiLine, readonly: true, ..default() }));
            });
        });

        // CORRECCIÓN DEFINITIVA: Usamos directamente el spawner fresco 'ui' para evadir el bloqueo de 'vbox'
        let mut w1 = ui.reborrow();
        w1.window("Caja de Herramientas", true, |s| { s.left = Val::Px(50.0); s.top = Val::Px(50.0); }, |win, _| {
            let mut w = win.reborrow(); w.label("¡Soy una ventana flotante libre!", |_,_| {});
        });
        
        let mut w2 = ui.reborrow();
        w2.window("Caja de Herramientas 2", false, |s| { s.left = Val::Px(400.0); s.top = Val::Px(150.0); }, |win, _| {
            let mut w = win.reborrow(); w.label("¡Soy una ventana flotante libre!", |_,_| {});
        });
    });
}
fn drag_cube(
    mut mouse_motion: MessageReader<MouseMotion>, 
    mouse_button: Res<ButtonInput<MouseButton>>,
    viewport_query: Query<&RelativeCursorPosition, With<ViewportPanel>>,
    mut cube_query: Query<&mut Transform, With<TestCube>>,
) {
    if !mouse_button.pressed(MouseButton::Left) && !mouse_button.pressed(MouseButton::Right) { return; }

    let Ok(rel_pos) = viewport_query.single() else { return };
    if !rel_pos.cursor_over() { return; }

    let Ok(mut transform) = cube_query.single_mut() else { return };

    for event in mouse_motion.read() {
        if mouse_button.pressed(MouseButton::Left) {
            transform.rotate_local_y(event.delta.x * 0.005);
            transform.rotate_local_x(event.delta.y * 0.005);
        } else if mouse_button.pressed(MouseButton::Right) {
            transform.translation.x += event.delta.x * 0.01;
            transform.translation.y -= event.delta.y * 0.01;
        }
    }
}

fn update_camera_viewport(
    ui_query: Query<(&GlobalTransform, &ComputedNode), With<ViewportPanel>>,
    mut camera_query: Query<&mut Camera, With<ViewportCamera>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Ok((transform, node)) = ui_query.single() else { return };
    let Ok(mut camera) = camera_query.single_mut() else { return };
    let Ok(window) = window_query.single() else { return };

    let size = node.size();
    if size.x <= 0.0 || size.y <= 0.0 { return; }

    let scale_factor = window.scale_factor();
    let pos = transform.translation();
    
    let physical_size = UVec2::new((size.x * scale_factor) as u32, (size.y * scale_factor) as u32);
    let physical_pos = UVec2::new(
        ((pos.x - size.x / 2.0) * scale_factor) as u32,
        ((pos.y - size.y / 2.0) * scale_factor) as u32,
    );

    camera.viewport = Some(bevy::camera::Viewport {
        physical_position: physical_pos,
        physical_size,
        ..default()
    });
}
