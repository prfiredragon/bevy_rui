use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_accordion".to_string(),
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
                    s.width = Val::Px(420.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Accordion", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().accordion(
                        "1. Configuración General",
                        |_| {},
                        |acc| {
                            let mut a = acc.reborrow();
                            a.reborrow().label("Idioma: Español", |f, col| {
                                f.font_size = FontSize::Px(16.0);
                                col.0 = Color::srgb(0.8, 0.8, 0.8);
                            });
                            a.reborrow().checkbox("Guardado automático", true, |s| {
                                s.margin = UiRect::top(Val::Px(6.0));
                            });
                        },
                    );

                    c.reborrow().accordion(
                        "2. Opciones de Video",
                        |_| {},
                        |acc| {
                            let mut a = acc.reborrow();
                            a.reborrow().checkbox("Sincronización Vertical (VSync)", true, |_| {});
                            a.reborrow().checkbox("Sombras Dinámicas", false, |s| {
                                s.margin = UiRect::top(Val::Px(6.0));
                            });
                        },
                    );

                    c.reborrow().accordion(
                        "3. Información del Sistema",
                        |_| {},
                        |acc| {
                            let mut a = acc.reborrow();
                            a.reborrow().label("Motor: Bevy 0.20 + bevy_rui", |f, col| {
                                f.font_size = FontSize::Px(16.0);
                                col.0 = Color::srgb(0.6, 0.85, 1.0);
                            });
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
