//! Single-line and multi-line editable text box widgets built on Bevy 0.20's native `TextInput` and `EditableText`.

use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy::ui_widgets::TextInput;

/// Line mode for a [`RuiTextBox`].
#[derive(Reflect, Clone, PartialEq, Default)]
pub enum RuiTextBoxMode {
    /// Single-line text input (Enter does not insert newlines).
    #[default]
    SingleLine,
    /// Multi-line text input (`allow_newlines: true`).
    MultiLine,
}

/// Component storing the synchronized text string, placeholder, and configuration of a text input box.
///
/// Mutating `RuiTextBox::text` from external systems (e.g., clearing it on button click) will automatically
/// update the underlying native `EditableText` buffer in [`sync_native_textbox_buffers`].
#[derive(Component)]
pub struct RuiTextBox {
    /// Current text content synchronized bidirectionally with Bevy's `EditableText`.
    pub text: String,
    /// Dimmed placeholder text displayed when `text` is empty and the widget is not focused.
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

/// Marker component for the visual-only placeholder child node inside a [`RuiTextBox`].
/// Never part of the `EditableText` buffer or `RuiTextBox::text`.
#[derive(Component)]
pub struct RuiTextBoxPlaceholder;

fn spawn_placeholder_child(cmds: &mut EntityCommands, placeholder: &str, font: &TextFont, color: &TextColor, padding: f32) {
    if placeholder.is_empty() {
        return;
    }
    let dim = color.0.with_alpha(color.0.alpha() * 0.45);
    let text = placeholder.to_string();
    let font = font.clone();
    cmds.with_children(move |p| {
        p.spawn((
            Text::new(text),
            font,
            TextColor(dim),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(padding),
                top: Val::Px(padding),
                right: Val::Px(padding),
                overflow: Overflow::clip(),
                ..default()
            },
            Pickable::IGNORE,
            RuiTextBoxPlaceholder,
        ));
    });
}

/// Spawns a single-line editable text box with `placeholder` text.
///
/// Read or modify the current text via the [`RuiTextBox`] component.
/// Also available via [`crate::widgets::RuiBuilderExt::textbox`].
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
    let pad = if let Val::Px(v) = s.padding.left { v } else { 8.0 };
    let (f2, c2) = (font.clone(), color.clone());

    // El buffer arranca VACÍO: el placeholder se dibuja como un nodo hijo superpuesto.
    let mut cmds = parent_cmd.spawn((
        s,
        font,
        color,
        bevy::ui_widgets::Button, 
        crate::focus::Focusable,
        TextInput,
        EditableText::default(),
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
            ..default() 
        }
    ));
    spawn_placeholder_child(&mut cmds, placeholder, &f2, &c2, pad);

    cmds
}
/// Spawns a multi-line editable text box (`allow_newlines: true`) with `placeholder` text.
///
/// Also available via [`crate::widgets::RuiBuilderExt::multiline_textbox`].
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
    let pad = if let Val::Px(v) = s.padding.left { v } else { 8.0 };
    let (f2, c2) = (font.clone(), color.clone());

    let mut cmds = parent_cmd.spawn((
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
            ..default() 
        }
    ));
    spawn_placeholder_child(&mut cmds, placeholder, &f2, &c2, pad);

    cmds
}

/// System that bidirectionally synchronizes [`RuiTextBox::text`] with Bevy's native [`EditableText`]
/// and toggles [`RuiTextBoxPlaceholder`] visibility.
pub fn sync_native_textbox_buffers(
    mut q_textboxes: Query<(Entity, &mut RuiTextBox, &mut EditableText, Option<&Children>)>,
    mut q_placeholders: Query<&mut Visibility, With<RuiTextBoxPlaceholder>>,
    focus: Res<bevy::input_focus::InputFocus>,
) {
    let focused_entity = focus.get();

    for (entity, mut rui_box, mut editable_text, children) in &mut q_textboxes {
        let is_focused = Some(entity) == focused_entity;

        // ACCIÓN A: El texto fue alterado o vaciado externamente por código (.clear() desde un botón)
        if rui_box.text != rui_box.last_synced_text {
            editable_text.clear();
            if !rui_box.text.is_empty() {
                let string_to_insert = rui_box.text.clone();
                editable_text.pending_edits.push(bevy::text::TextEdit::Insert(string_to_insert.into()));
            }
            rui_box.last_synced_text = rui_box.text.clone();
        } 
        // ACCIÓN B: El usuario está escribiendo de forma interactiva con el teclado físico.
        // Esperamos a que Bevy aplique las ediciones pendientes para no leer un buffer desfasado.
        else if editable_text.pending_edits.is_empty() {
            let mut native_string = String::new();
            for sub_str in editable_text.value() {
                native_string.push_str(sub_str);
            }
            if native_string != rui_box.text {
                rui_box.text = native_string.clone();
                rui_box.last_synced_text = native_string;
            }
        }

        // PLACEHOLDER VISUAL: visible solo si está vacío y sin foco.
        let show = rui_box.text.is_empty() && !is_focused;
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut vis) = q_placeholders.get_mut(child) {
                    let target = if show { Visibility::Inherited } else { Visibility::Hidden };
                    if *vis != target { *vis = target; }
                }
            }
        }
    }
}

/// Placeholder system kept for backwards compatibility.
pub fn handle_textbox_input() {}
/// Placeholder system kept for backwards compatibility.
pub fn handle_textbox_scroll() {}
