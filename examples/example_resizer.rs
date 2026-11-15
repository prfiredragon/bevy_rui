use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_resizer".to_string(),
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
                    s.width = Val::Px(600.0);
                    s.height = Val::Px(360.0);
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Resizer (Horizontal y Vertical)", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().hbox(
                        |s| {
                            s.flex_grow = 1.0;
                            s.border = UiRect::all(Val::Px(1.0));
                        },
                        |split_h| {
                            let mut sh = split_h.reborrow();

                            sh.reborrow()
                                .vbox(
                                    |s| {
                                        s.width = Val::Percent(40.0);
                                        s.justify_content = JustifyContent::Center;
                                        s.align_items = AlignItems::Center;
                                    },
                                    |left| {
                                        left.reborrow().label("Panel Izquierdo", |f, _| {
                                            f.font_size = FontSize::Px(18.0);
                                        });
                                    },
                                )
                                .insert(BackgroundColor(Color::srgb(0.2, 0.28, 0.38)));

                            sh.reborrow().resizer(RuiResizerDir::Horizontal, 80.0, |_| {});

                            sh.reborrow().vbox(
                                |s| {
                                    s.flex_grow = 1.0;
                                    s.width = Val::Auto;
                                },
                                |right| {
                                    let mut r = right.reborrow();

                                    r.reborrow()
                                        .vbox(
                                            |s| {
                                                s.height = Val::Percent(50.0);
                                                s.justify_content = JustifyContent::Center;
                                                s.align_items = AlignItems::Center;
                                            },
                                            |top| {
                                                top.reborrow().label("Panel Superior", |f, _| {
                                                    f.font_size = FontSize::Px(18.0);
                                                });
                                            },
                                        )
                                        .insert(BackgroundColor(Color::srgb(0.22, 0.36, 0.28)));

                                    r.reborrow().resizer(RuiResizerDir::Vertical, 60.0, |_| {});

                                    r.reborrow()
                                        .vbox(
                                            |s| {
                                                s.flex_grow = 1.0;
                                                s.height = Val::Auto;
                                                s.justify_content = JustifyContent::Center;
                                                s.align_items = AlignItems::Center;
                                            },
                                            |bot| {
                                                bot.reborrow().label("Panel Inferior", |f, _| {
                                                    f.font_size = FontSize::Px(18.0);
                                                });
                                            },
                                        )
                                        .insert(BackgroundColor(Color::srgb(0.34, 0.24, 0.38)));
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
