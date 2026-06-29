use std::path::{PathBuf};
use bevy::prelude::*;
use crate::widgets::textbox::{spawn_textbox, RuiTextBox};
use crate::widgets::scrollview::spawn_scrollview;
use crate::theme::RuiThemeElement;
use crate::widgets::RuiBuilderExt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileDialogMode {
    Open,
    Save,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

#[derive(Component)]
pub struct RuiFileDialog {
    pub mode: FileDialogMode,
    pub current_dir: PathBuf,
    pub selected_file: String,
    pub filters: Vec<FileFilter>,
    pub filter_dropdown: Option<Entity>,
    pub list_container: Entity,
    pub textbox_entity: Entity,
    pub needs_refresh: bool,
    pub root_dir: Option<PathBuf>,
}

#[derive(Component, Clone)]
pub struct RuiFileItem {
    pub path: PathBuf,
    pub is_dir: bool,
    pub dialog_entity: Entity,
}

#[derive(Component)]
pub struct RuiFilePathText(pub Entity);

#[derive(Component, Clone)]
pub enum DialogButtonAction {
    UpDir(Entity),
    CreateDir(Entity),
    Confirm(Entity),
    Cancel(Entity),
    ConfirmCreateDir(Entity, Entity, Entity), // dialog_entity, modal_entity, textbox_entity
    CancelCreateDir(Entity), // modal_entity
}

#[derive(Message, Debug)]
pub struct RuiFileSelected {
    pub path: PathBuf,
    pub mode: FileDialogMode,
}

#[derive(Message, Debug)]
pub struct RuiFileCanceled;

pub fn spawn_file_dialog<'a>(
    parent: &'a mut ChildSpawnerCommands,
    title: &str,
    mode: FileDialogMode,
    start_dir: PathBuf,
    root_dir: Option<PathBuf>,
    filters: Vec<FileFilter>,
) -> EntityCommands<'a> {
    let mut list_container_entity = Entity::PLACEHOLDER;
    let mut textbox_entity = Entity::PLACEHOLDER;
    let mut filter_dropdown_entity = None;
    
    let mut window_cmds = parent.window(title, true, |s| {
        s.width = Val::Px(650.0);
        s.height = Val::Px(450.0);
        s.left = Val::Px(200.0);
        s.top = Val::Px(150.0);
    }, |win_p, dialog_id| {
        
        // Top Bar (Up button + Create Dir + Path)
        win_p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            padding: UiRect::all(Val::Px(4.0)),
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|top_p| {
            top_p.button(|s| { s.width = Val::Px(40.0); s.height = Val::Px(30.0); }, |b| {
                b.label("↑", |_,_|{}).insert(RuiThemeElement::ButtonText);
            }).insert(DialogButtonAction::UpDir(dialog_id));
            
            top_p.button(|s| { s.width = Val::Px(40.0); s.height = Val::Px(30.0); s.margin = UiRect::left(Val::Px(5.0)); }, |b| {
                b.label("+📁", |font,_|{ font.font_size = bevy::prelude::FontSize::Px(14.0); }).insert(RuiThemeElement::ButtonText);
            }).insert(DialogButtonAction::CreateDir(dialog_id));
            
            top_p.spawn((
                Text::new(start_dir.display().to_string()),
                TextFont { font_size: bevy::prelude::FontSize::Px(16.0), ..default() },
                TextColor(Color::WHITE),
                Node { margin: UiRect::left(Val::Px(10.0)), overflow: Overflow::clip(), flex_shrink: 1.0, min_width: Val::Px(0.0), ..default() }
            )).insert(RuiFilePathText(dialog_id));
        });
        
        // File List ScrollView
        spawn_scrollview(win_p, |s| {
            s.width = Val::Percent(100.0);
            s.height = Val::Px(300.0);
            //s.min_height = Val::Px(0.0);
            //s.flex_basis = Val::Px(0.0);
            //s.flex_grow = 1.0;
            //s.flex_shrink = 1.0;
            s.border = UiRect::all(Val::Px(1.0));
            s.margin = UiRect::vertical(Val::Px(5.0));
        }, |scroll_p| {
            list_container_entity = scroll_p.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..default()
            }).id();
        }).insert(BorderColor::all(Color::BLACK));
        
        // Bottom Bar
        win_p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            padding: UiRect::all(Val::Px(4.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }).with_children(|bot_p| {
            bot_p.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, flex_grow: 1.0, align_items: AlignItems::Center, ..default() }).with_children(|left_p| {
                // File name text box
                textbox_entity = spawn_textbox(left_p, "Nombre del archivo o carpeta...", |s, _, _| {
                    s.flex_grow = 1.0;
                    s.height = Val::Px(30.0);
                    s.margin = UiRect::right(Val::Px(10.0));
                }).id();
                
                if !filters.is_empty() {
                    let filter_strings: Vec<String> = filters.iter().map(|f| f.name.clone()).collect();
                    let str_refs: Vec<&str> = filter_strings.iter().map(|s| s.as_str()).collect();
                    filter_dropdown_entity = Some(crate::widgets::dropdown::spawn_dropdown(left_p, str_refs[0], &str_refs, |s| {
                        s.width = Val::Px(200.0);
                        s.height = Val::Px(30.0);
                        s.margin = UiRect::right(Val::Px(10.0));
                    }).id());
                }
            });
            
            // Buttons
            bot_p.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, ..default() }).with_children(|btn_p| {
                btn_p.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0)); }, |b| {
                    b.label("Cancelar", |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::Cancel(dialog_id));
                
                let confirm_label = if mode == FileDialogMode::Open { "Abrir" } else { "Guardar" };
                btn_p.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(30.0); }, |b| {
                    b.label(confirm_label, |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::Confirm(dialog_id));
            });
        });
    });
    
    window_cmds.insert(RuiFileDialog {
        mode,
        current_dir: start_dir.clone(),
        selected_file: String::new(),
        filters,
        filter_dropdown: filter_dropdown_entity,
        list_container: list_container_entity,
        textbox_entity,
        needs_refresh: true,
        root_dir,
    });
    
    window_cmds
}

