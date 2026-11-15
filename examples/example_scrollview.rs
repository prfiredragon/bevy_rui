use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_scrollview".to_string(),
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
                    s.width = Val::Px(420.0);
                    s.height = Val::Px(380.0);
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: ScrollView", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .scrollview(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.flex_grow = 1.0;
                                s.padding = UiRect::all(Val::Px(10.0));
                                s.border = UiRect::all(Val::Px(1.0));
                            },
                            |scroll| {
                                let mut sc = scroll.reborrow();
                                for i in 1..=25 {
                                    sc.reborrow().button(
                                        |s| {
                                            s.width = Val::Px(340.0);
                                            s.height = Val::Px(38.0);
                                            s.margin = UiRect::bottom(Val::Px(6.0));
                                        },
                                        |btn| {
                                            btn.reborrow()
                                                .label(&format!("Elemento de lista #{i}"), |f, _| {
                                                    f.font_size = FontSize::Px(16.0);
                                                })
                                                .insert(RuiThemeElement::ButtonText);
                                        },
                                    );
                                }
                            },
                        )
                        .insert((
                            BackgroundColor(Color::srgb(0.11, 0.11, 0.13)),
                            BorderColor::from(Color::srgb(0.25, 0.25, 0.3)),
                        ));
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
