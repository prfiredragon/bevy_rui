use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct ClipboardTextBox;

#[derive(Component)]
struct ClipboardStatus;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_clipboard".to_string(),
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
                "Game Clipboard",
                false,
                |s| {
                    s.width = Val::Px(420.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow()
                        .textbox("Código de invitación...", |s, font, _| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(42.0);
                            s.margin = UiRect::bottom(Val::Px(12.0));
                            font.font_size = FontSize::Px(18.0);
                        })
                        .insert((
                            ClipboardTextBox,
                            RuiTextBox {
                                text: "SEED-2026-RUI".to_string(),
                                placeholder: "Código de invitación...".to_string(),
                                ..default()
                            },
                        ));

                    c.reborrow().hbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.column_gap = Val::Px(10.0);
                            s.margin = UiRect::bottom(Val::Px(12.0));
                        },
                        |row| {
                            let mut r = row.reborrow();

                            r.reborrow()
                                .button(
                                    |s| {
                                        s.flex_grow = 1.0;
                                        s.height = Val::Px(42.0);
                                    },
                                    |btn| {
                                        btn.reborrow()
                                            .label("Copiar", |f, _| f.font_size = FontSize::Px(18.0))
                                            .insert(RuiThemeElement::ButtonText);
                                    },
                                )
                                .observe(
                                    |_: On<Activate>,
                                     q_box: Query<&RuiTextBox, With<ClipboardTextBox>>,
                                     mut q_status: Query<&mut Text, With<ClipboardStatus>>,
                                     mut clipboard: NonSendMut<RuiClipboard>| {
                                        if let Ok(tb) = q_box.single() {
                                            clipboard.set_text(tb.text.clone());
                                            if let Ok(mut status) = q_status.single_mut() {
                                                status.0 = format!("Copiado: \"{}\"", tb.text);
                                            }
                                        }
                                    },
                                );

                            r.reborrow()
                                .button(
                                    |s| {
                                        s.flex_grow = 1.0;
                                        s.height = Val::Px(42.0);
                                    },
                                    |btn| {
                                        btn.reborrow()
                                            .label("Pegar", |f, _| f.font_size = FontSize::Px(18.0))
                                            .insert(RuiThemeElement::ButtonText);
                                    },
                                )
                                .observe(
                                    |_: On<Activate>,
                                     mut q_box: Query<&mut RuiTextBox, With<ClipboardTextBox>>,
                                     mut q_status: Query<&mut Text, With<ClipboardStatus>>,
                                     mut clipboard: NonSendMut<RuiClipboard>| {
                                        if let Some(text) = clipboard.get_text() {
                                            if let Ok(mut tb) = q_box.single_mut() {
                                                tb.text = text.clone();
                                                tb.cursor_index = tb.text.chars().count();
                                            }
                                            if let Ok(mut status) = q_status.single_mut() {
                                                status.0 = format!("Pegado: \"{}\"", text);
                                            }
                                        }
                                    },
                                );
                        },
                    );

                    c.reborrow()
                        .label("Estado: Listo", |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::srgb(1.0, 0.95, 0.45);
                        })
                        .insert(ClipboardStatus);
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
