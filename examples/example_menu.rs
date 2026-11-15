use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct MenuStatusText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_menu".to_string(),
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
                    s.width = Val::Px(480.0);
                    s.height = Val::Px(280.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
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
                            m.reborrow().submenu("Archivo", None, 0, |_| {}, |file_menu| {
                                let mut fm = file_menu.reborrow();
                                fm.reborrow()
                                    .menu_item(
                                        "Abrir",
                                        Some(RuiIcon::Emoji("📁".to_string(), Some(f_open))),
                                        |_| {},
                                    )
                                    .observe(on_menu_select("Archivo -> Abrir"));

                                fm.reborrow()
                                    .menu_item(
                                        "Guardar",
                                        Some(RuiIcon::Emoji("💾".to_string(), Some(f_save))),
                                        |_| {},
                                    )
                                    .observe(on_menu_select("Archivo -> Guardar"));
                            });

                            let f_light = emoji_font.clone();
                            m.reborrow().submenu("Insertar", None, 0, |_| {}, |insert_menu| {
                                let mut im = insert_menu.reborrow();
                                im.reborrow().submenu(" Objetos 3D ", None, 1, |_| {}, |sub| {
                                    let mut sm = sub.reborrow();
                                    sm.reborrow()
                                        .menu_item("Cubo", None, |_| {})
                                        .observe(on_menu_select("Insertar -> Objetos 3D -> Cubo"));
                                    sm.reborrow()
                                        .menu_item("Esfera", None, |_| {})
                                        .observe(on_menu_select("Insertar -> Objetos 3D -> Esfera"));
                                });

                                im.reborrow()
                                    .menu_item(
                                        "Luz Direccional",
                                        Some(RuiIcon::Emoji("☀️".to_string(), Some(f_light))),
                                        |_| {},
                                    )
                                    .observe(on_menu_select("Insertar -> Luz Direccional"));
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
                            s.row_gap = Val::Px(12.0);
                            s.padding = UiRect::all(Val::Px(20.0));
                        },
                        |body| {
                            let mut b = body.reborrow();
                            b.reborrow().label("Widget: Menu (MenuBar & Submenu)", |f, col| {
                                f.font_size = FontSize::Px(24.0);
                                col.0 = Color::WHITE;
                            });
                            b.reborrow()
                                .label("Acción: Selecciona una opción del menú superior", |f, col| {
                                    f.font_size = FontSize::Px(16.0);
                                    col.0 = Color::srgb(0.5, 0.9, 0.6);
                                })
                                .insert(MenuStatusText);
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

fn on_menu_select(
    label: &'static str,
) -> impl Fn(On<Activate>, Query<&mut Text, With<MenuStatusText>>) {
    move |_: On<Activate>, mut q_txt: Query<&mut Text, With<MenuStatusText>>| {
        if let Ok(mut txt) = q_txt.single_mut() {
            txt.0 = format!("Acción: {label}");
        }
    }
}
