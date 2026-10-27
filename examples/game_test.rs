use bevy::{prelude::*, ui_widgets::Activate, window::PresentMode};
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
        .add_plugins(bevy_rui::RuiPlugin)
        .insert_resource(bevy_rui::navigation::RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, animate_cube)
        .add_observer(handle_button_activations)
        .run();
}

#[derive(Component)]
struct AnimatedCube;

#[derive(Component)]
struct SettingsWindow;

#[derive(Component, Clone, Copy)]
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
    
    // Paneles generales y ventanas usan image_panel
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

    commands.spawn((
        Camera2d,
        Camera {
            order: 1, 
            clear_color: ClearColorConfig::None,
            ..default()
        },
        UiPickingCamera,
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera { order: 0, ..default() },
        MeshPickingCamera,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.3))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        AnimatedCube,
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // UI - Floating Window
    commands.rui_root(|s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
    }, |rui| {
        let mut r = rui.reborrow();
        r.window("Game Menu", false, |s| {
            s.width = Val::Px(250.0);
            s.left = Val::Auto;
            s.top = Val::Auto;
        }, |win, _| {
            let mut w = win.reborrow();
            
            let mut w_btn1 = w.reborrow();
            w_btn1.reborrow().button(|s| {
                s.width = Val::Percent(100.0);
                s.margin = UiRect::bottom(Val::Px(10.0));
            }, |b| {
                let mut btn = b.reborrow();
                btn.reborrow().label("Play", |_,_|{}).insert(crate::theme::RuiThemeElement::ButtonText);
            }).insert((MenuAction::Play, bevy::input_focus::AutoFocus));

            let mut w_btn2 = w.reborrow();
            w_btn2.reborrow().button(|s| {
                s.width = Val::Percent(100.0);
                s.margin = UiRect::bottom(Val::Px(10.0));
            }, |b| {
                let mut btn = b.reborrow();
                btn.reborrow().label("Settings", |_,_|{}).insert(crate::theme::RuiThemeElement::ButtonText);
            }).insert(MenuAction::Settings);

            let mut w_btn3 = w.reborrow();
            w_btn3.reborrow().button(|s| {
                s.width = Val::Percent(100.0);
            }, |b| {
                let mut btn = b.reborrow();
                btn.reborrow().label("Quit", |_,_|{}).insert(crate::theme::RuiThemeElement::ButtonText);
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
        transform.translation.x = -t.sin() * 2.0;
        transform.translation.y = t.cos() * 2.0;
        
        transform.rotate_x(1.0 * time.delta_secs());
        transform.rotate_y(2.0 * time.delta_secs());
    }
}

// Observer moderno usando On<Activate> y trigger.entity
fn handle_button_activations(
    trigger: On<Activate>,
    actions: Query<&MenuAction>,
    settings_windows: Query<Entity, With<SettingsWindow>>,
    mut commands: Commands,
    mut app_exit: MessageWriter<AppExit>,
) {
    if let Ok(action) = actions.get(trigger.entity) {
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
                    let mut r = rui.reborrow();
                    r.window("Settings", false, |s| {
                        s.width = Val::Px(500.0);
                        s.height = Val::Px(400.0);
                    }, |win, _| {
                        let mut w = win.reborrow();
                        w.tabs(0, |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.flex_basis = Val::Px(0.0);
                            s.flex_grow = 1.0;
                            s.flex_shrink = 1.0;
                        }, |tabs| {
                            // Pestaña General
                            tabs.tab("General", false, |_, _| {}, |content| {
                                let mut c = content.reborrow();
                                
                                let mut c_spawn1 = c.reborrow();
                                c_spawn1.spawn(Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::bottom(Val::Px(15.0)),
                                    ..default()
                                }).with_children(|row| {
                                    let mut r_bind = row.reborrow();
                                    r_bind.reborrow().label("Nombre del personaje: ", |_,_| {});
                                    let mut r_bind2 = r_bind.reborrow();
                                    r_bind2.reborrow().textbox("Ingresa nombre...", |s, _, _| {
                                        s.width = Val::Px(150.0);
                                        s.margin = UiRect::left(Val::Px(10.0));
                                    });
                                });

                                let mut c_chk = c.reborrow();
                                c_chk.reborrow().checkbox("Habilitar sonido espacial", true, |s| {
                                    s.margin = UiRect::bottom(Val::Px(15.0));
                                });

                                let mut c_dd = c.reborrow();
                                c_dd.reborrow().dropdown("Calidad Gráfica", &["Baja", "Media", "Alta", "Ultra"], |s| {
                                    s.width = Val::Px(200.0);
                                });
                            });

                            // Pestaña Audio
                            tabs.tab("Audio", false, |_, _| {}, |content| {
                                let mut c = content.reborrow();
                                
                                let mut c_spawn1 = c.reborrow();
                                c_spawn1.spawn(Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::bottom(Val::Px(15.0)),
                                    ..default()
                                }).with_children(|row| {
                                    let mut r_bind = row.reborrow();
                                    r_bind.reborrow().label("Volumen Maestro: ", |_,_| {});
                                    let mut r_bind2 = r_bind.reborrow();
                                    r_bind2.reborrow().slider(0.0, 100.0, 50.0, |s| {
                                        s.margin = UiRect::left(Val::Px(10.0));
                                    });
                                });

                                let mut c_spawn2 = c.reborrow();
                                c_spawn2.spawn(Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::bottom(Val::Px(15.0)),
                                    ..default()
                                }).with_children(|row| {
                                    let mut r_bind = row.reborrow();
                                    r_bind.reborrow().label("Efectos SFX: ", |_,_| {});
                                    let mut r_bind2 = r_bind.reborrow();
                                    r_bind2.reborrow().slider(0.0, 100.0, 75.0, |s| {
                                        s.margin = UiRect::left(Val::Px(10.0));
                                    });
                                });
                            });

                            // Pestaña Otros
                            tabs.tab("Otros", false, |_, _| {}, |content| {
                                let mut c = content.reborrow();
                                c.reborrow().accordion("Opciones Avanzadas", |s| {
                                    s.width = Val::Percent(100.0);
                                }, |acc| {
                                    let mut a = acc.reborrow();
                                    let mut a1 = a.reborrow();
                                    a1.reborrow().checkbox("Modo desarrollador", false, |s| {
                                        s.margin = UiRect::bottom(Val::Px(10.0));
                                    });
                                    let mut a2 = a.reborrow();
                                    a2.reborrow().checkbox("Mostrar FPS", true, |_| {});
                                });
                            });
                        });

                        let mut w_btn = win.reborrow();
                        w_btn.reborrow().button(|s| {
                            s.width = Val::Px(120.0);
                            s.margin = UiRect::top(Val::Px(15.0));
                            s.align_self = AlignSelf::Center;
                        }, |b| {
                            let mut btn = b.reborrow();
                            btn.reborrow().label("Back", |_,_| {}).insert(crate::theme::RuiThemeElement::ButtonText);
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
