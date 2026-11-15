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
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_canvas_layer (HUD sobre Cámara del Mundo)"
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.insert_resource(ClearColor(Color::srgb(0.09, 0.13, 0.20)));

    // 1. CÁMARA DEL MUNDO (order: 0, dibuja el escenario del juego en el fondo)
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
    // ESCENA DEL MUNDO (Renderizada en `world_camera`, order: 0)
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
            // Orbes animados de fondo en el mundo
            for (i, (offset_x, base_y, col)) in [
                (-220.0, -60.0, Color::srgb(0.2, 0.75, 0.95)),
                (220.0, -40.0, Color::srgb(0.95, 0.65, 0.2)),
                (-160.0, 90.0, Color::srgb(0.4, 0.9, 0.55)),
                (180.0, 80.0, Color::srgb(0.85, 0.4, 0.95)),
            ]
            .into_iter()
            .enumerate()
            {
                world.spawn((
                    Node {
                        width: Val::Px(28.0),
                        height: Val::Px(28.0),
                        border_radius: BorderRadius::all(Val::Px(14.0)),
                        ..default()
                    },
                    BackgroundColor(col),
                    UiTransform::from_translation(Val2::px(offset_x, base_y)),
                    WorldOrb {
                        phase: i as f32 * 1.4,
                        base_y,
                    },
                    Pickable::IGNORE,
                ));
            }

            // Objeto central interactivo del mundo (en Cámara 0)
            let mut w = world.reborrow();
            w.vbox(
                |s| {
                    s.width = Val::Px(250.0);
                    s.height = Val::Px(150.0);
                    s.padding = UiRect::all(Val::Px(16.0));
                    s.justify_content = JustifyContent::Center;
                    s.align_items = AlignItems::Center;
                    s.row_gap = Val::Px(10.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(14.0));
                },
                |actor| {
                    let mut a = actor.reborrow();
                    a.reborrow().label(" ESCENA DEL MUNDO (Cámara 0)", |f, col| {
                        f.font_size = FontSize::Px(16.0);
                        col.0 = Color::srgb(0.55, 0.88, 1.0);
                    });
                    a.reborrow().label("Entidad en movimiento detrás del HUD", |f, col| {
                        f.font_size = FontSize::Px(13.0);
                        col.0 = Color::srgb(0.78, 0.83, 0.90);
                    });
                    a.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Px(190.0);
                                s.height = Val::Px(36.0);
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Click en el Mundo", |f, _| f.font_size = FontSize::Px(14.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .observe(
                            |_: On<Activate>, mut q_txt: Query<&mut Text, With<HudStatusText>>| {
                                if let Ok(mut txt) = q_txt.single_mut() {
                                    txt.0 = "¡Hiciste click directamente en el Mundo (detrás del canvas_layer)!".to_string();
                                }
                            },
                        );
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.14, 0.22, 0.34)),
                BorderColor::from(Color::srgb(0.35, 0.70, 0.95)),
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

                    // BARRA SUPERIOR DEL HUD (Arriba Izquierda + Arriba Derecha)
                    c.reborrow().hbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.justify_content = JustifyContent::SpaceBetween;
                            s.align_items = AlignItems::FlexStart;
                        },
                        |top_bar| {
                            let mut tb = top_bar.reborrow();

                            // Panel Superior Izquierdo: Estado del Jugador + Barra de Salud
                            tb.reborrow()
                                .vbox(
                                    |s| {
                                        s.width = Val::Px(290.0);
                                        s.height = Val::Auto;
                                        s.padding = UiRect::all(Val::Px(14.0));
                                        s.row_gap = Val::Px(8.0);
                                        s.border = UiRect::all(Val::Px(1.0));
                                        s.border_radius = BorderRadius::all(Val::Px(8.0));
                                    },
                                    |stats| {
                                        let mut st = stats.reborrow();
                                        st.reborrow().label("HUD — JUGADOR 01", |f, col| {
                                            f.font_size = FontSize::Px(18.0);
                                            col.0 = Color::WHITE;
                                        });
                                        st.reborrow().label("Integridad del Escudo (HP)", |f, col| {
                                            f.font_size = FontSize::Px(14.0);
                                            col.0 = Color::srgb(0.75, 0.8, 0.9);
                                        });
                                        st.reborrow()
                                            .progress_bar(0.0, 100.0, 70.0, |s| {
                                                s.width = Val::Percent(100.0);
                                                s.height = Val::Px(16.0);
                                            })
                                            .insert(HudHealthBar { value: 70.0 });
                                    },
                                )
                                .insert((
                                    BackgroundColor(Color::srgba(0.10, 0.12, 0.16, 0.90)),
                                    BorderColor::from(Color::srgb(0.3, 0.45, 0.65)),
                                ));

                            // Panel Superior Derecho: Explicación de Cámaras
                            tb.reborrow()
                                .vbox(
                                    |s| {
                                        s.width = Val::Px(320.0);
                                        s.height = Val::Auto;
                                        s.padding = UiRect::all(Val::Px(14.0));
                                        s.row_gap = Val::Px(6.0);
                                        s.border = UiRect::all(Val::Px(1.0));
                                        s.border_radius = BorderRadius::all(Val::Px(8.0));
                                    },
                                    |info_box| {
                                        let mut ib = info_box.reborrow();
                                        ib.reborrow().label("Widget: canvas_layer", |f, col| {
                                            f.font_size = FontSize::Px(18.0);
                                            col.0 = Color::srgb(0.45, 0.85, 1.0);
                                        });
                                        ib.reborrow().label("• Cámara 0: Escena animada en el fondo", |f, col| {
                                            f.font_size = FontSize::Px(14.0);
                                            col.0 = Color::srgb(0.8, 0.85, 0.9);
                                        });
                                        ib.reborrow().label("• Cámara 1: HUD superpuesto (Pickable::IGNORE)", |f, col| {
                                            f.font_size = FontSize::Px(14.0);
                                            col.0 = Color::srgb(0.8, 0.85, 0.9);
                                        });
                                    },
                                )
                                .insert((
                                    BackgroundColor(Color::srgba(0.10, 0.12, 0.16, 0.90)),
                                    BorderColor::from(Color::srgb(0.3, 0.45, 0.65)),
                                ));
                        },
                    );

                    // BARRA INFERIOR DEL HUD (Hotbar de Acciones)
                    c.reborrow().vbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.align_items = AlignItems::Center;
                            s.row_gap = Val::Px(10.0);
                        },
                        |bottom_area| {
                            let mut ba = bottom_area.reborrow();

                            ba.reborrow()
                                .label(
                                    "El centro está despejado dejando ver e interactuar con la Cámara 0",
                                    |f, col| {
                                        f.font_size = FontSize::Px(16.0);
                                        col.0 = Color::srgb(0.45, 0.95, 0.6);
                                    },
                                )
                                .insert(HudStatusText);

                            ba.reborrow()
                                .hbox(
                                    |s| {
                                        s.width = Val::Auto;
                                        s.height = Val::Auto;
                                        s.padding = UiRect::all(Val::Px(12.0));
                                        s.column_gap = Val::Px(12.0);
                                        s.border = UiRect::all(Val::Px(1.0));
                                        s.border_radius = BorderRadius::all(Val::Px(8.0));
                                    },
                                    |hotbar| {
                                        let mut hb = hotbar.reborrow();

                                        hb.reborrow()
                                            .button(
                                                |s| {
                                                    s.width = Val::Px(170.0);
                                                    s.height = Val::Px(40.0);
                                                },
                                                |btn| {
                                                    btn.reborrow()
                                                        .label("Velocidad Turbo", |f, _| f.font_size = FontSize::Px(15.0))
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
                                                                "HUD -> Velocidad del mundo cambiada a {:.1}x",
                                                                actor.speed
                                                            );
                                                        }
                                                    }
                                                },
                                            );

                                        hb.reborrow()
                                            .button(
                                                |s| {
                                                    s.width = Val::Px(170.0);
                                                    s.height = Val::Px(40.0);
                                                },
                                                |btn| {
                                                    btn.reborrow()
                                                        .label("Recargar +20 HP", |f, _| f.font_size = FontSize::Px(15.0))
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
                                                                "HUD -> Salud recargada al {:.0}%",
                                                                bar.value
                                                            );
                                                        }
                                                    }
                                                },
                                            );

                                        hb.reborrow()
                                            .button(
                                                |s| {
                                                    s.width = Val::Px(170.0);
                                                    s.height = Val::Px(40.0);
                                                },
                                                |btn| {
                                                    btn.reborrow()
                                                        .label("Recibir Daño -25", |f, _| f.font_size = FontSize::Px(15.0))
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
                                                                "HUD -> Daño recibido, salud al {:.0}%",
                                                                bar.value
                                                            );
                                                        }
                                                    }
                                                },
                                            );
                                    },
                                )
                                .insert((
                                    BackgroundColor(Color::srgba(0.10, 0.12, 0.16, 0.90)),
                                    BorderColor::from(Color::srgb(0.3, 0.45, 0.65)),
                                ));
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
