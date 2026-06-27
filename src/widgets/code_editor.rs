use bevy::prelude::*;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

#[derive(Resource)]
pub struct RuiSyntaxAssets {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub default_theme: String,
}

impl Default for RuiSyntaxAssets {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            // Puedes usar "base16-ocean.dark", "base16-eighties.dark", "InspiredGitHub", etc.
            default_theme: "base16-ocean.dark".to_string(), 
        }
    }
}


use ropey::Rope;

#[derive(Clone, Default)]
pub struct EditorState {
    pub text: Rope,
    pub cursor_index: usize,
    pub selection: Option<(usize, usize)>,
}

#[derive(Component)]
pub struct RuiCodeEditor {
    pub text: Rope,
    pub language: String, // ej: "rs" para Rust, "py" para Python
    pub placeholder: String,
    pub cursor_timer: Timer,
    pub show_cursor: bool,
    pub cursor_index: usize,
    pub scroll_offset: Vec2,
    pub readonly: bool,
    pub selection: Option<(usize, usize)>,
    pub selection_anchor: Option<usize>,
    pub cursor_moved: bool,
    pub dragging_v_scroll: bool,
    pub dragging_h_scroll: bool,
    pub previous_cursor_index: usize,
    pub previous_text_len: usize,
    pub previous_focused: bool,
    pub undo_stack: Vec<EditorState>,
    pub redo_stack: Vec<EditorState>,
}

