use bevy::prelude::*;
use bevy::window::PresentMode;
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
        .add_systems(Update, auto_rotate_cube)
        .run();
}

#[derive(Component)]
struct TestCube;

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
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        UiPickingCamera,
    ));

    let viewport_camera = commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        MeshPickingCamera,
        RuiViewportCamera,
    )).id();

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
        Pickable::default(),
    )).observe(|_: On<PointerClick>| {
        let msg = "¡Cubo 3D clickeado directamente!".to_string();
        println!("{}", msg);
    });

    // 1. Raíz que cubre la pantalla
    commands.rui_root(|_| {}, |ui| {
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
                    fm.reborrow().menu_item("Open", Some(RuiIcon::Emoji("📁".to_string(), Some(emoji_font.clone()))), |_| {});
                    fm.reborrow().menu_item("Save", Some(RuiIcon::Emoji("💾".to_string(), Some(emoji_font.clone()))), |_| {});
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

            // --- ÁREA DE DOCKING ---
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
                            content.reborrow().label("Propiedades", |_,_|{});
                        });
                        tabs.tab("Settings", true, |_, _| {}, |content| { content.reborrow().label("Ajustes globales", |_,_|{}); });
                    });
                });
            });
        });
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
