use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_code_editor".to_string(),
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

    let sample_code = r#"// Ejemplo de Code Editor centrado
fn saludar(nombre: &str) {
    println!("¡Hola, {} desde bevy_rui!", nombre);
}

fn main() {
    for i in 1..=3 {
        saludar(&format!("Usuario #{i}"));
    }
}
"#;

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.vbox(
                |s| {
                    s.width = Val::Px(680.0);
                    s.height = Val::Px(420.0);
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Code Editor", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().code_editor(sample_code, "rs", |s, font, _| {
                        s.width = Val::Percent(100.0);
                        s.flex_grow = 1.0;
                        font.font = FontSource::Handle(asset_server.load("fonts/FiraMono-Medium.ttf"));
                        font.font_size = FontSize::Px(18.0);
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
