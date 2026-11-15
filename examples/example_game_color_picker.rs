use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct ColorPreviewBox;

#[derive(Component)]
struct ColorValueText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_color_picker".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_color_preview)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut theme: ResMut<RuiTheme>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    apply_game_theme(&asset_server, &mut theme);

    commands.spawn(Camera2d);

    let initial_color = Color::srgb(1.0, 0.8, 0.1);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.window(
                "Game Color Picker",
                false,
                |s| {
                    s.width = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow().color_picker(initial_color, |s| {
                        s.width = Val::Percent(100.0);
                        s.margin = UiRect::bottom(Val::Px(12.0));
                    });

                    c.reborrow().spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(44.0),
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(initial_color),
                        BorderColor::from(Color::WHITE),
                        ColorPreviewBox,
                    ));

                    c.reborrow()
                        .label("Color: RGB(1.00, 0.80, 0.10)", |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::WHITE;
                        })
                        .insert(ColorValueText);
                },
            );
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
                "Color: RGB({:.2}, {:.2}, {:.2})",
                srgba.red, srgba.green, srgba.blue
            );
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