impl Default for RuiCodeEditor {
    fn default() -> Self {
        Self {
            text: Rope::new(),
            language: "rs".to_string(), // Por defecto Rust
            placeholder: String::new(),
            cursor_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            show_cursor: false,
            cursor_index: 0,
            scroll_offset: Vec2::ZERO,
            readonly: false,
            selection: None,
            selection_anchor: None,
            cursor_moved: false,
            dragging_v_scroll: false,
            dragging_h_scroll: false,
            previous_cursor_index: 0,
            previous_text_len: 0,
            previous_focused: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
}


use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

// Componentes marcadores específicos para el Code Editor
#[derive(Component)]
pub struct RuiCodeEditorGutter;

#[derive(Component)]
pub struct RuiCodeEditorText;

#[derive(Component)]
pub struct RuiCodeEditorSelection;

#[derive(Component)]
pub struct RuiCodeEditorCursor;



pub fn spawn_code_editor<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    placeholder: &str,
    language: &str,
    modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor),
) -> EntityCommands<'a> {
    // 1. Configuración del Contenedor Raíz (Row)
    let mut root_node = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        min_height: Val::Px(200.0), // Un code editor suele ser más grande
        min_width: Val::Px(300.0),
        border: UiRect::all(Val::Px(2.0)),
        overflow: Overflow::clip(), // Esencial para esconder lo que desborda
        ..default()
    };
    
    let mut font = TextFont::default();
    let mut color = TextColor(Color::WHITE);
    modifier(&mut root_node, &mut font, &mut color);

    let mut cmds = parent_cmd.spawn((
        root_node,
        Button,
        crate::focus::Focusable,
        bevy::ui::FocusPolicy::Block,
        Pickable::default(),
        ImageNode { 
            visual_box: bevy::ui::VisualBox::BorderBox, 
            image_mode: bevy::ui::widget::NodeImageMode::Stretch, 
            ..ImageNode::default() 
        },
        crate::theme::RuiThemeElement::TextboxBg, // Puedes crear un ThemeElement::CodeEditorBg luego
        BorderColor::all(Color::srgb(0.051, 0.051, 0.051)),
        RelativeCursorPosition::default(),
        RuiCodeEditor { 
            placeholder: placeholder.to_string(), 
            language: language.to_string(),
            ..default() 
        },
    ));

    cmds.with_children(|parent| {
        
        // --- PANEL IZQUIERDO: EL GUTTER ---
        parent.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Px(45.0), // Ancho inicial para los números
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd, // Alinea los números a la derecha
                padding: UiRect::right(Val::Px(8.0)), // Separación con el código
                overflow: Overflow::clip(),
                ..default()
            },
            // Color de fondo ligeramente más claro/oscuro para diferenciar el gutter
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)), 
        )).with_children(|gutter| {
            gutter.spawn((
                Text::new("1"), // Iniciamos con la línea 1
                font.clone(),
                TextColor(Color::srgb(0.4, 0.4, 0.4)), // Gris atenuado
                RuiCodeEditorGutter,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.0), // Mismo padding top que el área de código
                    ..default()
                },
                Pickable::IGNORE,
            ));
        });

        // --- PANEL DERECHO: ÁREA DE CÓDIGO ---
        parent.spawn((
            Node {
                display: Display::Flex,
                flex_grow: 1.0, // Toma todo el espacio restante
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                overflow: Overflow::clip(),
                ..default()
            },
        )).with_children(|editor_area| {
            // El Texto Principal (Donde inyectaremos los TextSpans)
            editor_area.spawn((
                Text::new(placeholder),
                font,
                color,
                RuiCodeEditorText,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    ..default()
                },
                Pickable::IGNORE,
            ));

            // Cursor
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(2.0),
                    height: Val::Px(20.0),
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    ..default()
                },
                ImageNode::solid_color(Color::WHITE),
                Visibility::Hidden,
                RuiCodeEditorCursor,
                Pickable::IGNORE,
            ));

            // Highlight de Selección
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    width: Val::Px(0.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                ImageNode::solid_color(Color::srgba(0.2, 0.4, 0.8, 0.5)),
                Visibility::Hidden,
                RuiCodeEditorSelection,
                Pickable::IGNORE,
            ));

            // Scrollbars (Reutilizando tus componentes)
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    width: Val::Px(6.0),
                    height: Val::Percent(0.0),
                    ..default()
                },
                ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                Visibility::Hidden,
                crate::widgets::scrollview::RuiVerticalScrollbar,
                Pickable::IGNORE,
            ));
            
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                    width: Val::Percent(0.0),
                    height: Val::Px(6.0),
                    ..default()
                },
                ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                Visibility::Hidden,
                crate::widgets::scrollview::RuiHorizontalScrollbar,
                Pickable::IGNORE,
            ));
        });
    });

    cmds
}

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::input_focus::{InputFocus};
#[cfg(not(target_arch = "wasm32"))]
use crate::widgets::RuiClipboard;
// Asegúrate de importar EditorState y RuiCodeEditor de donde los definiste

