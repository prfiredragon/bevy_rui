use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct WindowRoot;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_windows".to_string(),
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

    let root_id = commands
        .rui_root(
            |s| {
                s.justify_content = JustifyContent::Center;
                s.align_items = AlignItems::Center;
            },
            |ui| {
                let mut root = ui.reborrow();
                spawn_centered_game_window(&mut root);
            },
        )
        .id();

    commands.entity(root_id).insert(WindowRoot);
}

fn spawn_centered_game_window<'a>(ui: &'a mut ChildSpawnerCommands<'a>) {
    ui.window(
        "Game Window (Arrástrame)",
        true,
        |s| {
            s.width = Val::Px(360.0);
            s.left = Val::Auto;
            s.top = Val::Auto;
        },
        |win, _| {
            let mut w = win.reborrow();
            w.reborrow().label("Ventana con Texturas Ninepatch", |f, col| {
                f.font_size = FontSize::Px(20.0);
                col.0 = Color::WHITE;
            });

            w.reborrow().label(
                "Arrastra la barra amarilla superior para mover la ventana.",
                |f, col| {
                    f.font_size = FontSize::Px(15.0);
                    col.0 = Color::srgb(0.9, 0.95, 1.0);
                },
            );

            w.reborrow()
                .button(
                    |s| {
                        s.width = Val::Percent(100.0);
                        s.height = Val::Px(42.0);
                        s.margin = UiRect::top(Val::Px(14.0));
                    },
                    |btn| {
                        btn.reborrow()
                            .label("Abrir Otra Ventana", |f, _| f.font_size = FontSize::Px(17.0))
                            .insert(RuiThemeElement::ButtonText);
                    },
                )
                .observe(
                    |_: On<Activate>,
                     mut commands: Commands,
                     q_root: Query<Entity, With<WindowRoot>>| {
                        if let Ok(root_e) = q_root.single() {
                            commands.entity(root_e).with_children(|ui| {
                                let mut spawner = ui.reborrow();
                                spawn_centered_game_window(&mut spawner);
                            });
                        }
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
}