fn spawn_create_dir_modal(commands: &mut Commands, dialog_entity: Entity) -> Entity {
    let mut modal_id = Entity::PLACEHOLDER;
    let mut textbox_id = Entity::PLACEHOLDER;
    
    commands.entity(dialog_entity).with_children(|win_p| {
        let modal = win_p.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgba(0.0, 0.0, 0.0, 0.7)) },
            ZIndex(1000), // Ensures it stays on top of everything inside the window
            Interaction::None,
            bevy::ui::FocusPolicy::Block, // Block clicks from passing through
            Pickable::default(),
        )).id();
        
        modal_id = modal;
    });
    
    // Use RuiBuilderExt methods on the modal entity
    commands.entity(modal_id).with_children(|bg_p| {
        bg_p.vbox(|s| {
            s.width = Val::Px(350.0);
            s.height = Val::Px(200.0);            
            s.left = Val::Auto; // Center horizontally
            s.top = Val::Auto;  // Center vertically
            s.padding = UiRect::all(Val::Px(15.0));
            s.border = UiRect::all(Val::Px(2.0));
            s.border_radius = BorderRadius::all(Val::Px(6.0));
            //s.align_items = AlignItems::Center;
        }, |panel| {
            
            panel.label("Crear Nuevo Directorio", |font, _| { font.font_size = bevy::prelude::FontSize::Px(18.0);});
            
            textbox_id = panel.textbox("Nombre de la carpeta", |s, _, _| {
                s.width = Val::Percent(80.0);
                s.height = Val::Px(35.0);
                s.left = Val::Percent(10.0);
                s.margin = UiRect::vertical(Val::Px(10.0));
            }).insert(RuiThemeElement::TextboxBg).id();
            
            panel.hbox(|s| {
                s.width = Val::Percent(100.0);
                s.justify_content = JustifyContent::SpaceEvenly;
            }, |buttons| {
                buttons.button(|s| {
                    s.width = Val::Px(80.0);
                    s.height = Val::Px(35.0);
                }, |b| {
                    b.label("Cancelar", |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::CancelCreateDir(modal_id));
                
                buttons.button(|s| {
                    s.width = Val::Px(80.0);
                    s.height = Val::Px(35.0);
                }, |b| {
                    b.label("Crear", |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::ConfirmCreateDir(dialog_entity, modal_id, textbox_id));
            });
        }).insert((
            crate::theme::RuiThemeElement::Panel,
            BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
            Interaction::None,
            bevy::ui::FocusPolicy::Block,
            Pickable::default(),
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::default() },
        ));
    });
    
    modal_id
}

pub fn update_file_list_ui(
    mut commands: Commands,
    mut q_dialogs: Query<(Entity, &mut RuiFileDialog)>,
    q_children: Query<&Children>,
    mut q_path_texts: Query<(&RuiFilePathText, &mut Text)>,
    asset_server: Res<AssetServer>,
    q_dropdowns: Query<&crate::widgets::dropdown::RuiDropdown>,
    q_texts: Query<&Text, Without<RuiFilePathText>>,
) {
    for (dialog_entity, mut dialog) in &mut q_dialogs {
        if !dialog.needs_refresh {
            continue;
        }
        dialog.needs_refresh = false;
        
        for (path_text_comp, mut text) in &mut q_path_texts {
            if path_text_comp.0 == dialog_entity {
                text.0 = dialog.current_dir.display().to_string();
            }
        }
        
        let container = dialog.list_container;
        
        // Clear old items
        if let Ok(children) = q_children.get(container) {
            for child in children {
                commands.entity(*child).try_despawn();
            }
        }
        
        let mut active_extensions: Vec<String> = vec![];
        if let Some(dropdown_entity) = dialog.filter_dropdown {
            if let Ok(dropdown) = q_dropdowns.get(dropdown_entity) {
                if let Ok(dropdown_text) = q_texts.get(dropdown.text_entity) {
                    let selected_filter_name = &dropdown_text.0;
                    if let Some(filter) = dialog.filters.iter().find(|f| &f.name == selected_filter_name) {
                        active_extensions = filter.extensions.clone();
                    }
                }
            }
        }

        let mut entries = Vec::new();
        if let Ok(read_dir) = std::fs::read_dir(&dialog.current_dir) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                let name = entry.file_name().to_string_lossy().to_string();
                
                if !is_dir && !active_extensions.is_empty() {
                    let mut matches = false;
                    for ext in &active_extensions {
                        if ext == "*" || ext == "*.*" {
                            matches = true;
                            break;
                        }
                        
                        let clean_ext = ext.strip_prefix("*.").unwrap_or(ext);
                        let suffix = format!(".{}", clean_ext.trim_start_matches('.'));
                        let name_lower = name.to_lowercase();
                        if name_lower.ends_with(&suffix.to_lowercase()) {
                            matches = true;
                            break;
                        }
                    }
                    if !matches {
                        continue;
                    }
                }
                
                entries.push((name, path, is_dir));
            }
        }
        
        // Sort: directories first, then alphabetical
        entries.sort_by(|a, b| {
            b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0))
        });
        
        if let Ok(mut entity_cmds) = commands.get_entity(container) {
            entity_cmds.with_children(|p| {
                for (name, path, is_dir) in entries {
                    // Seleccionar icono en base al tipo de archivo/directorio
                    let icon_path = if is_dir {
                        "icons/folder.png"
                    } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        match ext.to_lowercase().as_str() {
                            "png" | "jpg" | "jpeg" | "gif" | "bmp" => "icons/image.png",
                            "rs" | "toml" | "json" => "icons/code.png",
                            "mp3" | "wav" | "ogg" => "icons/audio.png",
                            _ => "icons/file.png",
                        }
                    } else {
                        "icons/file.png"
                    };
                    
                    let texture = asset_server.load(icon_path);
                    
                    let mut btn_cmds = p.button(|s| {
                        s.width = Val::Percent(100.0);
                        s.height = Val::Px(30.0);
                        s.justify_content = JustifyContent::FlexStart;
                        s.align_items = AlignItems::Center;
                        s.padding = UiRect::all(Val::Px(4.0));
                    }, |b| {
                        // Icon image
                        b.spawn((
                            ImageNode::new(texture),
                            Node { width: Val::Px(20.0), height: Val::Px(20.0), margin: UiRect::right(Val::Px(8.0)), ..default() },
                        ));
                        // Text
                        b.spawn((Text::new(name.clone()), TextFont { font_size: bevy::prelude::FontSize::Px(16.0), ..default() }, TextColor(Color::WHITE)));
                    });
                    
                    btn_cmds.insert((RuiFileItem { path, is_dir, dialog_entity }, crate::theme::RuiThemeElement::ListItem));

                    if !is_dir && name == dialog.selected_file {
                        btn_cmds.insert(crate::widgets::button::RuiSelected);
                    }
                }
            });
        }
    }
}

