use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct ContextStatusText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_context_menu".to_string(),
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
    let emoji_font = asset_server.load("fonts/openmojicolor.ttf");
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
                    s.width = Val::Px(400.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(16.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Context Menu", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .vbox(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Px(140.0);
                                s.justify_content = JustifyContent::Center;
                                s.align_items = AlignItems::Center;
                                s.border = UiRect::all(Val::Px(2.0));
                                s.border_radius = BorderRadius::all(Val::Px(6.0));
                                s.row_gap = Val::Px(8.0);
                            },
                            |area| {
                                let mut a = area.reborrow();
                                a.reborrow().label("Haz Clic Derecho Aquí", |f, col| {
                                    f.font_size = FontSize::Px(20.0);
                                    col.0 = Color::WHITE;
                                });
                                a.reborrow().label("(Abre el menú contextual)", |f, col| {
                                    f.font_size = FontSize::Px(14.0);
                                    col.0 = Color::srgb(0.7, 0.75, 0.85);
                                });

                                let f1 = emoji_font.clone();
                                let f2 = emoji_font.clone();
                                let f3 = emoji_font.clone();
                                a.reborrow().context_menu(|menu| {
                                    let mut m = menu.reborrow();
                                    m.reborrow()
                                        .context_menu_item(
                                            "Copiar elemento",
                                            Some(RuiIcon::Emoji("📋".to_string(), Some(f1))),
                                            |_| {},
                                        )
                                        .observe(set_status("Seleccionado: Copiar elemento"));

                                    m.reborrow()
                                        .context_menu_item(
                                            "Duplicar elemento",
                                            Some(RuiIcon::Emoji("✨".to_string(), Some(f2))),
                                            |_| {},
                                        )
                                        .observe(set_status("Seleccionado: Duplicar elemento"));

                                    m.reborrow()
                                        .context_menu_item(
                                            "Eliminar",
                                            Some(RuiIcon::Emoji("🗑️".to_string(), Some(f3))),
                                            |_| {},
                                        )
                                        .observe(set_status("Seleccionado: Eliminar"));
                                });
                            },
                        )
                        .insert((
                            BackgroundColor(Color::srgb(0.2, 0.24, 0.32)),
                            BorderColor::from(Color::srgb(0.4, 0.5, 0.7)),
                        ));

                    c.reborrow()
                        .label("Seleccionado: (ninguno)", |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(ContextStatusText);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
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
