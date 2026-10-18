use std::path::PathBuf;
use bevy::prelude::*;
use bevy::text::FontSize;
use crate::widgets::textbox::{spawn_textbox, RuiTextBox};
use crate::widgets::scrollview::spawn_scrollview;
use crate::theme::RuiThemeElement;
use crate::widgets::RuiBuilderExt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileDialogMode { Open, Save }

#[derive(Clone, Debug, PartialEq)]
pub struct FileFilter { pub name: String, pub extensions: Vec<String> }

#[derive(Component)]
pub struct RuiFileDialog {
    pub mode: FileDialogMode, pub current_dir: PathBuf, pub selected_file: String,
    pub filters: Vec<FileFilter>, pub filter_dropdown: Option<Entity>, pub list_container: Entity,
    pub textbox_entity: Entity, pub needs_refresh: bool, pub root_dir: Option<PathBuf>,
}

#[derive(Component, Clone)] pub struct RuiFileItem { pub path: PathBuf, pub is_dir: bool, pub dialog_entity: Entity }
#[derive(Component)] pub struct RuiFilePathText(pub Entity);
#[derive(Component, Clone)] pub enum DialogButtonAction { UpDir(Entity), CreateDir(Entity), Confirm(Entity), Cancel(Entity), ConfirmCreateDir(Entity, Entity, Entity), CancelCreateDir(Entity) }

#[derive(Message, Debug)] pub struct RuiFileSelected { pub path: PathBuf, pub mode: FileDialogMode }
#[derive(Message, Debug)] pub struct RuiFileCanceled;

pub fn spawn_file_dialog<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    title: &str,
    mode: FileDialogMode,
    start_dir: PathBuf,
    root_dir: Option<PathBuf>,
    filters: Vec<FileFilter>,
) -> EntityCommands<'w> {
    let mut list_container_entity = Entity::PLACEHOLDER;
    let mut textbox_entity = Entity::PLACEHOLDER;
    let mut filter_dropdown_entity = None;
    
    let mut window_cmds = parent.window(title, true, |s| {
        s.width = Val::Px(650.0); s.height = Val::Px(450.0);
        s.left = Val::Px(200.0);  s.top = Val::Px(150.0);
    }, |win_p, dialog_id| {
        
        // 1. Top Bar
        win_p.spawn(Node {
            display: Display::Flex, flex_direction: FlexDirection::Row,
            width: Val::Percent(100.0), height: Val::Px(40.0), padding: UiRect::all(Val::Px(4.0)), align_items: AlignItems::Center, ..default()
        }).with_children(|top_p| {
            let mut p = top_p.reborrow();
            
            let mut b1 = p.reborrow();
            b1.button(|s| { s.width = Val::Px(40.0); s.height = Val::Px(30.0); }, |b| {
                let mut inner = b.reborrow();
                inner.label("↑", |_,_|{}).insert(RuiThemeElement::ButtonText);
            }).insert(DialogButtonAction::UpDir(dialog_id));
            
            let mut b2 = p.reborrow();
            b2.button(|s| { s.width = Val::Px(40.0); s.height = Val::Px(30.0); s.margin = UiRect::left(Val::Px(5.0)); }, |b| {
                let mut inner = b.reborrow();
                inner.label("+📁", |font,_|{ font.font_size = FontSize::Px(14.0); }).insert(RuiThemeElement::ButtonText);
            }).insert(DialogButtonAction::CreateDir(dialog_id));
            
            let mut b3 = p.reborrow();
            b3.spawn((Text::new(start_dir.display().to_string()), TextFont { font_size: FontSize::Px(16.0), ..default() }, TextColor(Color::WHITE), Node { margin: UiRect::left(Val::Px(10.0)), overflow: Overflow::clip(), flex_shrink: 1.0, min_width: Val::Px(0.0), ..default() })).insert(RuiFilePathText(dialog_id));
        });
        
        // 2. File List ScrollView
        let mut list_container_id = Entity::PLACEHOLDER;
        let mut scroll_spawner = win_p.reborrow();
        let mut scroll_cmds = spawn_scrollview(&mut scroll_spawner, |s| {
            s.width = Val::Percent(100.0); s.height = Val::Px(300.0);
            s.border = UiRect::all(Val::Px(1.0)); s.margin = UiRect::vertical(Val::Px(5.0));
        }, |scroll_p| {
            list_container_id = scroll_p.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Column, width: Val::Percent(100.0), ..default() }).id();
        });
        scroll_cmds.insert(BorderColor::from(Color::BLACK));
        list_container_entity = list_container_id;
        
        // 3. Bottom Bar
        win_p.spawn(Node {
            display: Display::Flex, flex_direction: FlexDirection::Row, width: Val::Percent(100.0), height: Val::Px(40.0), padding: UiRect::all(Val::Px(4.0)), align_items: AlignItems::Center, justify_content: JustifyContent::SpaceBetween, ..default()
        }).with_children(|bot_p| {
            bot_p.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, flex_grow: 1.0, align_items: AlignItems::Center, ..default() }).with_children(|left_p| {
                left_p.spawn(Node { display: Display::Flex, width: Val::Percent(100.0), ..default() }).with_children(|sub_left| {
                    let mut p = sub_left.reborrow();
                    
                    let mut txt_binding = p.reborrow();
                    textbox_entity = spawn_textbox(&mut txt_binding, "Nombre del archivo o carpeta...", |s, _, _| {
                        s.flex_grow = 1.0; s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0));
                    }).id();
                    
                    if !filters.is_empty() {
                        let filter_strings: Vec<String> = filters.iter().map(|f| f.name.clone()).collect();
                        let str_refs: Vec<&str> = filter_strings.iter().map(|s| s.as_str()).collect();
                        
                        let mut drop_binding = p.reborrow();
                        let dropdown_ent = crate::widgets::dropdown::spawn_dropdown(&mut drop_binding, &str_refs[0], &str_refs, |s| {
                            s.width = Val::Px(200.0); s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0));
                        }).id();
                        filter_dropdown_entity = Some(dropdown_ent);
                    }
                });
            });
            
            bot_p.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, ..default() }).with_children(|btn_p| {
                let mut p = btn_p.reborrow();
                
                let mut btn1 = p.reborrow();
                btn1.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0)); }, |b| {
                    let mut inner = b.reborrow();
                    inner.label("Cancelar", |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::Cancel(dialog_id));
                
                let confirm_label = if mode == FileDialogMode::Open { "Abrir" } else { "Guardar" };
                let mut btn2 = p.reborrow();
                btn2.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(30.0); }, |b| {
                    let mut inner = b.reborrow();
                    inner.label(confirm_label, |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::Confirm(dialog_id));
            });
        });
    });
    
    window_cmds.insert(RuiFileDialog {
        mode, current_dir: start_dir.clone(), selected_file: String::new(), filters,
        filter_dropdown: filter_dropdown_entity, list_container: list_container_entity,
        textbox_entity, needs_refresh: true, root_dir,
    });
    window_cmds
}


