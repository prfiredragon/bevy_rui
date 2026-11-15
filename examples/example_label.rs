use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_label".to_string(),
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
                    s.width = Val::Px(400.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(14.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Label", |font, color| {
                        font.font_size = FontSize::Px(28.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().label("Etiqueta de subtítulo estilizada", |font, color| {
                        font.font_size = FontSize::Px(20.0);
                        color.0 = Color::srgb(0.45, 0.8, 1.0);
                    });

                    c.reborrow().label(
                        "Texto descriptivo con color personalizado y Pickable::IGNORE automático.",
                        |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::srgb(0.75, 0.75, 0.78);
                        },
                    );

                    c.reborrow().label("Estado: Activo ✓", |font, color| {
                        font.font_size = FontSize::Px(18.0);
                        color.0 = Color::srgb(0.5, 0.95, 0.55);
                    });
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
