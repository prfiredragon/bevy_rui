use bevy::{prelude::*, window::PresentMode};
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
        .add_plugins(bevy_rui::RuiPlugin)
        .insert_resource(bevy_rui::navigation::RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, animate_cube)
        .add_systems(Update, handle_buttons)
        .run();
}

#[derive(Component)]
struct AnimatedCube;

#[derive(Component)]
struct SettingsWindow;

#[derive(Component)]
enum MenuAction {
    Play,
    Settings,
    Quit,
    CloseSettings,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut theme: ResMut<RuiTheme>,
) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    
    theme.image_panel = Some(asset_server.load("theme/kenney/blue/button_square_flat.png"));
    theme.panel_margin = 10.0;
    // Cabecera de ventana
    theme.image_window_header = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.window_header_margin = 10.0;
    theme.window_header_layout_margin = UiRect::all(Val::Px(5.0));
    theme.color_window_header_text = Color::srgb(0.2, 0.2, 0.2);

    // Botones
    theme.image_button_normal = Some(asset_server.load("theme/kenney/yellow/button_square_depth_gradient.png"));
    theme.image_button_hover = Some(asset_server.load("theme/kenney/yellow/button_square_depth_flat.png"));
    theme.image_button_pressed = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.button_margin = 10.0;
    theme.color_button_text = Color::srgb(0.2, 0.2, 0.2);

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

    // 3D Scene
    // A fullscreen camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera { order: 0, ..default() },
        MeshPickingCamera, // Requerido para proyectar rayos desde esta cámara
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.3))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        AnimatedCube,
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // UI - Floating Window
    commands.rui_root(|s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
    }, |rui| {
        rui.window("Game Menu", false, |s| {
            s.width = Val::Px(250.0);
            s.left = Val::Auto; // Center horizontally
            s.top = Val::Auto;  // Center vertically
        }, |win, _| {
            win.button(|s| {
                s.width = Val::Percent(100.0);
                s.margin = UiRect::bottom(Val::Px(10.0));
            }, |b| {
                b.label("Play", |_,_|{}).insert(crate::theme::RuiThemeElement::ButtonText);
            }).insert((MenuAction::Play, bevy::input_focus::AutoFocus));

            win.button(|s| {
                s.width = Val::Percent(100.0);
                s.margin = UiRect::bottom(Val::Px(10.0));
            }, |b| {
                b.label("Settings", |_,_|{}).insert(crate::theme::RuiThemeElement::ButtonText);
            }).insert(MenuAction::Settings);

            win.button(|s| {
                s.width = Val::Percent(100.0);
            }, |b| {
                b.label("Quit", |_,_|{}).insert(crate::theme::RuiThemeElement::ButtonText);
            }).insert(MenuAction::Quit);
        });
    });
}

fn animate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<AnimatedCube>>,
) {
    for mut transform in &mut query {
        let t = time.elapsed_secs();
        // Circular motion: top to left, left to bottom, bottom to right, right to top
        // Start at top (0, 2), then left (-2, 0)...
        // x = -2 * sin(t), y = 2 * cos(t)
        transform.translation.x = -t.sin() * 2.0;
        transform.translation.y = t.cos() * 2.0;
        
        // Rotation
        transform.rotate_x(1.0 * time.delta_secs());
        transform.rotate_y(2.0 * time.delta_secs());
    }
}

fn handle_buttons(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &MenuAction), Changed<Interaction>>,
    settings_windows: Query<Entity, With<SettingsWindow>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction == Interaction::Pressed {
            match action {
                MenuAction::Play => {
                    println!("Play button clicked!");
                }
                MenuAction::Settings => {
                    println!("Settings button clicked!");
                    commands.rui_root(|s| {
                        s.justify_content = JustifyContent::Center;
                        s.align_items = AlignItems::Center;
                        s.position_type = PositionType::Absolute;
                        s.width = Val::Percent(100.0);
                        s.height = Val::Percent(100.0);
                    }, |rui| {
                        rui.window("Settings", false, |s| {
                            s.width = Val::Px(500.0);
                            s.height = Val::Px(400.0);
                        }, |win, _| {
                            win.tabs(0, |s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Auto;
                                s.flex_basis = Val::Px(0.0);
                                s.flex_grow = 1.0;
                                s.flex_shrink = 1.0;
                            }, |tabs| {
                                tabs.tab("General", false, |_, _| {}, |content| {
                                    // Row with label and textbox
                                    content.spawn(Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::bottom(Val::Px(15.0)),
                                        ..default()
                                    }).with_children(|row| {
                                        row.label("Nombre del personaje: ", |_,_| {});
                                        row.textbox("Ingresa nombre...", |s, _, _| {
                                            s.width = Val::Px(150.0);
                                            s.margin = UiRect::left(Val::Px(10.0));
                                        });
                                    });

                                    // Checkbox
                                    content.checkbox("Habilitar sonido espacial", true, |s| {
                                        s.margin = UiRect::bottom(Val::Px(15.0));
                                    });

                                    // Dropdown
                                    content.dropdown("Calidad Gráfica", &["Baja", "Media", "Alta", "Ultra"], |s| {
                                        s.width = Val::Px(200.0);
                                    });
                                });

                                tabs.tab("Audio", false, |_, _| {}, |content| {
                                    content.spawn(Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::bottom(Val::Px(15.0)),
                                        ..default()
                                    }).with_children(|row| {
                                        row.label("Volumen Maestro: ", |_,_| {});
                                        row.slider(0.0, 100.0, 50.0, |s| {
                                            s.margin = UiRect::left(Val::Px(10.0));
                                        });
                                    });

                                    content.spawn(Node {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::bottom(Val::Px(15.0)),
                                        ..default()
                                    }).with_children(|row| {
                                        row.label("Efectos SFX: ", |_,_| {});
                                        row.slider(0.0, 100.0, 75.0, |s| {
                                            s.margin = UiRect::left(Val::Px(10.0));
                                        });
                                    });
                                });

                                tabs.tab("Otros", false, |_, _| {}, |content| {
                                    content.accordion("Opciones Avanzadas", |s| {
                                        s.width = Val::Percent(100.0);
                                    }, |acc| {
                                        acc.checkbox("Modo desarrollador", false, |s| {
                                            s.margin = UiRect::bottom(Val::Px(10.0));
                                        });
                                        acc.checkbox("Mostrar FPS", true, |_| {});
                                    });
                                });
                            });

                            // Back Button
                            win.button(|s| {
                                s.width = Val::Px(120.0);
                                s.margin = UiRect::top(Val::Px(15.0));
                                s.align_self = AlignSelf::Center;
                            }, |b| {
                                b.label("Back", |_,_| {}).insert(crate::theme::RuiThemeElement::ButtonText);
                            }).insert(MenuAction::CloseSettings);
                        });
                    }).insert(SettingsWindow);
                }
                MenuAction::Quit => {
                    println!("Quit button clicked!");
                    app_exit.write(AppExit::Success);
                }
                MenuAction::CloseSettings => {
                    for entity in &settings_windows {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
