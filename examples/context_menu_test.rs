use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::*;

#[derive(Component)]
struct StatusLog;

#[derive(Component)]
struct TargetCard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/afacad.ttf");
    let emoji_font = asset_server.load("fonts/openmojicolor.ttf");
    commands.insert_resource(RuiDefaultFont(font_handle));

    commands.spawn(Camera2d);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
            s.flex_direction = FlexDirection::Column;
            s.row_gap = Val::Px(24.0);
        },
        |ui| {
            let mut root = ui.reborrow();

            root.reborrow().label(
                "Prueba de Context Menu (Clic Derecho en cada tarjeta)",
                |font, color| {
                    font.font_size = FontSize::Px(24.0);
                    color.0 = Color::WHITE;
                },
            );

            // Contenedor horizontal con 3 tarjetas de prueba
            root.reborrow().hbox(
                |s| {
                    s.width = Val::Auto;
                    s.height = Val::Auto;
                    s.column_gap = Val::Px(24.0);
                    s.justify_content = JustifyContent::Center;
                    s.align_items = AlignItems::Center;
                },
                |hbox| {
                    let mut hb = hbox.reborrow();

                    // --- TARJETA 1: Acciones de Edición con Iconos ---
                    let mut card1 = hb.reborrow();
                    let mut c1_cmds = card1.vbox(
                        |s| {
                            s.width = Val::Px(220.0);
                            s.height = Val::Px(140.0);
                            s.padding = UiRect::all(Val::Px(16.0));
                            s.border = UiRect::all(Val::Px(2.0));
                            s.border_radius = BorderRadius::all(Val::Px(8.0));
                            s.justify_content = JustifyContent::Center;
                            s.align_items = AlignItems::Center;
                            s.row_gap = Val::Px(8.0);
                        },
                        |c| {
                            let mut cb = c.reborrow();
                            cb.reborrow().label("Tarjeta 1: Edición", |f, _| {
                                f.font_size = FontSize::Px(18.0);
                            });
                            cb.reborrow().label("(Haz clic derecho aquí)", |f, col| {
                                f.font_size = FontSize::Px(14.0);
                                col.0 = Color::srgb(0.7, 0.7, 0.7);
                            });

                            let emoji_copy = emoji_font.clone();
                            let emoji_cut = emoji_font.clone();
                            let emoji_paste = emoji_font.clone();
                            let emoji_del = emoji_font.clone();

                            cb.reborrow().context_menu(|menu| {
                                let mut m = menu.reborrow();

                                m.reborrow()
                                    .context_menu_item(
                                        "Copiar",
                                        Some(RuiIcon::Emoji("📋".to_string(), Some(emoji_copy))),
                                        |_| {},
                                    )
                                    .observe(log_action("Tarjeta 1 -> Copiar"));

                                m.reborrow()
                                    .context_menu_item(
                                        "Cortar",
                                        Some(RuiIcon::Emoji("✂️".to_string(), Some(emoji_cut))),
                                        |_| {},
                                    )
                                    .observe(log_action("Tarjeta 1 -> Cortar"));

                                m.reborrow()
                                    .context_menu_item(
                                        "Pegar",
                                        Some(RuiIcon::Emoji("📌".to_string(), Some(emoji_paste))),
                                        |_| {},
                                    )
                                    .observe(log_action("Tarjeta 1 -> Pegar"));

                                m.reborrow()
                                    .context_menu_item(
                                        "Eliminar",
                                        Some(RuiIcon::Emoji("🗑️".to_string(), Some(emoji_del))),
                                        |_| {},
                                    )
                                    .observe(log_action("Tarjeta 1 -> Eliminar"));
                            });
                        },
                    );
                    c1_cmds.insert((
                        BackgroundColor(Color::srgb(0.18, 0.22, 0.28)),
                        BorderColor::from(Color::srgb(0.35, 0.45, 0.6)),
                    ));

                    // --- TARJETA 2: Cambiar color de la tarjeta ---
                    let mut card2 = hb.reborrow();
                    let mut card2_entity = Entity::PLACEHOLDER;
                    let mut c2_cmds = card2.vbox(
                        |s| {
                            s.width = Val::Px(220.0);
                            s.height = Val::Px(140.0);
                            s.padding = UiRect::all(Val::Px(16.0));
                            s.border = UiRect::all(Val::Px(2.0));
                            s.border_radius = BorderRadius::all(Val::Px(8.0));
                            s.justify_content = JustifyContent::Center;
                            s.align_items = AlignItems::Center;
                            s.row_gap = Val::Px(8.0);
                        },
                        |c| {
                            card2_entity = c.target_entity();
                            let mut cb = c.reborrow();
                            cb.reborrow().label("Tarjeta 2: Color", |f, _| {
                                f.font_size = FontSize::Px(18.0);
                            });
                            cb.reborrow().label("(Cambia mi color)", |f, col| {
                                f.font_size = FontSize::Px(14.0);
                                col.0 = Color::srgb(0.8, 0.8, 0.8);
                            });

                            cb.reborrow().context_menu(|menu| {
                                let mut m = menu.reborrow();

                                m.reborrow()
                                    .context_menu_item("Pintar Rojo", None, |_| {})
                                    .observe(change_card_color(
                                        card2_entity,
                                        Color::srgb(0.45, 0.15, 0.15),
                                        "Tarjeta 2 -> Color Rojo",
                                    ));

                                m.reborrow()
                                    .context_menu_item("Pintar Verde", None, |_| {})
                                    .observe(change_card_color(
                                        card2_entity,
                                        Color::srgb(0.15, 0.40, 0.20),
                                        "Tarjeta 2 -> Color Verde",
                                    ));

                                m.reborrow()
                                    .context_menu_item("Pintar Morado", None, |_| {})
                                    .observe(change_card_color(
                                        card2_entity,
                                        Color::srgb(0.32, 0.18, 0.42),
                                        "Tarjeta 2 -> Color Morado",
                                    ));
                            });
                        },
                    );
                    c2_cmds.insert((
                        TargetCard,
                        BackgroundColor(Color::srgb(0.22, 0.22, 0.25)),
                        BorderColor::from(Color::srgb(0.45, 0.45, 0.5)),
                    ));

                    // --- TARJETA 3: Lista larga para probar el Scrollview del Context Menu ---
                    let mut card3 = hb.reborrow();
                    let mut c3_cmds = card3.vbox(
                        |s| {
                            s.width = Val::Px(220.0);
                            s.height = Val::Px(140.0);
                            s.padding = UiRect::all(Val::Px(16.0));
                            s.border = UiRect::all(Val::Px(2.0));
                            s.border_radius = BorderRadius::all(Val::Px(8.0));
                            s.justify_content = JustifyContent::Center;
                            s.align_items = AlignItems::Center;
                            s.row_gap = Val::Px(8.0);
                        },
                        |c| {
                            let mut cb = c.reborrow();
                            cb.reborrow().label("Tarjeta 3: Menú Largo", |f, _| {
                                f.font_size = FontSize::Px(18.0);
                            });
                            cb.reborrow().label("(20 opciones / Scroll)", |f, col| {
                                f.font_size = FontSize::Px(14.0);
                                col.0 = Color::srgb(0.7, 0.7, 0.7);
                            });

                            cb.reborrow().context_menu(|menu| {
                                let mut m = menu.reborrow();
                                for i in 1..=20 {
                                    let label = format!("Opción #{i}");
                                    let log_msg = format!("Tarjeta 3 -> Seleccionada Opción #{i}");
                                    m.reborrow()
                                        .context_menu_item(&label, None, |_| {})
                                        .observe(move |_: On<Activate>, mut q_log: Query<&mut Text, With<StatusLog>>| {
                                            println!("{log_msg}");
                                            if let Ok(mut txt) = q_log.single_mut() {
                                                txt.0 = format!("Última acción: {log_msg}");
                                            }
                                        });
                                }
                            });
                        },
                    );
                    c3_cmds.insert((
                        BackgroundColor(Color::srgb(0.25, 0.20, 0.16)),
                        BorderColor::from(Color::srgb(0.55, 0.42, 0.30)),
                    ));
                },
            );

            // Etiqueta inferior donde se muestra la acción ejecutada
            root.reborrow()
                .label("Última acción: (ninguna todavía)", |font, color| {
                    font.font_size = FontSize::Px(18.0);
                    color.0 = Color::srgb(0.4, 0.95, 0.5);
                })
                .insert(StatusLog);
        },
    );
}

fn log_action(
    msg: &'static str,
) -> impl Fn(On<Activate>, Query<&mut Text, With<StatusLog>>) {
    move |_: On<Activate>, mut q_log: Query<&mut Text, With<StatusLog>>| {
        println!("{msg}");
        if let Ok(mut txt) = q_log.single_mut() {
            txt.0 = format!("Última acción: {msg}");
        }
    }
}

fn change_card_color(
    card_entity: Entity,
    new_color: Color,
    msg: &'static str,
) -> impl Fn(On<Activate>, Query<&mut BackgroundColor, With<TargetCard>>, Query<&mut Text, With<StatusLog>>) {
    move |_: On<Activate>,
          mut q_bg: Query<&mut BackgroundColor, With<TargetCard>>,
          mut q_log: Query<&mut Text, With<StatusLog>>| {
        println!("{msg}");
        if let Ok(mut bg) = q_bg.get_mut(card_entity) {
            bg.0 = new_color;
        }
        if let Ok(mut txt) = q_log.single_mut() {
            txt.0 = format!("Última acción: {msg}");
        }
    }
}
