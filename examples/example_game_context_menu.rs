use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct ContextStatusText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_context_menu".to_string(),
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
    let emoji_font = asset_server.load("fonts/openmojicolor.ttf");
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
                "Game Context Menu",
                false,
                |s| {
                    s.width = Val::Px(380.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow()
                        .vbox(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Px(130.0);
                                s.justify_content = JustifyContent::Center;
                                s.align_items = AlignItems::Center;
                                s.row_gap = Val::Px(6.0);
                                s.margin = UiRect::bottom(Val::Px(12.0));
                            },
                            |area| {
                                let mut a = area.reborrow();
                                a.reborrow().label("Ranura de Inventario", |f, col| {
                                    f.font_size = FontSize::Px(20.0);
                                    col.0 = Color::WHITE;
                                });
                                a.reborrow().label("(Haz Clic Derecho aquí)", |f, col| {
                                    f.font_size = FontSize::Px(14.0);
                                    col.0 = Color::srgb(0.85, 0.9, 1.0);
                                });

                                let f1 = emoji_font.clone();
                                let f2 = emoji_font.clone();
                                let f3 = emoji_font.clone();
                                a.reborrow().context_menu(|menu| {
                                    let mut m = menu.reborrow();
                                    m.reborrow()
                                        .context_menu_item(
                                            "Equipar Objeto",
                                            Some(RuiIcon::Emoji("⚔️".to_string(), Some(f1))),
                                            |_| {},
                                        )
                                        .observe(set_status("Acción: Equipar Objeto"));

                                    m.reborrow()
                                        .context_menu_item(
                                            "Inspeccionar",
                                            Some(RuiIcon::Emoji("🔍".to_string(), Some(f2))),
                                            |_| {},
                                        )
                                        .observe(set_status("Acción: Inspeccionar"));

                                    m.reborrow()
                                        .context_menu_item(
                                            "Descartar",
                                            Some(RuiIcon::Emoji("🗑️".to_string(), Some(f3))),
                                            |_| {},
                                        )
                                        .observe(set_status("Acción: Descartar"));
                                });
                            },
                        )
                        .insert((
                            ImageNode::default(),
                            BackgroundColor(Color::NONE),
                            RuiThemeElement::TextboxBg,
                        ));

                    c.reborrow()
                        .label("Acción: (ninguna)", |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::srgb(1.0, 0.95, 0.45);
                        })
                        .insert(ContextStatusText);
                },
            );
        },
    );
}

fn set_status(
    msg: &'static str,
) -> impl Fn(On<Activate>, Query<&mut Text, With<ContextStatusText>>) {
    move |_: On<Activate>, mut q_txt: Query<&mut Text, With<ContextStatusText>>| {
        if let Ok(mut txt) = q_txt.single_mut() {
            txt.0 = msg.to_string();
        }
    }
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
    theme.image_context_menu_bg = Some(asset_server.load("theme/kenney/blue/button_square_flat.png"));
    theme.context_menu_bg_margin = 10.0;
}