pub fn handle_file_clicks(
    q_interactions: Query<(&Interaction, &RuiFileItem), Changed<Interaction>>,
    mut q_dialogs: Query<(Entity, &mut RuiFileDialog)>,
    mut q_textboxes: Query<&mut RuiTextBox>,
) {
    for (interaction, item) in &q_interactions {
        if *interaction == Interaction::Pressed {
            for (dialog_entity, mut dialog) in &mut q_dialogs {
                if dialog_entity != item.dialog_entity {
                    continue; // Solo afectar a la ventana correspondiente
                }
                
                if item.is_dir {
                    dialog.current_dir = item.path.clone();
                    dialog.needs_refresh = true;
                } else {
                    if let Some(file_name) = item.path.file_name() {
                        dialog.selected_file = file_name.to_string_lossy().to_string();
                        if let Ok(mut textbox) = q_textboxes.get_mut(dialog.textbox_entity) {
                            textbox.text = dialog.selected_file.clone();
                            textbox.cursor_index = textbox.text.chars().count();
                        }
                        dialog.needs_refresh = true;
                    }
                }
            }
        }
    }
}

pub fn handle_dialog_buttons(
    mut commands: Commands,
    q_interactions: Query<(&Interaction, &DialogButtonAction), Changed<Interaction>>,
    mut q_dialogs: Query<(Entity, &mut RuiFileDialog)>,
    q_textboxes: Query<&RuiTextBox>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
    mut ev_selected: MessageWriter<RuiFileSelected>,
    mut ev_canceled: MessageWriter<RuiFileCanceled>,
    q_dropdowns: Query<&crate::widgets::dropdown::RuiDropdown>,
    q_texts: Query<&Text>,
) {
    for (interaction, action) in &q_interactions {
        if *interaction == Interaction::Pressed {
            match action {
                DialogButtonAction::CancelCreateDir(modal_ent) => {
                    active_scope.remove_window(*modal_ent);
                    commands.entity(*modal_ent).try_despawn();
                    continue;
                }
                DialogButtonAction::ConfirmCreateDir(d_ent, modal_ent, txt_ent) => {
                    let mut dir_name = String::new();
                    if let Ok(textbox) = q_textboxes.get(*txt_ent) {
                        dir_name = textbox.text.trim().to_string();
                    }
                    if !dir_name.is_empty() {
                        if let Ok((_, mut dialog)) = q_dialogs.get_mut(*d_ent) {
                            let new_dir = dialog.current_dir.join(&dir_name);
                            let _ = std::fs::create_dir_all(&new_dir);
                            dialog.needs_refresh = true;
                        }
                    }
                    active_scope.remove_window(*modal_ent);
                    commands.entity(*modal_ent).try_despawn();
                    continue;
                }
                DialogButtonAction::CreateDir(d_ent) => {
                    let modal = spawn_create_dir_modal(&mut commands, *d_ent);
                    active_scope.push_window(modal);
                    continue;
                }
                _ => {}
            }
            
            let target_dialog = match action {
                DialogButtonAction::UpDir(e) => *e,
                DialogButtonAction::Confirm(e) => *e,
                DialogButtonAction::Cancel(e) => *e,
                _ => unreachable!(),
            };
            
            for (dialog_entity, mut dialog) in &mut q_dialogs {
                if dialog_entity != target_dialog {
                    continue; // Solo procesar clics para esta ventana
                }
                
                match action {
                    DialogButtonAction::UpDir(_) => {
                        if let Some(parent_dir) = dialog.current_dir.parent() {
                            let can_go_up = if let Some(ref root) = dialog.root_dir {
                                parent_dir.starts_with(root)
                            } else {
                                true
                            };
                            
                            if can_go_up {
                                dialog.current_dir = parent_dir.to_path_buf();
                                dialog.needs_refresh = true;
                            }
                        }
                    }
                    DialogButtonAction::Confirm(_) => {
                        let mut file_name = if let Ok(textbox) = q_textboxes.get(dialog.textbox_entity) {
                            textbox.text.clone()
                        } else {
                            dialog.selected_file.clone()
                        };
                        
                        if !file_name.is_empty() {
                            if let Some(dropdown_entity) = dialog.filter_dropdown {
                                if let Ok(dropdown) = q_dropdowns.get(dropdown_entity) {
                                    if let Ok(dropdown_text) = q_texts.get(dropdown.text_entity) {
                                        let selected_filter_name = &dropdown_text.0;
                                        if let Some(filter) = dialog.filters.iter().find(|f| &f.name == selected_filter_name) {
                                            if !filter.extensions.is_empty() && !filter.extensions.contains(&"*".to_string()) && !filter.extensions.contains(&"*.*".to_string()) {
                                                let ext = &filter.extensions[0];
                                                let clean_ext = ext.strip_prefix("*.").unwrap_or(ext);
                                                let suffix = format!(".{}", clean_ext.trim_start_matches('.'));
                                                if !file_name.to_lowercase().ends_with(&suffix.to_lowercase()) {
                                                    file_name.push_str(&suffix);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        
                            let path = dialog.current_dir.join(file_name);
                            ev_selected.write(RuiFileSelected { path, mode: dialog.mode });
                            commands.entity(dialog_entity).try_despawn();
                        }
                    }
                    DialogButtonAction::Cancel(_) => {
                        ev_canceled.write(RuiFileCanceled);
                        commands.entity(dialog_entity).try_despawn();
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_dropdown_change(
    mut q_dialogs: Query<&mut RuiFileDialog>,
    q_dropdowns: Query<&crate::widgets::dropdown::RuiDropdown>,
    q_texts: Query<&Text, Changed<Text>>,
) {
    for mut dialog in &mut q_dialogs {
        if let Some(dropdown_entity) = dialog.filter_dropdown {
            if let Ok(dropdown) = q_dropdowns.get(dropdown_entity) {
                if q_texts.get(dropdown.text_entity).is_ok() {
                    dialog.needs_refresh = true;
                }
            }
        }
    }
}
