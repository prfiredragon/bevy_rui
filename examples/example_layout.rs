use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_layout".to_string(),
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
                    s.width = Val::Px(520.0);
                    s.height = Val::Px(340.0);
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(14.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |vbox| {
                    let mut vb = vbox.reborrow();

                    vb.reborrow().label("Widget: Layout (vbox + hbox)", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    vb.reborrow().hbox(
                        |s| {
                            s.flex_grow = 1.0;
                            s.column_gap = Val::Px(12.0);
                        },
                        |hbox| {
                            let mut hb = hbox.reborrow();

                            hb.reborrow()
                                .vbox(
                                    |s| {
                                        s.flex_grow = 1.0;
                                        s.justify_content = JustifyContent::Center;
                                        s.align_items = AlignItems::Center;
                                        s.border_radius = BorderRadius::all(Val::Px(6.0));
                                    },
                                    |col1| {
                                        col1.reborrow().label("Columna 1 (vbox)", |f, _| {
                                            f.font_size = FontSize::Px(18.0);
                                        });
                                    },
                                )
                                .insert(BackgroundColor(Color::srgb(0.22, 0.32, 0.48)));

                            hb.reborrow()
                                .vbox(
                                    |s| {
                                        s.flex_grow = 1.0;
                                        s.justify_content = JustifyContent::Center;
                                        s.align_items = AlignItems::Center;
                                        s.border_radius = BorderRadius::all(Val::Px(6.0));
                                    },
                                    |col2| {
                                        col2.reborrow().label("Columna 2 (vbox)", |f, _| {
                                            f.font_size = FontSize::Px(18.0);
                                        });
                                    },
                                )
                                .insert(BackgroundColor(Color::srgb(0.22, 0.42, 0.34)));
                        },
                    );

                    vb.reborrow()
                        .hbox(
                            |s| {
                                s.height = Val::Px(50.0);
                                s.justify_content = JustifyContent::Center;
                                s.align_items = AlignItems::Center;
                                s.border_radius = BorderRadius::all(Val::Px(6.0));
                            },
                            |footer| {
                                footer.reborrow().label("Fila Inferior (hbox)", |f, _| {
                                    f.font_size = FontSize::Px(18.0);
                                });
                            },
                        )
                        .insert(BackgroundColor(Color::srgb(0.35, 0.25, 0.42)));
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
