use std::env;
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_file_dialog".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_dialog_events)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.spawn(Camera2d);

    let root_id = commands
        .rui_root(
            |s| {
                s.justify_content = JustifyContent::Center;
                s.align_items = AlignItems::Center;
            },
            |ui| {
                let mut root = ui.reborrow();

                root.reborrow()
                    .vbox(
                        |s| {
                            s.width = Val::Px(420.0);
                            s.height = Val::Auto;
                            s.padding = UiRect::all(Val::Px(24.0));
                            s.row_gap = Val::Px(14.0);
                            s.align_items = AlignItems::Center;
                            s.border = UiRect::all(Val::Px(2.0));
                            s.border_radius = BorderRadius::all(Val::Px(8.0));
                        },
                        |card| {
                            let mut c = card.reborrow();

                            c.reborrow().label("Widget: File Dialog", |f, col| {
                                f.font_size = FontSize::Px(24.0);
                                col.0 = Color::WHITE;
                            });

                            c.reborrow()
                                .button(
                                    |s| {
                                        s.width = Val::Px(220.0);
                                        s.height = Val::Px(42.0);
                                    },
                                    |btn| {
                                        btn.reborrow()
                                            .label("Abrir Archivo...", |f, _| f.font_size = FontSize::Px(18.0))
                                            .insert(RuiThemeElement::ButtonText);
                                    },
                                )
                                .observe(open_dialog_observer(FileDialogMode::Open));

                            c.reborrow()
                                .button(
                                    |s| {
                                        s.width = Val::Px(220.0);
                                        s.height = Val::Px(42.0);
                                    },
                                    |btn| {
                                        btn.reborrow()
                                            .label("Guardar Archivo...", |f, _| f.font_size = FontSize::Px(18.0))
                                            .insert(RuiThemeElement::ButtonText);
                                    },
                                )
                                .observe(open_dialog_observer(FileDialogMode::Save));

                            c.reborrow()
                                .label("Resultado: Selecciona Abrir o Guardar", |f, col| {
                                    f.font_size = FontSize::Px(16.0);
                                    col.0 = Color::srgb(0.5, 0.9, 0.6);
                                })
                                .insert(DialogResultText);
                        },
                    )
                    .insert((
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                        BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
                    ));
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
                "Seleccionar Archivo para Abrir"
            } else {
                "Guardar Archivo Como..."
            };
            commands.entity(root_e).with_children(|ui| {
                let mut spawner = ui.reborrow();
                let mut dlg = spawn_file_dialog(
                    &mut spawner,
                    title,
                    mode,
                    start_dir,
                    None,
                    vec![
                        FileFilter {
                            name: "Todos (*.*)".to_string(),
                            extensions: vec!["*".to_string()],
                        },
                        FileFilter {
                            name: "Rust (*.rs)".to_string(),
                            extensions: vec!["rs".to_string()],
                        },
                    ],
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
            txt.0 = format!("Seleccionado ({:?}): {}", msg.mode, msg.path.display());
        }
    }
    for _ in ev_canceled.read() {
        if let Ok(mut txt) = q_txt.single_mut() {
            txt.0 = "Resultado: Operación cancelada".to_string();
        }
    }
}
