//! Ejemplo para demostrar cómo utilizar el RuiFileDialog para "Abrir" y "Guardar" archivos
//! utilizando exclusivamente los widgets nativos de bevy_rui.

use std::env;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rui::widgets::RuiBuilderExt;
use bevy_rui::RuiRootBuilderExt;
use bevy_rui::widgets::file_dialog::{spawn_file_dialog, FileDialogMode, RuiFileSelected, RuiFileCanceled};

fn main() {
    App::new()
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .insert_resource(UiPickingSettings {
            require_markers: true,
        })
        
        .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest()) // Tu configuración actual para pixel art/texturas nítidas
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            // Cambiamos el modo de presentación para evitar el crash en Hyprland
                            present_mode: PresentMode::AutoNoVsync, 
                            ..default()
                        }),
                        ..default()
                    }),
            )
        
        .add_plugins(MeshPickingPlugin) // Necesario en 0.18.1 para picking 3D
        .add_plugins(bevy_rui::RuiPlugin)
        // Habilitar el plugin de navegación (opcional pero recomendado para Rui)
        .insert_resource(bevy_rui::navigation::RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, handle_buttons)
        // Sistema que escucha la respuesta del File Dialog
        .add_systems(Update, listen_to_file_dialog_messages)
        .run();
}

#[derive(Component)]
enum DialogTrigger {
    Open,
    Save,
}

#[derive(Component)]
struct MainUiRoot;

fn setup(mut commands: Commands) {
    // Cámara de la interfaz de usuario
    commands.spawn(Camera2d);

    // Utilizamos rui_root como en el resto de la aplicación
    commands.rui_root(|_| {}, |ui| {
        ui.vbox(|s| {
            s.width = Val::Percent(100.0);
            s.height = Val::Percent(100.0);
            s.align_items = AlignItems::Center;
            s.justify_content = JustifyContent::Center;
        }, |vbox| {
            // Texto de título
            vbox.label("Ejemplo de File Dialog Nativo en Rui", |font, color| {
                font.font_size = 30.0;
                color.0 = Color::WHITE;
            });

            // Botón para probar la ventana de "Abrir Archivo"
            vbox.button(|s| {
                s.width = Val::Px(200.0);
                s.height = Val::Px(50.0);
                s.margin = UiRect::vertical(Val::Px(20.0));
            }, |b| {
                b.label("Probar 'Abrir Archivo'", |_,_|{}).insert(bevy_rui::theme::RuiThemeElement::ButtonText);
            }).insert(DialogTrigger::Open);

            // Botón para probar la ventana de "Guardar Archivo"
            vbox.button(|s| {
                s.width = Val::Px(200.0);
                s.height = Val::Px(50.0);
            }, |b| {
                b.label("Probar 'Guardar Archivo'", |_,_|{}).insert(bevy_rui::theme::RuiThemeElement::ButtonText);
            }).insert(DialogTrigger::Save);
        });
    }).insert(MainUiRoot);
}

/// Escucha los clics de nuestros botones de prueba e invoca al File Dialog.
fn handle_buttons(
    mut commands: Commands,
    q_interactions: Query<(&Interaction, &DialogTrigger), Changed<Interaction>>,
    q_root: Query<Entity, With<MainUiRoot>>,
) {
    let root_entity = if let Some(e) = q_root.iter().next() { e } else { return; };
    
    for (interaction, trigger) in &q_interactions {
        if *interaction == Interaction::Pressed {
            // Obtenemos el directorio actual donde se está ejecutando el juego
            let start_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());

            // Decidimos el modo y el título dependiendo del botón
            let (mode, title) = match trigger {
                DialogTrigger::Open => (FileDialogMode::Open, "Seleccionar Archivo para Abrir"),
                DialogTrigger::Save => (FileDialogMode::Save, "Guardar Archivo Como..."),
            };

            // Para invocar el explorador de archivos, lo enganchamos al rui_root principal.
            commands.entity(root_entity).with_children(|ui| {
                spawn_file_dialog(ui, title, mode, start_dir);
            });
        }
    }
}

/// Este sistema es el encargado de recoger el resultado de la ventana de archivos.
/// Escuchamos a los mensajes (eventos) que dispara `file_dialog.rs`.
fn listen_to_file_dialog_messages(
    mut ev_selected: MessageReader<RuiFileSelected>,
    mut ev_canceled: MessageReader<RuiFileCanceled>,
) {
    // Si el usuario eligió un archivo y pulsó "Confirmar" (Abrir o Guardar)
    for msg in ev_selected.read() {
        match msg.mode {
            FileDialogMode::Open => println!("¡El usuario quiere ABRIR el archivo: {:?}", msg.path),
            FileDialogMode::Save => println!("¡El usuario quiere GUARDAR en el archivo: {:?}", msg.path),
        }
    }

    // Si el usuario cerró la ventana o pulsó "Cancelar"
    for _msg in ev_canceled.read() {
        println!("El usuario ha cancelado el File Dialog.");
    }
}