pub fn handle_code_editor_input(
    mut events: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    mut query: Query<&mut RuiCodeEditor>,
    #[cfg(not(target_arch = "wasm32"))]
    mut clipboard: NonSendMut<RuiClipboard>,
) {
    let Some(focused_entity) = focus.get() else { return };
    let Ok(mut editor) = query.get_mut(focused_entity) else { return };

    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight]);
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let mut changed_text = false;
    let mut changed_cursor = false;
    
    // Con Rope, ya no clonamos todo el texto a un Vec<char>
    let mut cursor = editor.cursor_index.clamp(0, editor.text.len_chars());
    let old_cursor = cursor;
    
    let old_state = EditorState {
        text: editor.text.clone(), // Rope es muy barato de clonar (comparte memoria inmutablemente)
        cursor_index: editor.cursor_index,
        selection: editor.selection,
    };
    let mut is_undo_redo = false;

    // --- UNDO / REDO / COPY / CUT / SELECT ALL ---
    if ctrl {
        if keys.just_pressed(KeyCode::KeyZ) && !editor.readonly {
            if let Some(state) = editor.undo_stack.pop() {
                editor.redo_stack.push(old_state.clone());
                editor.text = state.text;
                cursor = state.cursor_index;
                editor.selection = state.selection;
                editor.selection_anchor = None;
                changed_text = true;
                is_undo_redo = true;
            }
        } else if keys.just_pressed(KeyCode::KeyY) && !editor.readonly {
            if let Some(state) = editor.redo_stack.pop() {
                editor.undo_stack.push(old_state.clone());
                editor.text = state.text;
                cursor = state.cursor_index;
                editor.selection = state.selection;
                editor.selection_anchor = None;
                changed_text = true;
                is_undo_redo = true;
            }
        }

        if keys.just_pressed(KeyCode::KeyA) {
            let len = editor.text.len_chars();
            editor.selection_anchor = Some(0);
            editor.selection = Some((0, len));
            cursor = len;
            changed_cursor = true;
        }
        if keys.just_pressed(KeyCode::KeyC) || (!editor.readonly && keys.just_pressed(KeyCode::KeyX)) {
            if let Some((start, end)) = editor.selection {
                let min = start.min(end);
                let max = start.max(end);
                // Rope permite extraer slices eficientemente
                let copied_text = editor.text.slice(min..max).to_string(); 
                #[cfg(not(target_arch = "wasm32"))]
                clipboard.set_text(copied_text);
                
                if keys.just_pressed(KeyCode::KeyX) && !editor.readonly {
                    editor.text.remove(min..max);
                    cursor = min;
                    editor.selection = None;
                    editor.selection_anchor = None;
                    changed_text = true;
                }
            }
        }
    }

    // --- NAVEGACIÓN ---
    let mut nav_pressed = false;
    let max_chars = editor.text.len_chars();
    
    if keys.just_pressed(KeyCode::ArrowLeft) {
        nav_pressed = true;
        if !shift && editor.selection.is_some() {
            let (start, end) = editor.selection.unwrap();
            cursor = start.min(end);
        } else if cursor > 0 { cursor -= 1; }
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        nav_pressed = true;
        if !shift && editor.selection.is_some() {
            let (start, end) = editor.selection.unwrap();
            cursor = start.max(end);
        } else if cursor < max_chars { cursor += 1; }
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        nav_pressed = true;
        let current_line_idx = editor.text.char_to_line(cursor);
        if current_line_idx > 0 {
            let current_col = cursor - editor.text.line_to_char(current_line_idx);
            let prev_line_char_idx = editor.text.line_to_char(current_line_idx - 1);
            let prev_line_len = editor.text.line(current_line_idx - 1).len_chars() - 1; // -1 por el \n
            cursor = prev_line_char_idx + current_col.min(prev_line_len.max(0));
        } else { cursor = 0; }
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        nav_pressed = true;
        let current_line_idx = editor.text.char_to_line(cursor);
        if current_line_idx + 1 < editor.text.len_lines() {
            let current_col = cursor - editor.text.line_to_char(current_line_idx);
            let next_line_char_idx = editor.text.line_to_char(current_line_idx + 1);
            let next_line_len = editor.text.line(current_line_idx + 1).len_chars() - 1; 
            cursor = next_line_char_idx + current_col.min(next_line_len.max(0));
        } else { cursor = max_chars; }
    } else if keys.just_pressed(KeyCode::Home) { 
        nav_pressed = true; 
        let line_idx = editor.text.char_to_line(cursor);
        cursor = editor.text.line_to_char(line_idx);
    } else if keys.just_pressed(KeyCode::End) { 
        nav_pressed = true; 
        let line_idx = editor.text.char_to_line(cursor);
        let line_start = editor.text.line_to_char(line_idx);
        let line_len = editor.text.line(line_idx).len_chars();
        cursor = line_start + line_len.saturating_sub(1); // Excluir el salto de línea
    }

    if nav_pressed {
        changed_cursor = true;
        if shift {
            if editor.selection.is_none() || editor.selection_anchor.is_none() { editor.selection_anchor = Some(old_cursor); }
            let anchor = editor.selection_anchor.unwrap();
            editor.selection = if anchor != cursor { Some((anchor, cursor)) } else { None };
        } else {
            editor.selection = None;
            editor.selection_anchor = None;
        }
    }

    // --- ESCRITURA Y EDICIÓN ---
    if !editor.readonly {
        // PEGAR (Paste)
        if ctrl && keys.just_pressed(KeyCode::KeyV) {
            if let Some(pasted_text) = clipboard.get_text() {
                if let Some((start, end)) = editor.selection.take() {
                    let min = start.min(end);
                    editor.text.remove(min..start.max(end));
                    cursor = min;
                }
                editor.text.insert(cursor, &pasted_text);
                cursor += pasted_text.chars().count();
                changed_text = true;
            }
        }

        // TAB (Indentación de 4 espacios)
        if keys.just_pressed(KeyCode::Tab) {
            if let Some((start, end)) = editor.selection.take() {
                let min = start.min(end);
                editor.text.remove(min..start.max(end));
                cursor = min;
            }
            editor.text.insert(cursor, "    ");
            cursor += 4;
            changed_text = true;
        }

        // ESCRITURA ESTÁNDAR
        for event in events.read() {
            if event.state == ButtonState::Pressed {
                if let Key::Character(c) = &event.logical_key {
                    if ctrl { continue; }
                    let char_str = c.as_str();
                    if !char_str.is_empty() && char_str != " " {
                        if let Some((start, end)) = editor.selection.take() {
                            let min = start.min(end);
                            editor.text.remove(min..start.max(end));
                            cursor = min;
                        }
                        editor.text.insert(cursor, char_str);
                        cursor += char_str.chars().count();
                        changed_text = true;
                    }
                }
            }
        }

        // ENTER (Con Auto-indentación)
        if keys.just_pressed(KeyCode::Enter) {
            if let Some((start, end)) = editor.selection.take() {
                let min = start.min(end);
                editor.text.remove(min..start.max(end));
                cursor = min;
            }
            
            // Lógica de Auto-indentación
            let current_line_idx = editor.text.char_to_line(cursor);
            let line_slice = editor.text.line(current_line_idx);
            let mut indent_spaces = String::new();
            
            for ch in line_slice.chars() {
                if ch == ' ' || ch == '\t' { indent_spaces.push(ch); } 
                else { break; }
            }

            editor.text.insert_char(cursor, '\n');
            cursor += 1;
            
            if !indent_spaces.is_empty() {
                editor.text.insert(cursor, &indent_spaces);
                cursor += indent_spaces.chars().count();
            }
            changed_text = true;
        }

        // BORRADO (Backspace y Delete)
        if keys.just_pressed(KeyCode::Backspace) {
            if let Some((start, end)) = editor.selection.take() {
                let min = start.min(end);
                editor.text.remove(min..start.max(end));
                cursor = min;
                changed_text = true;
            } else if cursor > 0 {
                editor.text.remove((cursor - 1)..cursor);
                cursor -= 1;
                changed_text = true;
            }
        } else if keys.just_pressed(KeyCode::Delete) {
            if let Some((start, end)) = editor.selection.take() {
                let min = start.min(end);
                editor.text.remove(min..start.max(end));
                cursor = min;
                changed_text = true;
            } else if cursor < editor.text.len_chars() {
                editor.text.remove(cursor..(cursor + 1));
                changed_text = true;
            }
        } else if keys.just_pressed(KeyCode::Space) {
            if let Some((start, end)) = editor.selection.take() {
                let min = start.min(end);
                editor.text.remove(min..start.max(end));
                cursor = min;
            }
            editor.text.insert_char(cursor, ' ');
            cursor += 1;
            changed_text = true;
        }
    }

    // --- ACTUALIZAR ESTADO ---
    if changed_text || changed_cursor {
        if changed_text {
            if !is_undo_redo {
                // Comparamos el Rope entero es O(1) si no cambiaron
                if editor.text != old_state.text { 
                    editor.undo_stack.push(old_state);
                    if editor.undo_stack.len() > 50 { editor.undo_stack.remove(0); }
                    editor.redo_stack.clear();
                }
            }
        }
        editor.cursor_index = cursor;
        editor.show_cursor = true;
        editor.cursor_timer.reset();
    }
}

