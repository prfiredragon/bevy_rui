use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy::ui_widgets::TextInput;

#[derive(Reflect, Clone, PartialEq, Default)]
pub enum RuiTextBoxMode {
    #[default]
    SingleLine,
    MultiLine,
}

#[derive(Component)]
pub struct RuiTextBox {
    pub text: String,
    pub placeholder: String,
    pub cursor_index: usize,
    pub mode: RuiTextBoxMode,
    pub max_characters: Option<usize>,
    pub readonly: bool,
    pub last_synced_text: String,
}

impl Default for RuiTextBox {
    fn default() -> Self {
        Self {
            text: String::new(),
            placeholder: String::new(),
            cursor_index: 0,
            mode: RuiTextBoxMode::SingleLine,
            max_characters: None,
            readonly: false,
            last_synced_text: String::new(),
        }
    }
}

pub fn spawn_textbox<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    placeholder: &str,
    modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        min_height: Val::Px(36.0),
        min_width: Val::Px(100.0),
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect::all(Val::Px(8.0)), 
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        overflow: Overflow::clip(),
        ..default()
    };
    let mut font = TextFont::default();
    let mut color = TextColor(Color::WHITE);
    modifier(&mut s, &mut font, &mut color);

    let cmds = parent_cmd.spawn((
        s,
        font,
        color,
        bevy::ui_widgets::Button, 
        crate::focus::Focusable,
        TextInput,
        EditableText {
            visible_lines: Some(1.0),
            allow_newlines: false,
            ..default()
        },
        TextLayout {
            linebreak: LineBreak::WordOrCharacter,
            ..default()
        },
        TextCursorStyle {
            color: Color::WHITE,
            selected_text_color: Some(Color::BLACK),
            ..default()
        },
        crate::theme::RuiThemeElement::TextboxBg,
        BorderColor::from(Color::srgb(0.05, 0.05, 0.05)),
        RuiTextBox { 
            placeholder: placeholder.to_string(), 
            mode: RuiTextBoxMode::SingleLine, 
            text: String::new(),
            last_synced_text: String::new(),
            ..default() 
        }
    ));

    cmds
}
pub fn spawn_multiline_textbox<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    placeholder: &str,
    modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        min_height: Val::Px(80.0), 
        min_width: Val::Px(100.0),
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect::all(Val::Px(8.0)),
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        overflow: Overflow::clip(),
        ..default()
    };
    let mut font = TextFont::default();
    let mut color = TextColor(Color::WHITE);
    modifier(&mut s, &mut font, &mut color);

    let cmds = parent_cmd.spawn((
        s,
        font,
        color,
        bevy::ui_widgets::Button, 
        crate::focus::Focusable,
        TextInput,
        EditableText {
            visible_lines: Some(8.0),
            allow_newlines: true,
            ..default()
        },
        TextLayout {
            linebreak: LineBreak::WordOrCharacter,
            ..default()
        },
        TextCursorStyle {
            color: Color::WHITE,
            selected_text_color: Some(Color::BLACK),
            ..default()
        },
        crate::theme::RuiThemeElement::TextboxBg,
        BorderColor::from(Color::srgb(0.05, 0.05, 0.05)),
        RuiTextBox { 
            placeholder: placeholder.to_string(), 
            mode: RuiTextBoxMode::MultiLine, 
            text: String::new(),
            last_synced_text: String::new(),
            ..default() 
        }
    ));

    cmds
}

// CORRECCIÓN DEFINITIVA DE ENUMS: Importamos y empaquetamos el String bajo la ruta legal de bevy::clipboard
pub fn sync_native_textbox_buffers(
    mut q_textboxes: Query<(Entity, &mut RuiTextBox, &mut EditableText, &mut Text, &crate::focus::Focusable)>,
    focus: Res<bevy::input_focus::InputFocus>,
) {
    let focused_entity = focus.get();

    for (entity, mut rui_box, mut editable_text, mut visual_text, _) in &mut q_textboxes {
        let is_focused = Some(entity) == focused_entity;

        // ACCIÓN A: Si el usuario modificó la variable 'text' externamente por código (ej. clic en el cubo)
        if rui_box.text != rui_box.last_synced_text {
            editable_text.clear();
            // CORRECCIÓN NATIVA: Envolvemos con Ready(Ok(String)) usando el enum del módulo raíz de Bevy
            editable_text.pending_paste = Some(bevy::clipboard::ClipboardRead::Ready(Ok(rui_box.text.clone())));
            rui_box.last_synced_text = rui_box.text.clone();
        } 
        // ACCIÓN B: El usuario está escribiendo de forma interactiva con el teclado
        else {
            let mut native_string = String::new();
            native_string.reserve(editable_text.value().into_iter().map(str::len).sum());
            for sub_str in editable_text.value() {
                native_string.push_str(sub_str);
            }
            
            rui_box.text = native_string.clone();
            rui_box.last_synced_text = native_string;
        }

        // CONTROL DE PLACEHOLDER VISIBLE
        if rui_box.text.is_empty() && !is_focused {
            visual_text.0 = rui_box.placeholder.clone();
        } else {
            visual_text.0 = rui_box.text.clone();
        }
    }
}
pub fn handle_textbox_input() {}
pub fn handle_textbox_scroll() {}
