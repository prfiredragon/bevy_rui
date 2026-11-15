use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_resizer".to_string(),
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
                "Game Resizer",
                false,
                |s| {
                    s.width = Val::Px(560.0);
                    s.height = Val::Px(340.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow().hbox(
                        |s| {
                            s.flex_grow = 1.0;
                        },
                        |split_h| {
                            let mut sh = split_h.reborrow();

                            sh.reborrow()
                                .vbox(
                                    |s| {
                                        s.width = Val::Percent(45.0);
                                        s.justify_content = JustifyContent::Center;
                                        s.align_items = AlignItems::Center;
                                    },
                                    |left| {
                                        left.reborrow().label("Equipo", |f, _| {
                                            f.font_size = FontSize::Px(18.0);
                                        });
                                    },
                                )
                                .insert((
                                    ImageNode::default(),
                                    BackgroundColor(Color::NONE),
                                    RuiThemeElement::TextboxBg,
                                ));

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
                                                top.reborrow().label("Estadísticas", |f, _| {
                                                    f.font_size = FontSize::Px(18.0);
                                                });
                                            },
                                        )
                                        .insert((
                                            ImageNode::default(),
                                            BackgroundColor(Color::NONE),
                                            RuiThemeElement::TextboxBg,
                                        ));

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
                                                bot.reborrow().label("Habilidades", |f, _| {
                                                    f.font_size = FontSize::Px(18.0);
                                                });
                                            },
                                        )
                                        .insert((
                                            ImageNode::default(),
                                            BackgroundColor(Color::NONE),
                                            RuiThemeElement::TextboxBg,
                                        ));
                                },
                            );
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
    theme.image_textbox_bg = Some(asset_server.load("theme/kenney/blue/textbox_bg.png"));
    theme.textbox_margin = 8.0;
}
