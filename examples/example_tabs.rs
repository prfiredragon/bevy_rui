use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_tabs".to_string(),
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
                    s.width = Val::Px(480.0);
                    s.height = Val::Px(320.0);
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Tabs", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().tabs(0, |_| {}, |tabs| {
                        tabs.tab(
                            "General",
                            false,
                            |n, _| {
                                n.padding = UiRect::all(Val::Px(16.0));
                                n.row_gap = Val::Px(10.0);
                            },
                            |content| {
                                let mut cp = content.reborrow();
                                cp.reborrow().label("Pestaña General", |f, _| {
                                    f.font_size = FontSize::Px(20.0);
                                });
                                cp.reborrow().checkbox("Habilitar notificaciones", true, |_| {});
                            },
                        );

                        tabs.tab(
                            "Audio",
                            false,
                            |n, _| {
                                n.padding = UiRect::all(Val::Px(16.0));
                                n.row_gap = Val::Px(10.0);
                            },
                            |content| {
                                let mut cp = content.reborrow();
                                cp.reborrow().label("Volumen Maestro", |f, _| {
                                    f.font_size = FontSize::Px(18.0);
                                });
                                cp.reborrow().slider(0.0, 100.0, 75.0, |_| {});
                            },
                        );

                        tabs.tab(
                            "Cerrable",
                            true,
                            |n, _| {
                                n.padding = UiRect::all(Val::Px(16.0));
                                n.justify_content = JustifyContent::Center;
                                n.align_items = AlignItems::Center;
                            },
                            |content| {
                                content.reborrow().label(
                                    "Esta pestaña tiene botón 'x' para cerrarse.",
                                    |f, col| {
                                        f.font_size = FontSize::Px(16.0);
                                        col.0 = Color::srgb(0.85, 0.85, 0.9);
                                    },
                                );
                            },
                        );
                    });
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
