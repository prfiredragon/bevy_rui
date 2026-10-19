use std::path::PathBuf;
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
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
#[derive(Component, Clone)] pub enum DialogButtonAction { UpDir(Entity), CreateDir(Entity), Confirm(Entity), Cancel(Entity) }

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
        
        let mut wp = win_p.reborrow();
        wp.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, width: Val::Percent(100.0), height: Val::Px(40.0), padding: UiRect::all(Val::Px(4.0)), align_items: AlignItems::Center, ..default() }).with_children(|top_p| {
            let mut p = top_p.reborrow();
            
            // SANEAMIENTO E0499: Reborrows secuenciales independientes para la barra superior
            let mut up_bind = p.reborrow();
            up_bind.button(|s| { s.width = Val::Px(40.0); s.height = Val::Px(30.0); }, |b| { b.reborrow().label("↑", |_,_|{}).insert(RuiThemeElement::ButtonText); }).observe(move |_: On<Activate>, mut q: Query<&mut RuiFileDialog>| {
                if let Ok(mut d) = q.get_mut(dialog_id) {
                    if let Some(parent_dir) = d.current_dir.parent() {
                        if d.root_dir.as_ref().map_or(true, |r| parent_dir.starts_with(r)) { d.current_dir = parent_dir.to_path_buf(); d.needs_refresh = true; }
                    }
                }
            });
            
            let mut create_bind = p.reborrow();
            create_bind.button(|s| { s.width = Val::Px(40.0); s.height = Val::Px(30.0); s.margin = UiRect::left(Val::Px(5.0)); }, |b| { b.reborrow().label("+📁", |f,_|{ f.font_size = FontSize::Px(14.0); }).insert(RuiThemeElement::ButtonText); }).observe(move |_: On<Activate>, mut commands: Commands, mut active_scope: ResMut<crate::focus::RuiActiveScope>| {
                let modal = spawn_create_dir_modal(&mut commands, dialog_id); active_scope.push_window(modal);
            });
            
            let mut txt_bind = p.reborrow();
            txt_bind.spawn((Text::new(start_dir.display().to_string()), TextFont { font_size: FontSize::Px(16.0), ..default() }, TextColor(Color::WHITE), Node { margin: UiRect::left(Val::Px(10.0)), overflow: Overflow::clip(), flex_shrink: 1.0, min_width: Val::Px(0.0), ..default() })).insert(RuiFilePathText(dialog_id));
        });
        
        let mut list_id = Entity::PLACEHOLDER;
        let mut win_p_bind = wp.reborrow();
        spawn_scrollview(&mut win_p_bind, |s| { s.width = Val::Percent(100.0); s.height = Val::Px(300.0); s.border = UiRect::all(Val::Px(1.0)); s.margin = UiRect::vertical(Val::Px(5.0)); }, |scroll_p| {
            list_id = scroll_p.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Column, width: Val::Percent(100.0), ..default() }).id();
        }).insert(BorderColor::from(Color::BLACK));
        list_container_entity = list_id;
        
        wp.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, width: Val::Percent(100.0), height: Val::Px(40.0), padding: UiRect::all(Val::Px(4.0)), align_items: AlignItems::Center, justify_content: JustifyContent::SpaceBetween, ..default() }).with_children(|bot_p| {
            let mut bp = bot_p.reborrow();
            bp.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, flex_grow: 1.0, align_items: AlignItems::Center, ..default() }).with_children(|left_p| {
                let mut lp = left_p.reborrow();
                
                let mut tb_bind = lp.reborrow();
                textbox_entity = spawn_textbox(&mut tb_bind, "Nombre del archivo o carpeta...", |s, _, _| { s.flex_grow = 1.0; s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0)); }).id();
                
                if !filters.is_empty() {
                    let filter_strings: Vec<String> = filters.iter().map(|f| f.name.clone()).collect();
                    let str_refs: Vec<&str> = filter_strings.iter().map(|s| s.as_str()).collect();
                    let mut drop_bind = lp.reborrow();
                    filter_dropdown_entity = Some(crate::widgets::dropdown::spawn_dropdown(&mut drop_bind, &str_refs[0], &str_refs, |s| { s.width = Val::Px(200.0); s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0)); }).id());
                }
            });
            
            bp.spawn(Node { display: Display::Flex, flex_direction: FlexDirection::Row, ..default() }).with_children(|btn_p| {
                let mut btn = btn_p.reborrow();
                
                let mut cancel_bind = btn.reborrow();
                cancel_bind.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(30.0); s.margin = UiRect::right(Val::Px(10.0)); }, |b| { b.reborrow().label("Cancelar", |_,_|{}).insert(RuiThemeElement::ButtonText); }).observe(move |_: On<Activate>, mut commands: Commands, mut ev: MessageWriter<RuiFileCanceled>| {
                    ev.write(RuiFileCanceled); commands.entity(dialog_id).despawn();
                });
                
                let confirm_label = if mode == FileDialogMode::Open { "Abrir" } else { "Guardar" };
                let mut confirm_bind = btn.reborrow();
                confirm_bind.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(30.0); }, |b| { b.reborrow().label(confirm_label, |_,_|{}).insert(RuiThemeElement::ButtonText); }).observe(move |_: On<Activate>, mut commands: Commands, q_dialog: Query<&RuiFileDialog>, q_txt: Query<&RuiTextBox>, q_drop: Query<&crate::widgets::dropdown::RuiDropdown>, q_texts: Query<&Text>, mut ev: MessageWriter<RuiFileSelected>| {
                    if let Ok(dialog) = q_dialog.get(dialog_id) {
                        let mut file_name = if let Ok(tb) = q_txt.get(dialog.textbox_entity) { tb.text.clone() } else { dialog.selected_file.clone() };
                        if !file_name.is_empty() {
                            if let Some(drop_ent) = dialog.filter_dropdown {
                                if let Ok(dropdown) = q_drop.get(drop_ent) {
                                    if let Ok(drop_text) = q_texts.get(dropdown.text_entity) {
                                        if let Some(filter) = dialog.filters.iter().find(|f| &f.name == &drop_text.0) {
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
                            ev.write(RuiFileSelected { path: dialog.current_dir.join(file_name), mode: dialog.mode }); commands.entity(dialog_id).despawn();
                        }
                    }
                });
            });
        });
    });
    window_cmds.insert(RuiFileDialog { mode, current_dir: start_dir.clone(), selected_file: String::new(), filters, filter_dropdown: filter_dropdown_entity, list_container: list_container_entity, textbox_entity, needs_refresh: true, root_dir });
    window_cmds
}
fn spawn_create_dir_modal(commands: &mut Commands, dialog_entity: Entity) -> Entity {
    let mut modal_id = Entity::PLACEHOLDER;
    commands.entity(dialog_entity).with_children(|win_p| {
        modal_id = win_p.spawn((
            Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), display: Display::Flex, justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            ImageNode::default(), BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), ZIndex(1000), Pickable::default(),
        )).id();
    });
    
    let mut textbox_captured = Entity::PLACEHOLDER;
    commands.entity(modal_id).with_children(|bg_p| {
        bg_p.spawn(Node { flex_direction: FlexDirection::Column, width: Val::Px(350.0), height: Val::Px(200.0), padding: UiRect::all(Val::Px(15.0)), border: UiRect::all(Val::Px(2.0)), ..default() }).with_children(|panel| {
            let mut p = panel.reborrow();
            
            let mut lbl_bind = p.reborrow();
            lbl_bind.spawn((Text::new("Crear Nuevo Directorio"), TextFont { font_size: FontSize::Px(18.0), ..default() }, TextColor(Color::WHITE)));
            
            let mut txt_bind = p.reborrow();
            textbox_captured = spawn_textbox(&mut txt_bind, "Nombre de la carpeta", |s, _, _| { s.width = Val::Percent(100.0); s.height = Val::Px(35.0); s.margin = UiRect::vertical(Val::Px(15.0)); }).insert((crate::focus::Focusable, bevy::ui_widgets::Button)).id();
            
            let mut box_bind = p.reborrow();
            box_bind.spawn(Node { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceEvenly, ..default() }).with_children(|buttons| {
                let mut b = buttons.reborrow();
                
                let mut cancel_bind = b.reborrow();
                cancel_bind.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(35.0); }, |inner_b| { inner_b.reborrow().label("Cancelar", |_,_|{}).insert(RuiThemeElement::ButtonText); }).observe(move |_: On<Activate>, mut commands: Commands, mut active_scope: ResMut<crate::focus::RuiActiveScope>| {
                    active_scope.remove_window(modal_id); commands.entity(modal_id).despawn();
                });
                
                let mut confirm_bind = b.reborrow();
                confirm_bind.button(|s| { s.width = Val::Px(80.0); s.height = Val::Px(35.0); }, |inner_b| { inner_b.reborrow().label("Crear", |_,_|{}).insert(RuiThemeElement::ButtonText); }).observe(move |_: On<Activate>, mut commands: Commands, mut q_dialog: Query<&mut RuiFileDialog>, q_txt: Query<&RuiTextBox>, mut active_scope: ResMut<crate::focus::RuiActiveScope>| {
                    if let Ok(textbox) = q_txt.get(textbox_captured) {
                        let dir_name = textbox.text.trim().to_string();
                        if !dir_name.is_empty() {
                            if let Ok(mut dialog) = q_dialog.get_mut(dialog_entity) { let _ = std::fs::create_dir_all(dialog.current_dir.join(&dir_name)); dialog.needs_refresh = true; }
                        }
                    }
                    active_scope.remove_window(modal_id); commands.entity(modal_id).despawn();
                });
            });
        }).insert((crate::theme::RuiThemeElement::Panel, BorderColor::from(Color::srgb(0.6, 0.6, 0.6)), Pickable::default(), ImageNode::default(), BackgroundColor(Color::srgb(0.12, 0.12, 0.12))));
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
                    let mut btn_cmds = item_spawner.button(|s| { s.width = Val::Percent(100.0); s.height = Val::Px(30.0); s.justify_content = JustifyContent::FlexStart; s.align_items = AlignItems::Center; s.padding = UiRect::all(Val::Px(4.0)); }, |_| {});
                    
                    let dialog_ent = dialog_entity;
                    let p_copy = path.clone();
                    btn_cmds.observe(move |_: On<Activate>, mut q_dial: Query<&mut RuiFileDialog>, mut q_tb: Query<&mut RuiTextBox>| {
                        if let Ok(mut dial) = q_dial.get_mut(dialog_ent) {
                            if is_dir { dial.current_dir = p_copy.clone(); dial.needs_refresh = true; } else {
                                if let Some(f_name) = p_copy.file_name() {
                                    dial.selected_file = f_name.to_string_lossy().to_string();
                                    if let Ok(mut textbox) = q_tb.get_mut(dial.textbox_entity) { textbox.text = dial.selected_file.clone(); textbox.cursor_index = textbox.text.chars().count(); }
                                    dial.needs_refresh = true;
                                }
                            }
                        }
                    });
                    
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

pub fn handle_dropdown_change(mut q_dialogs: Query<&mut RuiFileDialog>, q_dropdowns: Query<&crate::widgets::dropdown::RuiDropdown>, q_texts: Query<&Text, Changed<Text>>) {
    for mut dialog in &mut q_dialogs {
        if let Some(dropdown_entity) = dialog.filter_dropdown {
            if let Ok(dropdown) = q_dropdowns.get(dropdown_entity) {
                if q_texts.get(dropdown.text_entity).is_ok() { dialog.needs_refresh = true; }
            }
        }
    }
}

pub fn handle_file_clicks() {}
pub fn handle_dialog_buttons() {}
