use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct WorldActor {
    speed: f32,
    angle: f32,
}

#[derive(Component)]
struct WorldOrb {
    phase: f32,
    base_y: f32,
}

#[derive(Component)]
struct HudHealthBar {
    value: f32,
}

#[derive(Component)]
struct HudStatusText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_canvas_layer (HUD 9-Slice sobre Juego)"
                            .to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_world_scene)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut theme: ResMut<RuiTheme>,
) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.insert_resource(ClearColor(Color::srgb(0.12, 0.18, 0.28)));
    apply_game_theme(&asset_server, &mut theme);

    // 1. CÁMARA DEL MUNDO DEL JUEGO (order: 0, se dibuja en el fondo)
    let world_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 0,
                ..default()
            },
            Msaa::Off,
        ))
        .id();

    // 2. CÁMARA EXCLUSIVA DEL HUD (order: 1, transparente con ClearColorConfig::None)
    let ui_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Msaa::Off,
            UiPickingCamera,
        ))
        .id();

    // =========================================================================
    // ESCENA DEL JUEGO (Renderizada en `world_camera`, order: 0)
    // =========================================================================
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            UiTargetCamera(world_camera),
            Pickable::IGNORE,
        ))
        .with_children(|world| {
            // Monedas / Orbes dorados flotando en el escenario del juego
            for (i, (offset_x, base_y)) in [
                (-230.0, -65.0),
                (230.0, -45.0),
                (-175.0, 85.0),
                (185.0, 75.0),
            ]
            .into_iter()
            .enumerate()
            {
                world.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        border_radius: BorderRadius::all(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.82, 0.12)),
                    BorderColor::from(Color::srgb(1.0, 0.96, 0.65)),
                    UiTransform::from_translation(Val2::px(offset_x, base_y)),
                    WorldOrb {
                        phase: i as f32 * 1.5,
                        base_y,
                    },
                    Pickable::IGNORE,
                ));
            }

            // Entidad interactiva en el centro del mundo (Cámara 0)
            let mut w = world.reborrow();
            w.vbox(
                |s| {
                    s.width = Val::Px(270.0);
                    s.height = Val::Px(160.0);
                    s.padding = UiRect::all(Val::Px(16.0));
                    s.justify_content = JustifyContent::Center;
                    s.align_items = AlignItems::Center;
                    s.row_gap = Val::Px(10.0);
                },
                |actor| {
                    let mut a = actor.reborrow();
                    a.reborrow().label(" MUNDO DEL JUEGO (Cámara 0)", |f, col| {
                        f.font_size = FontSize::Px(18.0);
                        col.0 = Color::srgb(1.0, 0.95, 0.45);
                    });
                    a.reborrow().label("Cofre/Objetivo moviéndose bajo el HUD", |f, col| {
                        f.font_size = FontSize::Px(14.0);
                        col.0 = Color::WHITE;
                    });
                    a.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Px(200.0);
                                s.height = Val::Px(40.0);
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Recolectar Botín", |f, _| f.font_size = FontSize::Px(16.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .observe(
                            |_: On<Activate>, mut q_txt: Query<&mut Text, With<HudStatusText>>| {
                                if let Ok(mut txt) = q_txt.single_mut() {
                                    txt.0 = "¡Botín recolectado haciendo click en el Mundo (detrás del HUD)!".to_string();
                                }
                            },
                        );
                },
            )
            .insert((
                RuiThemeElement::Panel,
                ImageNode::default(),
                UiTransform::default(),
                WorldActor {
                    speed: 1.0,
                    angle: 0.0,
                },
            ));
        });

    // =========================================================================
    // CAPA HUD SUPERPUESTA (`canvas_layer` en `ui_camera`, order: 1)
    // =========================================================================
    commands.rui_root(
        |_| {},
        |ui| {
            let mut root = ui.reborrow();
            root.canvas_layer(
                ui_camera,
                |s| {
                    s.flex_direction = FlexDirection::Column;
                    s.justify_content = JustifyContent::SpaceBetween;
                    s.padding = UiRect::all(Val::Px(24.0));
                },
                |canvas| {
                    let mut c = canvas.reborrow();

                    // FILA SUPERIOR DEL HUD (Panel de Héroe + Panel de Cámaras)
                    c.reborrow().hbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.justify_content = JustifyContent::SpaceBetween;
                            s.align_items = AlignItems::FlexStart;
                        },
                        |top_row| {
                            let mut tr = top_row.reborrow();

                            // Arriba Izquierda: Estado del Héroe con textura 9-slice
                            tr.reborrow()
                                .vbox(
                                    |s| {
                                        s.width = Val::Px(290.0);
                                        s.height = Val::Auto;
                                        s.padding = UiRect::all(Val::Px(14.0));
                                        s.row_gap = Val::Px(8.0);
                                    },
                                    |hero_hud| {
                                        let mut hh = hero_hud.reborrow();
                                        hh.reborrow().label("PALADÍN LVL 15", |f, col| {
                                            f.font_size = FontSize::Px(19.0);
                                            col.0 = Color::srgb(1.0, 0.95, 0.45);
                                        });
                                        hh.reborrow().label("Puntos de Vida (HP)", |f, col| {
                                            f.font_size = FontSize::Px(15.0);
                                            col.0 = Color::WHITE;
                                        });
                                        hh.reborrow()
                                            .progress_bar(0.0, 100.0, 75.0, |s| {
                                                s.width = Val::Percent(100.0);
                                                s.height = Val::Px(20.0);
                                            })
                                            .insert(HudHealthBar { value: 75.0 });
                                    },
                                )
                                .insert((RuiThemeElement::Panel, ImageNode::default()));

                            // Arriba Derecha: Indicador de Cámaras del Canvas Layer
                            tr.reborrow()
                                .vbox(
                                    |s| {
                                        s.width = Val::Px(320.0);
                                        s.height = Val::Auto;
                                        s.padding = UiRect::all(Val::Px(14.0));
                                        s.row_gap = Val::Px(6.0);
                                    },
                                    |cam_info| {
                                        let mut ci = cam_info.reborrow();
                                        ci.reborrow().label("HUD: canvas_layer (9-Slice)", |f, col| {
                                            f.font_size = FontSize::Px(18.0);
                                            col.0 = Color::srgb(1.0, 0.95, 0.45);
                                        });
                                        ci.reborrow().label("• Cámara 0: Mundo del juego animado", |f, col| {
                                            f.font_size = FontSize::Px(15.0);
                                            col.0 = Color::WHITE;
                                        });
                                        ci.reborrow().label("• Cámara 1: Capa HUD superpuesta", |f, col| {
                                            f.font_size = FontSize::Px(15.0);
                                            col.0 = Color::WHITE;
                                        });
                                    },
                                )
                                .insert((RuiThemeElement::Panel, ImageNode::default()));
                        },
                    );

                    // BARRA INFERIOR DEL HUD (Hotbar de Habilidades del Juego)
                    c.reborrow().vbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.align_items = AlignItems::Center;
                            s.row_gap = Val::Px(10.0);
                        },
                        |bottom_hud| {
                            let mut bh = bottom_hud.reborrow();

                            bh.reborrow()
                                .label(
                                    "¡Usa la barra de habilidades del HUD o haz click en el cofre del mundo!",
                                    |f, col| {
                                        f.font_size = FontSize::Px(17.0);
                                        col.0 = Color::srgb(1.0, 0.95, 0.45);
                                    },
                                )
                                .insert(HudStatusText);

                            bh.reborrow()
                                .hbox(
                                    |s| {
                                        s.width = Val::Auto;
                                        s.height = Val::Auto;
                                        s.padding = UiRect::all(Val::Px(12.0));
                                        s.column_gap = Val::Px(12.0);
                                    },
                                    |hotbar| {
                                        let mut hb = hotbar.reborrow();

                                        hb.reborrow()
                                            .button(
                                                |s| {
                                                    s.width = Val::Px(180.0);
                                                    s.height = Val::Px(44.0);
                                                },
                                                |btn| {
                                                    btn.reborrow()
                                                        .label("⚡ Habilidad Turbo", |f, _| f.font_size = FontSize::Px(17.0))
                                                        .insert(RuiThemeElement::ButtonText);
                                                },
                                            )
                                            .observe(
                                                |_: On<Activate>,
                                                 mut q_actor: Query<&mut WorldActor>,
                                                 mut q_txt: Query<&mut Text, With<HudStatusText>>| {
                                                    if let Ok(mut actor) = q_actor.single_mut() {
                                                        actor.speed = if actor.speed > 2.0 { 1.0 } else { 3.0 };
                                                        if let Ok(mut txt) = q_txt.single_mut() {
                                                            txt.0 = format!(
                                                                "¡Habilidad activada! Velocidad del mundo: {:.1}x",
                                                                actor.speed
                                                            );
                                                        }
                                                    }
                                                },
                                            );

                                        hb.reborrow()
                                            .button(
                                                |s| {
                                                    s.width = Val::Px(180.0);
                                                    s.height = Val::Px(44.0);
                                                },
                                                |btn| {
                                                    btn.reborrow()
                                                        .label("Poción +20 HP", |f, _| f.font_size = FontSize::Px(17.0))
                                                        .insert(RuiThemeElement::ButtonText);
                                                },
                                            )
                                            .observe(
                                                |_: On<Activate>,
                                                 mut q_bar: Query<(&mut HudHealthBar, &Children)>,
                                                 mut q_node: Query<&mut Node>,
                                                 mut q_txt: Query<&mut Text, With<HudStatusText>>| {
                                                    if let Ok((mut bar, children)) = q_bar.single_mut() {
                                                        bar.value = (bar.value + 20.0).min(100.0);
                                                        if let Some(&fill) = children.first() {
                                                            if let Ok(mut node) = q_node.get_mut(fill) {
                                                                node.width = Val::Percent(bar.value);
                                                            }
                                                        }
                                                        if let Ok(mut txt) = q_txt.single_mut() {
                                                            txt.0 = format!(
                                                                "¡Poción usada desde el HUD! HP: {:.0}%",
                                                                bar.value
                                                            );
                                                        }
                                                    }
                                                },
                                            );

                                        hb.reborrow()
                                            .button(
                                                |s| {
                                                    s.width = Val::Px(180.0);
                                                    s.height = Val::Px(44.0);
                                                },
                                                |btn| {
                                                    btn.reborrow()
                                                        .label("Recibir Golpe -25", |f, _| f.font_size = FontSize::Px(17.0))
                                                        .insert(RuiThemeElement::ButtonText);
                                                },
                                            )
                                            .observe(
                                                |_: On<Activate>,
                                                 mut q_bar: Query<(&mut HudHealthBar, &Children)>,
                                                 mut q_node: Query<&mut Node>,
                                                 mut q_txt: Query<&mut Text, With<HudStatusText>>| {
                                                    if let Ok((mut bar, children)) = q_bar.single_mut() {
                                                        bar.value = (bar.value - 25.0).max(0.0);
                                                        if let Some(&fill) = children.first() {
                                                            if let Ok(mut node) = q_node.get_mut(fill) {
                                                                node.width = Val::Percent(bar.value);
                                                            }
                                                        }
                                                        if let Ok(mut txt) = q_txt.single_mut() {
                                                            txt.0 = format!(
                                                                "¡Impacto recibido! HP restante: {:.0}%",
                                                                bar.value
                                                            );
                                                        }
                                                    }
                                                },
                                            );
                                    },
                                )
                                .insert((RuiThemeElement::Panel, ImageNode::default()));
                        },
                    );
                },
            );
        },
    );
}

