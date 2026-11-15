use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct MenuStatusText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_menu".to_string(),
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
            root.vbox(
                |s| {
                    s.width = Val::Px(460.0);
                    s.height = Val::Px(260.0);
                    s.padding = UiRect::all(Val::Px(12.0));
                    s.overflow = Overflow::visible();
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().menu_bar(
                        |s| {
                            s.flex_shrink = 0.0;
                        },
                        |menu| {
                            let mut m = menu.reborrow();

                            let f_open = emoji_font.clone();
                            let f_save = emoji_font.clone();
                            m.reborrow().submenu("Partida", None, 0, |_| {}, |file_menu| {
                                let mut fm = file_menu.reborrow();
                                fm.reborrow()
                                    .menu_item(
                                        "Cargar",
                                        Some(RuiIcon::Emoji("📁".to_string(), Some(f_open))),
                                        |_| {},
                                    )
                                    .observe(on_menu_select("Partida -> Cargar"));

                                fm.reborrow()
                                    .menu_item(
                                        "Guardar",
                                        Some(RuiIcon::Emoji("💾".to_string(), Some(f_save))),
                                        |_| {},
                                    )
                                    .observe(on_menu_select("Partida -> Guardar"));
                            });

                            m.reborrow().submenu("Invocar", None, 0, |_| {}, |insert_menu| {
                                let mut im = insert_menu.reborrow();
                                im.reborrow().submenu(" Criaturas ", None, 1, |_| {}, |sub| {
                                    let mut sm = sub.reborrow();
                                    sm.reborrow()
                                        .menu_item("Gólem", None, |_| {})
                                        .observe(on_menu_select("Invocar -> Gólem"));
                                    sm.reborrow()
                                        .menu_item("Fénix", None, |_| {})
                                        .observe(on_menu_select("Invocar -> Fénix"));
                                });
                            });

                            m.reborrow()
                                .menu_item("Ayuda", None, |_| {})
                                .observe(on_menu_select("Ayuda"));
                        },
                    );

                    c.reborrow().vbox(
                        |s| {
                            s.flex_grow = 1.0;
                            s.justify_content = JustifyContent::Center;
                            s.align_items = AlignItems::Center;
                            s.row_gap = Val::Px(10.0);
                        },
                        |body| {
                            let mut b = body.reborrow();
                            b.reborrow().label("Game Menu Bar", |f, col| {
                                f.font_size = FontSize::Px(24.0);
                                col.0 = Color::WHITE;
                            });
                            b.reborrow()
                                .label("Acción: Selecciona del menú superior", |f, col| {
                                    f.font_size = FontSize::Px(16.0);
                                    col.0 = Color::srgb(1.0, 0.95, 0.45);
                                })
                                .insert(MenuStatusText);
                        },
                    );
                },
            )
            .insert((
                ImageNode::default(),
                BackgroundColor(Color::NONE),
                RuiThemeElement::Panel,
            ));
        },
    );
}

fn on_menu_select(
    label: &'static str,
) -> impl Fn(On<Activate>, Query<&mut Text, With<MenuStatusText>>) {
    move |_: On<Activate>, mut q_txt: Query<&mut Text, With<MenuStatusText>>| {
        if let Ok(mut txt) = q_txt.single_mut() {
            txt.0 = format!("Acción: {label}");
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
}