use bevy::prelude::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::util::LinesWithEndings;

pub fn update_code_editor_visuals(
    mut commands: Commands,
    // Solo ejecutamos esto si el editor realmente cambió (eficiencia pura)
    editor_query: Query<(Entity, &RuiCodeEditor, &Children), Changed<RuiCodeEditor>>,
    mut text_query: Query<(Entity, &mut Text, &TextFont), With<RuiCodeEditorText>>,
    mut gutter_query: Query<&mut Text, (With<RuiCodeEditorGutter>, Without<RuiCodeEditorText>)>,
    children_query: Query<&Children>,
    syntax_assets: Res<RuiSyntaxAssets>,
) {
    for (editor_entity, editor, children) in &editor_query {
        
        // 1. Encontrar los nodos de Gutter y Texto entre los hijos
        let mut text_entity_opt = None;
        let mut gutter_text_opt = None;

        // Buscamos de forma recursiva en los hijos (simplificado)
        for child in children.iter() {
            if let Ok(grand_children) = children_query.get(child) {
                for grand_child in grand_children.iter() {
                    if text_query.contains(grand_child) {
                        text_entity_opt = Some(grand_child);
                    }
                    if gutter_query.contains(grand_child) {
                        gutter_text_opt = Some(grand_child);
                    }
                }
            }
        }

        // 2. Actualizar los números de línea (Gutter)
        if let Some(gutter_entity) = gutter_text_opt {
            if let Ok(mut gutter_text) = gutter_query.get_mut(gutter_entity) {
                let total_lines = editor.text.len_lines();
                let mut line_numbers = String::with_capacity(total_lines * 3);
                for i in 1..=total_lines {
                    line_numbers.push_str(&format!("{}\n", i));
                }
                **gutter_text = line_numbers; // En Bevy 0.15, mut Text se desreferencia a String
            }
        }

        // 3. Procesar el texto con Syntect y generar TextSpans
        if let Some(text_entity) = text_entity_opt {
            if let Ok((entity, mut root_text, font)) = text_query.get_mut(text_entity) {
                // Limpiar los hijos anteriores (spans viejos)
                commands.entity(entity).despawn();
                
                // Bevy 0.15: El texto raíz queda vacío, todo el texto estará en los spans hijos
                **root_text = String::new(); 

                // Configurar Syntect para el lenguaje especificado
                let syntax = syntax_assets.syntax_set.find_syntax_by_extension(&editor.language)
                    .unwrap_or_else(|| syntax_assets.syntax_set.find_syntax_plain_text());
                
                let theme = &syntax_assets.theme_set.themes[&syntax_assets.default_theme];
                let mut highlighter = HighlightLines::new(syntax, theme);

                // Convertir el Rope a String temporalmente para iterar las líneas
                // (Para archivos MUY grandes en el futuro, esto se optimiza procesando solo las líneas visibles)
                let full_text = editor.text.to_string();

                commands.entity(entity).with_children(|parent| {
                    for line in LinesWithEndings::from(&full_text) {
                        // Syntect nos devuelve una lista de (Estilo, Palabra)
                        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &syntax_assets.syntax_set).unwrap();

                        for (style, text_segment) in ranges {
                            // Convertir el color de Syntect (RGBA) al Color de Bevy
                            let color = Color::srgba_u8(style.foreground.r, style.foreground.g, style.foreground.b, style.foreground.a);
                            
                            // Spawnear el segmento de texto colorido
                            parent.spawn((
                                TextSpan::new(text_segment),
                                TextFont {
                                    font: font.font.clone(),
                                    font_size: font.font_size,
                                    ..default()
                                },
                                TextColor(color),
                            ));
                        }
                    }
                });
            }
        }
    }
}