fn animate_world_scene(
    time: Res<Time>,
    mut q_actor: Query<(&mut WorldActor, &mut UiTransform), Without<WorldOrb>>,
    mut q_orbs: Query<(&WorldOrb, &mut UiTransform), Without<WorldActor>>,
) {
    let dt = time.delta_secs();
    let elapsed = time.elapsed_secs();

    for (mut actor, mut transform) in &mut q_actor {
        actor.angle += dt * 1.4 * actor.speed;
        let x = actor.angle.cos() * 75.0;
        let y = (actor.angle * 2.0).sin() * 28.0;
        transform.translation = Val2::px(x, y);
    }

    for (orb, mut transform) in &mut q_orbs {
        let Val::Px(cur_x) = transform.translation.x else {
            continue;
        };
        let y = orb.base_y + (elapsed * 2.2 + orb.phase).sin() * 22.0;
        transform.translation = Val2::px(cur_x, y);
    }
}

fn apply_game_theme(asset_server: &AssetServer, theme: &mut RuiTheme) {
    theme.image_panel = Some(asset_server.load("theme/kenney/blue/button_square_flat.png"));
    theme.panel_margin = 10.0;
    theme.image_window_header =
        Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.window_header_margin = 10.0;
    theme.window_header_layout_margin = UiRect::all(Val::Px(5.0));
    theme.color_window_header_text = Color::srgb(0.2, 0.2, 0.2);
    theme.image_button_normal =
        Some(asset_server.load("theme/kenney/yellow/button_square_depth_gradient.png"));
    theme.image_button_hover =
        Some(asset_server.load("theme/kenney/yellow/button_square_depth_flat.png"));
    theme.image_button_pressed =
        Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.button_margin = 10.0;
    theme.color_button_text = Color::srgb(0.2, 0.2, 0.2);
    theme.image_progress_bar_track =
        Some(asset_server.load("theme/kenney/blue/button_square_depth_flat.png"));
    theme.progress_bar_track_margin = 6.0;
    theme.image_progress_bar_fill =
        Some(asset_server.load("theme/kenney/yellow/button_square_depth_gradient.png"));
    theme.progress_bar_fill_margin = 6.0;
}
