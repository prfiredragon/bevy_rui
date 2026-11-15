use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct CheckboxStatusText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_checkbox".to_string(),
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
                "Game Checkbox",
                false,
                |s| {
                    s.width = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow()
                        .checkbox("Activar Música", true, |s| s.margin = UiRect::bottom(Val::Px(8.0)))
                        .observe(update_status);

                    c.reborrow()
                        .checkbox("Pantalla Completa", false, |s| s.margin = UiRect::bottom(Val::Px(8.0)))
                        .observe(update_status);

                    c.reborrow()
                        .checkbox("Mostrar FPS", true, |s| s.margin = UiRect::bottom(Val::Px(12.0)))
                        .observe(update_status);

                    c.reborrow()
                        .label("Opciones activas: 2 / 3", |font, color| {
                            font.font_size = FontSize::Px(17.0);
                            color.0 = Color::srgb(1.0, 0.95, 0.45);
                        })
                        .insert(CheckboxStatusText);
                },
            );
        },
    );
}

fn update_status(
    _: On<Activate>,
    q_checkboxes: Query<&RuiCheckbox>,
    mut q_status: Query<&mut Text, With<CheckboxStatusText>>,
) {
    let total = q_checkboxes.iter().count();
    let checked = q_checkboxes.iter().filter(|c| c.checked).count();
    if let Ok(mut txt) = q_status.single_mut() {
        txt.0 = format!("Opciones activas: {checked} / {total}");
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
