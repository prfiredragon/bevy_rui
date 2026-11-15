use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct SingleLineInput;

#[derive(Component)]
struct MultiLineInput;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_textbox".to_string(),
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
                "Game TextBox",
                false,
                |s| {
                    s.width = Val::Px(420.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();

                    c.reborrow()
                        .textbox("Nombre del Héroe...", |s, font, _| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(42.0);
                            s.margin = UiRect::bottom(Val::Px(10.0));
                            font.font_size = FontSize::Px(18.0);
                        })
                        .insert(SingleLineInput);

                    c.reborrow()
                        .multiline_textbox("Biografía del personaje...", |s, font, _| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(110.0);
                            s.margin = UiRect::bottom(Val::Px(12.0));
                            font.font_size = FontSize::Px(16.0);
                        })
                        .insert(MultiLineInput);

                    c.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Percent(100.0);
                                s.height = Val::Px(42.0);
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Limpiar Campos", |f, _| f.font_size = FontSize::Px(18.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .observe(
                            |_: On<Activate>,
                             mut q_boxes: Query<
                                &mut RuiTextBox,
                                Or<(With<SingleLineInput>, With<MultiLineInput>)>,
                            >| {
                                for mut tb in &mut q_boxes {
                                    tb.text.clear();
                                    tb.cursor_index = 0;
                                }
                            },
                        );
                },
            );
        },
    );
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
    theme.image_textbox_bg = Some(asset_server.load("theme/kenney/blue/textbox_bg.png"));
    theme.textbox_margin = 8.0;
}