fn spawn_create_dir_modal(commands: &mut Commands, dialog_entity: Entity) -> Entity {
    let mut modal_id = Entity::PLACEHOLDER;
    
    commands.entity(dialog_entity).with_children(|win_p| {
        modal_id = win_p.spawn((
            Node {
                position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0),
                display: Display::Flex, justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), ZIndex(1000), Pickable::default(),
        )).id();
    });
    
    let mut textbox_captured = Entity::PLACEHOLDER;
    commands.entity(modal_id).with_children(|bg_p| {
        let mut bg_spawner = bg_p.reborrow();
        bg_spawner.vbox(|s| {
            s.width = Val::Px(350.0); s.height = Val::Px(200.0); s.left = Val::Auto; s.top = Val::Auto;  
            s.padding = UiRect::all(Val::Px(15.0)); s.border = UiRect::all(Val::Px(2.0)); s.border_radius = BorderRadius::all(Val::Px(6.0));
        }, |panel| {
            let mut p = panel.reborrow();
            
            let mut l_binding = p.reborrow();
            l_binding.label("Crear Nuevo Directorio", |font, _| { font.font_size = FontSize::Px(18.0);});
            
            let mut txt_binding = p.reborrow();
            textbox_captured = spawn_textbox(&mut txt_binding, "Nombre de la carpeta", |s, _, _| {
                s.width = Val::Percent(80.0); s.height = Val::Px(35.0); s.left = Val::Percent(10.0); s.margin = UiRect::vertical(Val::Px(10.0));
            }).insert(RuiThemeElement::TextboxBg).id();
            
            let mut box_binding = p.reborrow();
            box_binding.hbox(|s| { s.width = Val::Percent(100.0); s.justify_content = JustifyContent::SpaceEvenly; }, |buttons| {
                let mut b = buttons.reborrow();
                
                let mut b_cancel = b.reborrow();
                b_cancel.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(35.0); }, |inner_b| {
                    let mut inner = inner_b.reborrow();
                    inner.label("Cancelar", |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::CancelCreateDir(modal_id));
                
                let mut b_confirm = b.reborrow();
                b_confirm.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(35.0); }, |inner_b| {
                    let mut inner = inner_b.reborrow();
                    inner.label("Crear", |_,_|{}).insert(RuiThemeElement::ButtonText);
                }).insert(DialogButtonAction::ConfirmCreateDir(dialog_entity, modal_id, textbox_captured));
            });
        }).insert((
            crate::theme::RuiThemeElement::Panel, BorderColor::from(Color::srgb(0.6, 0.6, 0.6)), Pickable::default(), ImageNode::default(), BackgroundColor(Color::NONE),
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
        if !dialog.needs_refresh { continue; }
        dialog.needs_refresh = false;
        
        for (path_text_comp, mut text) in &mut q_path_texts {
            if path_text_comp.0 == dialog_entity { text.0 = dialog.current_dir.display().to_string(); }
        }
        
        let container = dialog.list_container;
        if let Ok(children) = q_children.get(container) {
            for child in children { commands.entity(*child).try_despawn(); }
        }
        
        let mut active_extensions: Vec<String> = vec![];
        if let Some(dropdown_entity) = dialog.filter_dropdown {
            if let Ok(dropdown) = q_dropdowns.get(dropdown_entity) {
                if let Ok(dropdown_text) = q_texts.get(dropdown.text_entity) {
                    let selected_filter_name = &dropdown_text.0;
                    if let Some(filter) = dialog.filters.iter().find(|f| &f.name == selected_filter_name) { active_extensions = filter.extensions.clone(); }
                }
            }
        }

        let mut entries = Vec::new();
        if let Ok(read_dir) = std::fs::read_dir(&dialog.current_dir) {
            for entry in read_dir.flatten() {
                let path = entry.path(); let is_dir = path.is_dir();
                let name = entry.file_name().to_string_lossy().to_string();
                
                if !is_dir && !active_extensions.is_empty() {
                    let mut matches = false;
                    for ext in &active_extensions {
                        if ext == "*" || ext == "*.*" { matches = true; break; }
                        let clean_ext = ext.strip_prefix("*.").unwrap_or(ext);
                        let suffix = format!(".{}", clean_ext.trim_start_matches('.'));
                        if name.to_lowercase().ends_with(&suffix.to_lowercase()) { matches = true; break; }
                    }
                    if !matches { continue; }
                }
                entries.push((name, path, is_dir));
            }
        }
        
        entries.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
        
        if let Ok(mut entity_cmds) = commands.get_entity(container) {
            entity_cmds.with_children(|p| {
                let mut list_spawner = p.reborrow();
                for (name, path, is_dir) in entries {
                    let icon_path = if is_dir { "icons/folder.png" } else {
                        match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
                            "png" | "jpg" | "jpeg" | "gif" | "bmp" => "icons/image.png",
                            "rs" | "toml" | "json" => "icons/code.png",
                            "mp3" | "wav" | "ogg" => "icons/audio.png",
                            _ => "icons/file.png",
                        }
                    };
                    
                    let texture = asset_server.load(icon_path);
                    let mut item_spawner = list_spawner.reborrow();
                    let mut btn_cmds = item_spawner.button(|s| {
                        s.width = Val::Percent(100.0); s.height = Val::Px(30.0);
                        s.justify_content = JustifyContent::FlexStart; s.align_items = AlignItems::Center;
                        s.padding = UiRect::all(Val::Px(4.0));
                    }, |_| {});
                    
                    btn_cmds.with_children(|b_p| {
                        b_p.spawn((ImageNode::new(texture), Node { width: Val::Px(20.0), height: Val::Px(20.0), margin: UiRect::right(Val::Px(8.0)), ..default() }));
                        b_p.spawn((Text::new(name.clone()), TextFont { font_size: FontSize::Px(16.0), ..default() }, TextColor(Color::WHITE)));
                    });
                    
                    btn_cmds.insert((RuiFileItem { path, is_dir, dialog_entity }, crate::theme::RuiThemeElement::ListItem));
                    if !is_dir && name == dialog.selected_file { btn_cmds.insert(crate::widgets::button::RuiSelected); }
                }
            });
        }
    }
}


