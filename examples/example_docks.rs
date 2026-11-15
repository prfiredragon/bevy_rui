use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_docks".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.spawn(Camera2d);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.vbox(
                |s| {
                    s.width = Val::Px(720.0);
                    s.height = Val::Px(440.0);
                    s.padding = UiRect::all(Val::Px(16.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Docks (Paneles Divisibles)", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().dock_split_horizontal(
                        Val::Percent(32.0),
                        100.0,
                        |_| {},
                        |left_pane| {
                            left_pane.reborrow().dock_panel(0, |_| {}, |tabs| {
                                tabs.tab(
                                    "Jerarquía",
                                    false,
                                    |n, _| n.padding = UiRect::all(Val::Px(12.0)),
                                    |content| {
                                        let mut cp = content.reborrow();
                                        cp.reborrow().label("• Escena Principal", |_, _| {});
                                        cp.reborrow().label("  • Cámara", |_, _| {});
                                        cp.reborrow().label("  • Jugador", |_, _| {});
                                    },
                                );
                                tabs.tab(
                                    "Archivos",
                                    true,
                                    |n, _| n.padding = UiRect::all(Val::Px(12.0)),
                                    |content| {
                                        content.reborrow().label("assets/fonts/afacad.ttf", |_, _| {});
                                    },
                                );
                            });
                        },
                        |right_pane| {
                            right_pane.reborrow().dock_split_vertical(
                                Val::Percent(62.0),
                                80.0,
                                |_| {},
                                |top_pane| {
                                    top_pane.reborrow().dock_panel(0, |_| {}, |tabs| {
                                        tabs.tab(
                                            "Vista Central",
                                            false,
                                            |n, _| {
                                                n.justify_content = JustifyContent::Center;
                                                n.align_items = AlignItems::Center;
                                            },
                                            |content| {
                                                content.reborrow().label(
                                                    "Arrastra las barras divisorias para redimensionar",
                                                    |f, col| {
                                                        f.font_size = FontSize::Px(18.0);
                                                        col.0 = Color::srgb(0.6, 0.85, 1.0);
                                                    },
                                                );
                                            },
                                        );
                                    });
                                },
                                |bottom_pane| {
                                    bottom_pane.reborrow().dock_panel(0, |_| {}, |tabs| {
                                        tabs.tab(
                                            "Consola",
                                            false,
                                            |n, _| n.padding = UiRect::all(Val::Px(12.0)),
                                            |content| {
                                                content.reborrow().label(
                                                    "[INFO] Panel de Docks inicializado correctamente.",
                                                    |f, col| {
                                                        f.font_size = FontSize::Px(15.0);
                                                        col.0 = Color::srgb(0.5, 0.9, 0.6);
                                                    },
                                                );
                                            },
                                        );
                                    });
                                },
                            );
                        },
                    );
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
