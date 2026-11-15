use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_tooltip".to_string(),
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
                "Game Tooltip",
                false,
                |s| {
                    s.width = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow().label(
                        "Pasa el cursor sobre las habilidades:",
                        |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::WHITE;
                        },
                    );

                    c.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Px(44.0);
                                s.margin = UiRect::vertical(Val::Px(10.0));
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Espada de Fuego", |f, _| f.font_size = FontSize::Px(18.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .with_tooltip(
                            "Daño: 45 | Elemento: Fuego (+10% crítico)",
                            Color::srgb(0.12, 0.25, 0.38),
                            Color::srgb(1.0, 0.8, 0.0),
                        );

                    c.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Px(44.0);
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Escudo de Hielo", |f, _| f.font_size = FontSize::Px(18.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .with_tooltip(
                            "Defensa: +30 | Reduce daño mágico en 25%",
                            Color::srgb(0.12, 0.25, 0.38),
                            Color::srgb(0.4, 0.85, 1.0),
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
