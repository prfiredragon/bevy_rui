use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_accordion".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut theme: ResMut<RuiTheme>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    apply_game_theme(&asset_server, &mut theme);

    commands.spawn(Camera2d);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.window(
                "Game Accordion",
                false,
                |s| {
                    s.width = Val::Px(420.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow().accordion(
                        "1. Ajustes de Partida",
                        |s| s.margin = UiRect::bottom(Val::Px(8.0)),
                        |acc| {
                            let mut a = acc.reborrow();
                            a.reborrow().label("Dificultad: Normal", |f, col| {
                                f.font_size = FontSize::Px(16.0);
                                col.0 = Color::WHITE;
                            });
                            a.reborrow().checkbox("Autoguardado", true, |s| {
                                s.margin = UiRect::top(Val::Px(6.0));
                            });
                        },
                    );

                    c.reborrow().accordion(
                        "2. Gráficos del Juego",
                        |s| s.margin = UiRect::bottom(Val::Px(8.0)),
                        |acc| {
                            let mut a = acc.reborrow();
                            a.reborrow().checkbox("Sincronización Vertical (VSync)", true, |_| {});
                            a.reborrow().checkbox("Efectos de Partículas", true, |s| {
                                s.margin = UiRect::top(Val::Px(6.0));
                            });
                        },
                    );

                    c.reborrow().accordion(
                        "3. Créditos",
                        |_| {},
                        |acc| {
                            acc.reborrow().label("Motor: Bevy 0.20 + bevy_rui (Ninepatch)", |f, col| {
                                f.font_size = FontSize::Px(16.0);
                                col.0 = Color::srgb(1.0, 0.9, 0.4);
                            });
                        },
                    );
                },
            );
        },
    );
}

fn apply_game_theme(asset_server: &AssetServer, theme: &mut RuiTheme) {
    theme.image_panel = Some(asset_server.load("theme/kenney/blue/button_square_flat.png"));
    theme.panel_margin = 10.0;
    theme.image_window_header = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.window_header_margin = 10.0;
    theme.window_header_layout_margin = UiRect::all(Val::Px(5.0));
    theme.color_window_header_text = Color::srgb(0.2, 0.2, 0.2);
    theme.image_button_normal = Some(asset_server.load("theme/kenney/yellow/button_square_depth_gradient.png"));
    theme.image_button_hover = Some(asset_server.load("theme/kenney/yellow/button_square_depth_flat.png"));
    theme.image_button_pressed = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.button_margin = 10.0;
    theme.color_button_text = Color::srgb(0.2, 0.2, 0.2);
}
