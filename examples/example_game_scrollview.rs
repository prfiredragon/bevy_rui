use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_scrollview".to_string(),
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
                "Game ScrollView (Tienda)",
                false,
                |s| {
                    s.width = Val::Px(380.0);
                    s.height = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow()
                        .scrollview(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.flex_grow = 1.0;
                                s.padding = UiRect::all(Val::Px(10.0));
                            },
                            |scroll| {
                                let mut sc = scroll.reborrow();
                                for i in 1..=20 {
                                    sc.reborrow().button(
                                        |s| {
                                            s.width = Val::Px(304.0);
                                            s.height = Val::Px(40.0);
                                            s.margin = UiRect::bottom(Val::Px(6.0));
                                        },
                                        |btn| {
                                            btn.reborrow()
                                                .label(
                                                    &format!("Poción de Energía #{i}"),
                                                    |f, _| f.font_size = FontSize::Px(16.0),
                                                )
                                                .insert(RuiThemeElement::ButtonText);
                                        },
                                    );
                                }
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
