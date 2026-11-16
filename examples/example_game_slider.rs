use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;
use bevy_rui::widgets::slider::RuiSlider;

#[derive(Component)]
struct SliderValueLabel;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_slider".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .insert_resource(RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, sync_slider_label)
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
                "Game Slider (Volumen)",
                false,
                |s| {
                    s.width = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow().slider(0.0, 100.0, 75.0, |s| {
                        s.width = Val::Percent(100.0);
                        s.margin = UiRect::vertical(Val::Px(12.0));
                    });

                    c.reborrow()
                        .label("Volumen Maestro: 75%", |font, color| {
                            font.font_size = FontSize::Px(18.0);
                            color.0 = Color::WHITE;
                        })
                        .insert(SliderValueLabel);
                },
            );
        },
    );
}

fn sync_slider_label(
    q_slider: Query<&RuiSlider, Changed<RuiSlider>>,
    mut q_label: Query<&mut Text, With<SliderValueLabel>>,
) {
    for slider in &q_slider {
        if let Ok(mut txt) = q_label.single_mut() {
            txt.0 = format!("Volumen Maestro: {:.0}%", slider.value);
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
    theme.image_slider_track = Some(asset_server.load("theme/kenney/blue/textbox_bg.png"));
    theme.slider_track_margin = 4.0;
    theme.image_progress_bar_fill = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.progress_bar_fill_margin = 4.0;
    theme.image_slider_handle = Some(asset_server.load("theme/kenney/yellow/button_square_depth_gradient.png"));
}
