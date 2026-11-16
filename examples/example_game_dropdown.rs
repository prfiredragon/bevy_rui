use bevy::{prelude::*, text::FontSize, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct SelectedDropdownLabel;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_dropdown".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .insert_resource(RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, sync_dropdown_label)
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
                "Game Dropdown",
                false,
                |s| {
                    s.width = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                    s.overflow = Overflow::visible();
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow().vbox(
                        |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Auto;
                            s.row_gap = Val::Px(16.0);
                            s.padding = UiRect::vertical(Val::Px(6.0));
                            s.align_items = AlignItems::Center;
                            s.overflow = Overflow::visible();
                        },
                        |body| {
                            let mut b = body.reborrow();

                            b.reborrow().dropdown(
                                "Guerrero",
                                &["Guerrero", "Mago", "Arquero", "Paladín"],
                                |s| {
                                    s.width = Val::Px(240.0);
                                    s.height = Val::Px(44.0);
                                    s.padding = UiRect::horizontal(Val::Px(12.0));
                                },
                            );

                            b.reborrow()
                                .label("Héroe elegido: Guerrero", |font, color| {
                                    font.font_size = FontSize::Px(18.0);
                                    color.0 = Color::WHITE;
                                })
                                .insert(SelectedDropdownLabel);
                        },
                    );
                },
            );
        },
    );
}

fn sync_dropdown_label(
    q_dropdown: Query<&RuiDropdown>,
    q_texts: Query<&Text, (Changed<Text>, Without<SelectedDropdownLabel>)>,
    mut q_label: Query<&mut Text, With<SelectedDropdownLabel>>,
) {
    for dropdown in &q_dropdown {
        if let Ok(selected_txt) = q_texts.get(dropdown.text_entity) {
            if let Ok(mut label) = q_label.single_mut() {
                label.0 = format!("Héroe elegido: {}", selected_txt.0);
            }
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