pub fn handle_file_clicks(
    q_interactions: Query<(&bevy::ui::Pressed, &RuiFileItem)>,
    mut q_dialogs: Query<(Entity, &mut RuiFileDialog)>,
    mut q_textboxes: Query<&mut RuiTextBox>,
) {
    for (_, item) in &q_interactions {
        for (dialog_entity, mut dialog) in &mut q_dialogs {
            if dialog_entity != item.dialog_entity { continue; }
            
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

pub fn handle_dialog_buttons(
    mut commands: Commands,
    q_interactions: Query<(&bevy::ui::Pressed, &DialogButtonAction)>,
    mut q_dialogs: Query<(Entity, &mut RuiFileDialog)>,
    q_textboxes: Query<&RuiTextBox>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
    mut ev_selected: MessageWriter<RuiFileSelected>,
    mut ev_canceled: MessageWriter<RuiFileCanceled>,
    q_dropdowns: Query<&crate::widgets::dropdown::RuiDropdown>,
    q_texts: Query<&Text>,
) {
    for (_, action) in &q_interactions {
        match action {
            DialogButtonAction::CancelCreateDir(modal_ent) => {
                active_scope.remove_window(*modal_ent);
                commands.entity(*modal_ent).despawn();
                continue;
            }
            DialogButtonAction::ConfirmCreateDir(d_ent, modal_ent, txt_ent) => {
                let mut dir_name = String::new();
                if let Ok(textbox) = q_textboxes.get(*txt_ent) { dir_name = textbox.text.trim().to_string(); }
                if !dir_name.is_empty() {
                    if let Ok((_, mut dialog)) = q_dialogs.get_mut(*d_ent) {
                        let new_dir = dialog.current_dir.join(&dir_name);
                        let _ = std::fs::create_dir_all(&new_dir);
                        dialog.needs_refresh = true;
                    }
                }
                active_scope.remove_window(*modal_ent);
                commands.entity(*modal_ent).despawn();
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
            if dialog_entity != target_dialog { continue; }
            
            match action {
                DialogButtonAction::UpDir(_) => {
                    if let Some(parent_dir) = dialog.current_dir.parent() {
                        let can_go_up = if let Some(ref root) = dialog.root_dir { parent_dir.starts_with(root) } else { true };
                        if can_go_up {
                            dialog.current_dir = parent_dir.to_path_buf();
                            dialog.needs_refresh = true;
                        }
                    }
                }
                DialogButtonAction::Confirm(_) => {
                    let mut file_name = if let Ok(textbox) = q_textboxes.get(dialog.textbox_entity) { textbox.text.clone() } else { dialog.selected_file.clone() };
                    
                    if !file_name.is_empty() {
                        if let Some(dropdown_entity) = dialog.filter_dropdown {
                            if let Ok(dropdown) = q_dropdowns.get(dropdown_entity) {
                                if let Ok(dropdown_text) = q_texts.get(dropdown.text_entity) {
                                    let selected_filter_name = &dropdown_text.0;
                                    if let Some(filter) = dialog.filters.iter().find(|f| &f.name == selected_filter_name) {
                                        if let Some(first_ext) = filter.extensions.first() {
                                            if first_ext != "*" && first_ext != "*.*" {
                                                let clean_ext = first_ext.strip_prefix("*.").unwrap_or(first_ext);
                                                let suffix = format!(".{}", clean_ext.trim_start_matches('.'));
                                                if !file_name.to_lowercase().ends_with(&suffix.to_lowercase()) { file_name.push_str(&suffix); }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    
                        let path = dialog.current_dir.join(file_name);
                        ev_selected.write(RuiFileSelected { path, mode: dialog.mode });
                        commands.entity(dialog_entity).despawn();
                    }
                }
                DialogButtonAction::Cancel(_) => {
                    ev_canceled.write(RuiFileCanceled);
                    commands.entity(dialog_entity).despawn();
                }
                _ => {}
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
                if q_texts.get(dropdown.text_entity).is_ok() { dialog.needs_refresh = true; }
            }
        }
    }
}
