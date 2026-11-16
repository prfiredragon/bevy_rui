use std::env;
use bevy::{prelude::*, text::FontSize, ui_widgets::Activate, window::PresentMode};
use bevy_rui::prelude::*;
use bevy_rui::widgets::file_dialog::{
    spawn_file_dialog, FileDialogMode, FileFilter, RuiFileCanceled, RuiFileDialog, RuiFileSelected,
};

#[derive(Component)]
struct RootUi;

#[derive(Component)]
struct DialogResultText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_file_dialog".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_dialog_events)
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

                root.reborrow().window(
                    "Game File Dialog",
                    false,
                    |s| {
                        s.width = Val::Px(360.0);
                        s.left = Val::Auto;
                        s.top = Val::Auto;
                    },
                    |win, _| {
                        let mut c = win.reborrow();

                        c.reborrow()
                            .button(
                                |s| {
                                    s.width = Val::Percent(100.0);
                                    s.height = Val::Px(44.0);
                                    s.margin = UiRect::bottom(Val::Px(10.0));
                                },
                                |btn| {
                                    btn.reborrow()
                                        .label("Cargar Partida...", |f, _| f.font_size = FontSize::Px(18.0))
                                        .insert(RuiThemeElement::ButtonText);
                                },
                            )
                            .observe(open_dialog_observer(FileDialogMode::Open));

                        c.reborrow()
                            .button(
                                |s| {
                                    s.width = Val::Percent(100.0);
                                    s.height = Val::Px(44.0);
                                    s.margin = UiRect::bottom(Val::Px(10.0));
                                },
                                |btn| {
                                    btn.reborrow()
                                        .label("Guardar Partida...", |f, _| f.font_size = FontSize::Px(18.0))
                                        .insert(RuiThemeElement::ButtonText);
                                },
                            )
                            .observe(open_dialog_observer(FileDialogMode::Save));

                        c.reborrow()
                            .label("Selecciona Cargar o Guardar", |f, col| {
                                f.font_size = FontSize::Px(16.0);
                                col.0 = Color::WHITE;
                            })
                            .insert(DialogResultText);
                    },
                );
            },
        )
        .id();

    commands.entity(root_id).insert(RootUi);
}

fn open_dialog_observer(
    mode: FileDialogMode,
) -> impl Fn(On<Activate>, Commands, Query<Entity, With<RootUi>>, Query<Entity, With<RuiFileDialog>>) {
    move |_: On<Activate>,
          mut commands: Commands,
          q_root: Query<Entity, With<RootUi>>,
          q_existing: Query<Entity, With<RuiFileDialog>>| {
        if !q_existing.is_empty() {
            return;
        }
        if let Ok(root_e) = q_root.single() {
            let start_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());
            let title = if mode == FileDialogMode::Open {
                "Cargar Partida"
            } else {
                "Guardar Partida"
            };
            commands.entity(root_e).with_children(|ui| {
                let mut spawner = ui.reborrow();
                let mut dlg = spawn_file_dialog(
                    &mut spawner,
                    title,
                    mode,
                    start_dir,
                    None,
                    vec![FileFilter {
                        name: "Todos (*.*)".to_string(),
                        extensions: vec!["*".to_string()],
                    }],
                );
                dlg.entry::<Node>().and_modify(|mut n| {
                    n.left = Val::Auto;
                    n.top = Val::Auto;
                });
            });
        }
    }
}

fn handle_dialog_events(
    mut ev_selected: MessageReader<RuiFileSelected>,
    mut ev_canceled: MessageReader<RuiFileCanceled>,
    mut q_txt: Query<&mut Text, With<DialogResultText>>,
) {
    for msg in ev_selected.read() {
        if let Ok(mut txt) = q_txt.single_mut() {
            txt.0 = format!("{:?}: {}", msg.mode, msg.path.display());
        }
    }
    for _ in ev_canceled.read() {
        if let Ok(mut txt) = q_txt.single_mut() {
            txt.0 = "Diálogo cancelado".to_string();
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
    theme.image_textbox_bg = Some(asset_server.load("theme/kenney/blue/textbox_bg.png"));
    theme.textbox_margin = 8.0;
}
