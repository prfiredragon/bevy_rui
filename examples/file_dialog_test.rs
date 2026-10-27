//! Ejemplo para demostrar cómo utilizar el RuiFileDialog para "Abrir" y "Guardar" archivos
//! utilizando exclusivamente los widgets nativos de bevy_rui.

use std::env;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::ui_widgets::Activate;
use bevy_rui::widgets::RuiBuilderExt;
use bevy_rui::RuiRootBuilderExt;
use bevy_rui::widgets::file_dialog::{spawn_file_dialog, FileDialogMode, RuiFileSelected, RuiFileCanceled};

fn main() {
    App::new()
        .insert_resource(MeshPickingSettings {
            require_markers: false,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .insert_resource(UiPickingSettings {
            require_markers: false,
        })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) 
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync, 
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(MeshPickingPlugin)
        .add_plugins(bevy_rui::RuiPlugin)
        .insert_resource(bevy_rui::navigation::RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, listen_to_file_dialog_messages)
        .run();
}

#[derive(Component)]
struct MainUiRoot;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut btn_open_id = Entity::PLACEHOLDER;
    let mut btn_save_id = Entity::PLACEHOLDER;

    // Construcción de la jerarquía aplicando reborrows lineales limpios
    let root_entity = commands.rui_root(|_| {}, |ui| {
        let mut root_ui = ui.reborrow();
        root_ui.vbox(|s| {
            s.width = Val::Percent(100.0);
            s.height = Val::Percent(100.0);
            s.align_items = AlignItems::Center;
            s.justify_content = JustifyContent::Center;
        }, |vbox| {
            let mut vb = vbox.reborrow();
            
            let mut lbl_bind = vb.reborrow();
            lbl_bind.label("Ejemplo de File Dialog Nativo en Rui", |font, color| {
                font.font_size = FontSize::Px(30.0);
                color.0 = Color::WHITE;
            });

            let mut open_bind = vb.reborrow();
            btn_open_id = open_bind.button(|s| {
                s.width = Val::Px(200.0);
                s.height = Val::Px(50.0);
                s.margin = UiRect::vertical(Val::Px(20.0));
            }, |b| {
                let mut btn_lbl = b.reborrow();
                btn_lbl.label("Probar 'Abrir Archivo'", |_,_|{}).insert(bevy_rui::theme::RuiThemeElement::ButtonText);
            }).id();

            let mut save_bind = vb.reborrow();
            btn_save_id = save_bind.button(|s| {
                s.width = Val::Px(200.0);
                s.height = Val::Px(50.0);
            }, |b| {
                let mut btn_lbl = b.reborrow();
                btn_lbl.label("Probar 'Guardar Archivo'", |_,_|{}).insert(bevy_rui::theme::RuiThemeElement::ButtonText);
            }).id();
        });
    }).id();

    commands.entity(root_entity).insert(MainUiRoot);

    // CORRECCIÓN DEFINITIVA DE LIFETIMES: Hacemos reborrow explícito del spawner mutable ('ui') 
    // dentro de with_children para neutralizar la invarianza del Nomicon de Rust en Bevy 0.20.
    commands.entity(btn_open_id).observe(move |_: On<Activate>, mut commands: Commands, q_root: Query<Entity, With<MainUiRoot>>| {
        if let Ok(root_e) = q_root.single() {
            let start_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());
            
            commands.entity(root_e).with_children(|ui| {
                let mut local_ui = ui.reborrow();
                spawn_file_dialog(&mut local_ui, "Seleccionar Archivo para Abrir", FileDialogMode::Open, start_dir, None, vec![]);
            });
        }
    });

    commands.entity(btn_save_id).observe(move |_: On<Activate>, mut commands: Commands, q_root: Query<Entity, With<MainUiRoot>>| {
        if let Ok(root_e) = q_root.single() {
            let start_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());
            
            commands.entity(root_e).with_children(|ui| {
                let mut local_ui = ui.reborrow();
                spawn_file_dialog(&mut local_ui, "Guardar Archivo Como...", FileDialogMode::Save, start_dir, None, vec![]);
            });
        }
    });
}

fn listen_to_file_dialog_messages(
    mut ev_selected: MessageReader<RuiFileSelected>,
    mut ev_canceled: MessageReader<RuiFileCanceled>,
) {
    for msg in ev_selected.read() {
        match msg.mode {
            FileDialogMode::Open => println!("¡El usuario quiere ABRIR el archivo: {:?}", msg.path),
            FileDialogMode::Save => println!("¡El usuario quiere GUARDAR en el archivo: {:?}", msg.path),
        }
    }

    for _msg in ev_canceled.read() {
        println!("El usuario ha cancelado el File Dialog.");
    }
}
