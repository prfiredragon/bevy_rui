use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

#[derive(Component)]
struct ColorPreviewBox;

#[derive(Component)]
struct ColorValueText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_color_picker".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_color_preview)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.spawn(Camera2d);

    let initial_color = Color::srgb(0.25, 0.65, 0.95);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.vbox(
                |s| {
                    s.width = Val::Px(380.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(16.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Color Picker", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().color_picker(initial_color, |s| {
                        s.width = Val::Px(260.0);
                    });

                    c.reborrow().spawn((
                        Node {
                            width: Val::Px(260.0),
                            height: Val::Px(48.0),
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(initial_color),
                        BorderColor::from(Color::WHITE),
                        ColorPreviewBox,
                    ));

                    c.reborrow()
                        .label("Color actual: RGB(0.25, 0.65, 0.95)", |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::srgb(0.8, 0.8, 0.8);
                        })
                        .insert(ColorValueText);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}

fn update_color_preview(
    q_picker: Query<&RuiColorPicker, Changed<RuiColorPicker>>,
    mut q_preview: Query<&mut BackgroundColor, With<ColorPreviewBox>>,
    mut q_text: Query<&mut Text, With<ColorValueText>>,
) {
    for picker in &q_picker {
        if let Ok(mut bg) = q_preview.single_mut() {
            bg.0 = picker.color;
        }
        if let Ok(mut txt) = q_text.single_mut() {
            let srgba = picker.color.to_srgba();
            txt.0 = format!(
                "Color actual: RGB({:.2}, {:.2}, {:.2})",
                srgba.red, srgba.green, srgba.blue
            );
        }
    }
}
