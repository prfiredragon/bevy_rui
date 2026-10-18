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

    // INYECCIÓN EN EL ARRANQUE: Pasamos el placeholder convertido a SmolStr dentro de la cola nativa de Bevy
    let mut initial_text = EditableText::default();
    if !placeholder.is_empty() {
        initial_text.pending_edits.push(bevy::text::TextEdit::Insert(placeholder.to_string().into()));
    }

    let cmds = parent_cmd.spawn((
        s,
        font,
        color,
        bevy::ui_widgets::Button, 
        crate::focus::Focusable,
        TextInput,
        initial_text, // El buffer arranca pre-cargado con el texto de guía
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
            text: placeholder.to_string(), // Sincronizamos la variable inicial
            last_synced_text: placeholder.to_string(),
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

    // INYECCIÓN EN EL ARRANQUE: Pasamos el placeholder convertido a SmolStr dentro de la cola nativa de Bevy
    let mut initial_text = EditableText {
        visible_lines: Some(8.0),
        allow_newlines: true,
        ..default()
    };
    if !placeholder.is_empty() {
        initial_text.pending_edits.push(bevy::text::TextEdit::Insert(placeholder.to_string().into()));
    }

    let cmds = parent_cmd.spawn((
        s,
        font,
        color,
        bevy::ui_widgets::Button, 
        crate::focus::Focusable,
        TextInput,
        initial_text,
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
            text: placeholder.to_string(), // Sincronizamos la variable inicial
            last_synced_text: placeholder.to_string(),
            ..default() 
        }
    ));

    cmds
}

pub fn sync_native_textbox_buffers(
    mut q_textboxes: Query<(Entity, &mut RuiTextBox, &mut EditableText)>,
    focus: Res<bevy::input_focus::InputFocus>,
) {
    let focused_entity = focus.get();

    for (entity, mut rui_box, mut editable_text) in &mut q_textboxes {
        let is_focused = Some(entity) == focused_entity;

        // AUTOMATIZACIÓN DE PLACEHOLDER: Si el usuario selecciona el input y solo está el placeholder, limpiamos automáticamente
        if is_focused && rui_box.text == rui_box.placeholder {
            editable_text.clear();
            rui_box.text = String::new();
            rui_box.last_synced_text = String::new();
            continue;
        }

        // Si pierde el foco y quedó completamente vacío, restauramos el texto de guía como buffer nativo
        if !is_focused && rui_box.text.is_empty() && !rui_box.placeholder.is_empty() {
            editable_text.clear();
            editable_text.pending_edits.push(bevy::text::TextEdit::Insert(rui_box.placeholder.clone().into()));
            rui_box.text = rui_box.placeholder.clone();
            rui_box.last_synced_text = rui_box.placeholder.clone();
            continue;
        }

        // ACCIÓN A: El texto fue alterado o vaciado externamente por código (.clear() desde un botón)
        if rui_box.text != rui_box.last_synced_text {
            editable_text.clear();
            if !rui_box.text.is_empty() {
                let string_to_insert = rui_box.text.clone();
                editable_text.pending_edits.push(bevy::text::TextEdit::Insert(string_to_insert.into()));
            }
            rui_box.last_synced_text = rui_box.text.clone();
        } 
        // ACCIÓN B: El usuario está escribiendo de forma interactiva con el teclado físico
        else {
            let mut native_string = String::new();
            native_string.reserve(editable_text.value().into_iter().map(str::len).sum());
            for sub_str in editable_text.value() {
                native_string.push_str(sub_str);
            }
            rui_box.text = native_string.clone();
            rui_box.last_synced_text = native_string;
        }
    }
}

pub fn handle_textbox_input() {}
pub fn handle_textbox_scroll() {}
