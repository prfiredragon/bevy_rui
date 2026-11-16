use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct ClipboardTextBox;

#[derive(Component)]
struct ClipboardStatus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_clipboard".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .insert_resource(RuiNavigationConfig { enabled: true })
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
                    s.width = Val::Px(440.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(16.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Clipboard", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .textbox("Type something to copy...", |s, font, _| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(42.0);
                            font.font_size = FontSize::Px(18.0);
                        })
                        .insert((
                            ClipboardTextBox,
                            RuiTextBox {
                                text: "Hello from RuiClipboard!".to_string(),
                                placeholder: "Type something to copy...".to_string(),
                                ..default()
                            },
                        ));

                    c.reborrow().hbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.justify_content = JustifyContent::SpaceBetween;
                            s.column_gap = Val::Px(12.0);
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
                                            .label("Copy", |f, _| f.font_size = FontSize::Px(18.0))
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
                                                status.0 = format!("Copied: \"{}\"", tb.text);
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
                                            .label("Paste", |f, _| f.font_size = FontSize::Px(18.0))
                                            .insert(RuiThemeElement::ButtonText);
                                    },
                                )
                                .observe(
                                    |_: On<Activate>,
                                     mut q_box: Query<&mut RuiTextBox, With<ClipboardTextBox>>,
                                     mut q_status: Query<&mut Text, With<ClipboardStatus>>,
                                     mut clipboard: NonSendMut<RuiClipboard>| {
                                        if let Some(text) = clipboard.get_text() {
                                            let clean = text.replace(['\r', '\n'], " ");
                                            if let Ok(mut tb) = q_box.single_mut() {
                                                tb.text = clean.clone();
                                                tb.cursor_index = tb.text.chars().count();
                                            }
                                            if let Ok(mut status) = q_status.single_mut() {
                                                status.0 = format!("Pasted: \"{}\"", clean);
                                            }
                                        }
                                    },
                                );
                        },
                    );

                    c.reborrow()
                        .label("Status: Ready (Click Copy/Paste or use Ctrl+C / Ctrl+V)", |font, color| {
                            font.font_size = FontSize::Px(15.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(ClipboardStatus);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
