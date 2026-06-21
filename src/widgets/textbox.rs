use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::ui::RelativeCursorPosition;
use bevy::text::TextLayoutInfo;


use bevy::input_focus::{InputFocus, FocusCause};
use crate::widgets::{ RuiClipboard};
use crate::widgets::scrollview::{RuiVerticalScrollbar, RuiHorizontalScrollbar};

#[derive(Clone, Default)]
pub struct TextBoxState {
    pub text: String,
    pub cursor_index: usize,
    pub selection: Option<(usize, usize)>,
}

#[derive(Reflect, Clone, PartialEq, Default)]
pub enum RuiTextBoxMode {
    #[default]
    SingleLine,
    MultiLine,
}

#[derive(Component)]
pub struct RuiSelectionHighlight;

#[derive(Component)]
pub struct RuiTextBoxText;

#[derive(Component)]
pub struct RuiCursor;

#[derive(Component)]
pub struct RuiTextBox {
    pub text: String,
    pub placeholder: String,
    pub cursor_timer: Timer,
    pub show_cursor: bool,
    pub cursor_index: usize,
    pub mode: RuiTextBoxMode,
    pub scroll_offset: Vec2,
    pub max_characters: Option<usize>,
    pub readonly: bool,
    pub selection: Option<(usize, usize)>,
    pub selection_anchor: Option<usize>,
    pub word_wrap: bool,
    pub cursor_moved: bool,
    pub dragging_v_scroll: bool,
    pub dragging_h_scroll: bool,
    pub previous_cursor_index: usize,
    pub previous_text_len: usize,
    pub previous_focused: bool,
    pub undo_stack: Vec<TextBoxState>,
    pub redo_stack: Vec<TextBoxState>,
}

impl Default for RuiTextBox {
    fn default() -> Self {
        Self {
            text: String::new(),
            placeholder: String::new(),
            cursor_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            show_cursor: false,
            cursor_index: 0,
            mode: RuiTextBoxMode::SingleLine,
            scroll_offset: Vec2::ZERO,
            max_characters: None,
            readonly: false,
            selection: None,
            selection_anchor: None,
            word_wrap: false,
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

pub fn spawn_textbox<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    placeholder: &str,
    modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor),
) -> EntityCommands<'a> {
        let mut s = Node {
            display: Display::Flex,
            min_height: Val::Px(36.0),
            min_width: Val::Px(100.0),
            border: UiRect::all(Val::Px(2.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            overflow: Overflow::clip(),
            ..default()
        };
        let mut font = TextFont::default();
        let mut color = TextColor(Color::WHITE);
        modifier(&mut s, &mut font, &mut color);

        let mut cmds = parent_cmd.spawn((
            s,
            Button,
            crate::focus::Focusable,
            bevy::ui::FocusPolicy::Block,
            Pickable::default(),
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::default() },
            crate::theme::RuiThemeElement::TextboxBg,
            BorderColor::all(Color::srgb(0.051, 0.051, 0.051)),
            RelativeCursorPosition::default(),
            RuiTextBox { placeholder: placeholder.to_string(), ..default() }
        ));

        cmds.with_children(|parent| {
            parent.spawn((
                Text::new(placeholder),
                font,
                color,
                RuiTextBoxText,
                crate::theme::RuiThemeElement::Text,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    ..default()
                },
                Pickable::IGNORE,
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(1.0),
                    height: Val::Percent(50.0),
                    left: Val::Px(14.0),
                    ..default()
                },
                ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::WHITE) },
                Visibility::Hidden,
                RuiCursor,
                Pickable::IGNORE,
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    width: Val::Px(0.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgba(0.2, 0.4, 0.8, 0.5)) },
                Visibility::Hidden,
                RuiSelectionHighlight,
                Pickable::IGNORE,
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    width: Val::Px(6.0),
                    height: Val::Percent(0.0),
                    ..default()
                },
                ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)) },
                Visibility::Hidden,
                RuiVerticalScrollbar,
                Pickable::IGNORE,
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                    width: Val::Percent(0.0),
                    height: Val::Px(6.0),
                    ..default()
                },
                ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)) },
                Visibility::Hidden,
                RuiHorizontalScrollbar,
                Pickable::IGNORE,
            ));
        });

        cmds
    }

pub fn spawn_multiline_textbox<'a>(
    parent: &'a mut ChildSpawnerCommands,
    placeholder: &str,
    modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor),
) -> EntityCommands<'a> {
    let mut cmds = spawn_textbox(parent, placeholder, modifier);
        cmds.insert(RuiTextBox {
            placeholder: placeholder.to_string(),
            mode: RuiTextBoxMode::MultiLine,
            ..default()
        });
        cmds
}

pub fn handle_textbox_input(
    mut events: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    mut query: Query<&mut RuiTextBox>,
    mut clipboard: NonSendMut<RuiClipboard>,
) {
    let Some(focused_entity) = focus.get() else { return };
    let Ok(mut textbox) = query.get_mut(focused_entity) else { return };

    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight]);
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let mut changed_text = false;
    let mut changed_cursor = false;
    
    let mut current_chars: Vec<char> = textbox.text.chars().collect();
    let mut cursor = textbox.cursor_index.clamp(0, current_chars.len());
    let old_cursor = cursor;
    
    let old_state = TextBoxState {
        text: textbox.text.clone(),
        cursor_index: textbox.cursor_index,
        selection: textbox.selection,
    };
    let mut is_undo_redo = false;

    if ctrl {
        if keys.just_pressed(KeyCode::KeyZ) && !textbox.readonly {
            if let Some(state) = textbox.undo_stack.pop() {
                textbox.redo_stack.push(old_state.clone());
                current_chars = state.text.chars().collect();
                cursor = state.cursor_index;
                textbox.selection = state.selection;
                textbox.selection_anchor = None;
                changed_text = true;
                is_undo_redo = true;
            }
        } else if keys.just_pressed(KeyCode::KeyY) && !textbox.readonly {
            if let Some(state) = textbox.redo_stack.pop() {
                textbox.undo_stack.push(old_state.clone());
                current_chars = state.text.chars().collect();
                cursor = state.cursor_index;
                textbox.selection = state.selection;
                textbox.selection_anchor = None;
                changed_text = true;
                is_undo_redo = true;
            }
        }

        if keys.just_pressed(KeyCode::KeyA) {
            let len = current_chars.len();
            textbox.selection_anchor = Some(0);
            textbox.selection = Some((0, len));
            cursor = len;
            changed_cursor = true;
        }
        if keys.just_pressed(KeyCode::KeyC) || (!textbox.readonly && keys.just_pressed(KeyCode::KeyX)) {
            if textbox.selection.is_some() || keys.just_pressed(KeyCode::KeyC) {
                let (start, end) = textbox.selection.unwrap_or((0, current_chars.len()));
                let min = start.min(end);
                let max = start.max(end);
                let copied_text: String = current_chars[min..max].iter().collect();
                clipboard.set_text(copied_text);
                
                if keys.just_pressed(KeyCode::KeyX) && !textbox.readonly {
                    current_chars.drain(min..max);
                    cursor = min;
                    textbox.selection = None;
                    textbox.selection_anchor = None;
                    changed_text = true;
                }
            }
        }
    }

    let mut nav_pressed = false;
    if keys.just_pressed(KeyCode::ArrowLeft) {
        nav_pressed = true;
        if !shift && textbox.selection.is_some() {
            let (start, end) = textbox.selection.unwrap();
            cursor = start.min(end);
        } else if cursor > 0 { cursor -= 1; }
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        nav_pressed = true;
        if !shift && textbox.selection.is_some() {
            let (start, end) = textbox.selection.unwrap();
            cursor = start.max(end);
        } else if cursor < current_chars.len() { cursor += 1; }
    } else if keys.just_pressed(KeyCode::ArrowUp) && textbox.mode == RuiTextBoxMode::MultiLine {
        nav_pressed = true;
        let text_up_to_cursor: String = current_chars[..cursor].iter().collect();
        let lines: Vec<&str> = text_up_to_cursor.split('\n').collect();
        if lines.len() > 1 {
            let current_line_chars = lines.last().unwrap().chars().count();
            let prev_line_chars = lines[lines.len() - 2].chars().count();
            cursor -= current_line_chars + 1 + (prev_line_chars - current_line_chars.min(prev_line_chars));
        }
    } else if keys.just_pressed(KeyCode::ArrowDown) && textbox.mode == RuiTextBoxMode::MultiLine {
        nav_pressed = true;
        let text_up_to_cursor: String = current_chars[..cursor].iter().collect();
        let current_col = text_up_to_cursor.split('\n').last().unwrap().chars().count();
        let text_after_cursor: String = current_chars[cursor..].iter().collect();
        let lines_after: Vec<&str> = text_after_cursor.split('\n').collect();
        if lines_after.len() > 1 {
            cursor += lines_after[0].chars().count() + 1 + current_col.min(lines_after[1].chars().count());
        }
    } else if keys.just_pressed(KeyCode::Home) { nav_pressed = true; cursor = 0; }
    else if keys.just_pressed(KeyCode::End) { nav_pressed = true; cursor = current_chars.len(); }

    if nav_pressed {
        changed_cursor = true;
        if shift {
            if textbox.selection.is_none() || textbox.selection_anchor.is_none() { textbox.selection_anchor = Some(old_cursor); }
            let anchor = textbox.selection_anchor.unwrap();
            textbox.selection = if anchor != cursor { Some((anchor, cursor)) } else { None };
        } else {
            textbox.selection = None;
            textbox.selection_anchor = None;
        }
    }

    if !textbox.readonly {
        if ctrl && keys.just_pressed(KeyCode::KeyV) {
            if let Some(pasted_text) = clipboard.get_text() {
                if let Some((start, end)) = textbox.selection.take() {
                    let min = start.min(end);
                    let max = start.max(end);
                    current_chars.drain(min..max);
                    cursor = min;
                }
                for ch in pasted_text.chars() {
                    if textbox.max_characters.map_or(true, |limit| current_chars.len() < limit) {
                        current_chars.insert(cursor, ch);
                        cursor += 1;
                        changed_text = true;
                    }
                }
            }
        }

        for event in events.read() {
            if event.state == ButtonState::Pressed {
                if let Key::Character(c) = &event.logical_key {
                    if ctrl { continue; }
                    let char_str = c.as_str();
                    if !char_str.is_empty() && char_str != " " {
                        if let Some((start, end)) = textbox.selection.take() {
                            let min = start.min(end);
                            current_chars.drain(min..start.max(end));
                            cursor = min;
                        }
                        for ch in char_str.chars() {
                            if textbox.max_characters.map_or(true, |limit| current_chars.len() < limit) {
                                current_chars.insert(cursor, ch);
                                cursor += 1;
                                changed_text = true;
                            }
                        }
                    }
                }
            }
        }

        if keys.just_pressed(KeyCode::Enter) && textbox.mode == RuiTextBoxMode::MultiLine {
             if let Some((start, end)) = textbox.selection.take() {
                let min = start.min(end);
                current_chars.drain(min..start.max(end));
                cursor = min;
            }
            if textbox.max_characters.map_or(true, |limit| current_chars.len() < limit) {
                current_chars.insert(cursor, '\n');
                cursor += 1;
                changed_text = true;
            }
        }

        if keys.just_pressed(KeyCode::Backspace) {
            if let Some((start, end)) = textbox.selection.take() {
                let min = start.min(end);
                current_chars.drain(min..start.max(end));
                cursor = min;
                changed_text = true;
            } else if cursor > 0 {
                current_chars.remove(cursor - 1);
                cursor -= 1;
                changed_text = true;
            }
        } else if keys.just_pressed(KeyCode::Delete) {
            if let Some((start, end)) = textbox.selection.take() {
                let min = start.min(end);
                current_chars.drain(min..start.max(end));
                cursor = min;
                changed_text = true;
            } else if cursor < current_chars.len() {
                current_chars.remove(cursor);
                changed_text = true;
            }
        } else if keys.just_pressed(KeyCode::Space) {
            if let Some((start, end)) = textbox.selection.take() {
                let min = start.min(end);
                current_chars.drain(min..start.max(end));
                cursor = min;
            }
            if textbox.max_characters.map_or(true, |limit| current_chars.len() < limit) {
                current_chars.insert(cursor, ' ');
                cursor += 1;
                changed_text = true;
            }
        }
    }

    if changed_text || changed_cursor {
        if changed_text {
            if !is_undo_redo {
                let new_text: String = current_chars.clone().into_iter().collect();
                if new_text != old_state.text {
                    textbox.undo_stack.push(old_state);
                    if textbox.undo_stack.len() > 50 { textbox.undo_stack.remove(0); }
                    textbox.redo_stack.clear();
                }
            }
            textbox.text = current_chars.into_iter().collect();
        }
        textbox.cursor_index = cursor;
        textbox.show_cursor = true;
        textbox.cursor_timer.reset();
    }
}

pub fn handle_textbox_scroll(
    mut mouse_wheel_events: MessageReader<bevy::input::mouse::MouseWheel>,
    mut textbox_query: Query<(&RelativeCursorPosition, &mut RuiTextBox)>,
) {
    for event in mouse_wheel_events.read() {
        for (rel_pos, mut textbox) in &mut textbox_query {
            if rel_pos.cursor_over() {
                use bevy::input::mouse::MouseScrollUnit;
                let amount = match event.unit { MouseScrollUnit::Line => 30.0, MouseScrollUnit::Pixel => 1.0 };
                if textbox.mode == RuiTextBoxMode::MultiLine {
                    textbox.scroll_offset.y -= event.y * amount;
                    textbox.scroll_offset.x -= event.x * amount;
                } else {
                    textbox.scroll_offset.x -= (event.y + event.x) * amount;
                }
                textbox.cursor_moved = false;
            }
        }
    }
}

pub fn update_textbox_visuals(
    time: Res<Time>,
    focus: Res<InputFocus>,
    mut textbox_query: Query<(Entity, &RelativeCursorPosition, &mut RuiTextBox, &mut BorderColor, &ComputedNode, &Children)>,
    mut text_query: Query<(&mut Text, Option<&TextLayoutInfo>, &mut Node, Option<&mut bevy::text::TextLayout>, Option<&TextFont>), (With<RuiTextBoxText>, Without<RuiCursor>, Without<RuiSelectionHighlight>, Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>)>,
    mut cursor_query: Query<(&mut Node, &mut Visibility), (With<RuiCursor>, Without<RuiTextBoxText>, Without<RuiSelectionHighlight>, Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>)>,
    mut selection_query: Query<(&mut Node, &mut Visibility), (With<RuiSelectionHighlight>, Without<RuiTextBoxText>, Without<RuiCursor>, Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>)>,
    mut v_scroll_query: Query<(&mut Node, &mut Visibility), (With<RuiVerticalScrollbar>, Without<RuiTextBoxText>, Without<RuiCursor>, Without<RuiSelectionHighlight>, Without<RuiHorizontalScrollbar>)>,
    mut h_scroll_query: Query<(&mut Node, &mut Visibility), (With<RuiHorizontalScrollbar>, Without<RuiTextBoxText>, Without<RuiCursor>, Without<RuiSelectionHighlight>, Without<RuiVerticalScrollbar>)>,
) {
    for (entity, rel_pos, mut textbox, mut border, computed, children) in &mut textbox_query {
        let is_focused = focus.get() == Some(entity);
        let show_scrollbars = rel_pos.cursor_over() || textbox.dragging_v_scroll || textbox.dragging_h_scroll;

        if is_focused {
            *border = BorderColor::all(Color::srgb(0.4, 0.6, 1.0));
            textbox.cursor_timer.tick(time.delta());
            if textbox.cursor_timer.just_finished() { textbox.show_cursor = !textbox.show_cursor; }
        } else {
            *border = BorderColor::all(Color::srgb(0.3, 0.3, 0.3));
            textbox.show_cursor = false;
        }

        if textbox.cursor_index != textbox.previous_cursor_index || textbox.text.len() != textbox.previous_text_len || is_focused != textbox.previous_focused {
            textbox.cursor_moved = true;
            textbox.previous_cursor_index = textbox.cursor_index;
            textbox.previous_text_len = textbox.text.len();
            textbox.previous_focused = is_focused;
        }

        let parent_size = computed.size();
        let visible_w = (parent_size.x - 24.0).max(10.0);
        let visible_h = (parent_size.y - 24.0).max(10.0);
        let display_text = if textbox.text.is_empty() && !is_focused { textbox.placeholder.clone() } else { textbox.text.clone() };

        // Buscamos el hijo que contiene el texto usando el marcador RuiTextBoxText
        let text_child = children.iter().find(|&c| text_query.contains(c));

        if let Some((mut text, layout_opt, mut text_node, text_layout_opt, font_opt)) = text_child.and_then(|e| text_query.get_mut(e).ok()) {
                let font_size = font_opt.map_or(18.0, |f| match f.font_size { bevy::prelude::FontSize::Px(v) | bevy::prelude::FontSize::Vw(v) | bevy::prelude::FontSize::Vh(v) | bevy::prelude::FontSize::VMin(v) | bevy::prelude::FontSize::VMax(v) | bevy::prelude::FontSize::Rem(v) => v });
                if text.0 != display_text { text.0 = display_text; }
                
                // Si el texto visual está vacío (por ejemplo, al entrar en foco un textbox vacío),
                // forzamos el reset del scroll para evitar que el cursor o el placeholder desaparezcan.
                if text.0.is_empty() { textbox.scroll_offset = Vec2::ZERO; }
                let (text_w, text_h) = layout_opt.map_or((0.0, 0.0), |l| (l.size.x, l.size.y));

                if textbox.mode == RuiTextBoxMode::SingleLine || !textbox.word_wrap {
                    text_node.width = Val::Auto;
                    if let Some(mut text_layout) = text_layout_opt { text_layout.linebreak = bevy::text::LineBreak::NoWrap; }
                } else {
                    text_node.width = Val::Px(visible_w);
                    if let Some(mut text_layout) = text_layout_opt { text_layout.linebreak = bevy::text::LineBreak::WordBoundary; }
                }

                let line_height = font_size * 1.2;
                let char_count = textbox.text.chars().count();
                let (cursor_x, cursor_y);

                if textbox.mode == RuiTextBoxMode::SingleLine {
                    let avg_char_w = if char_count > 0 && text_w > 0.0 { text_w / char_count as f32 } else { font_size * 0.5 };
                    cursor_x = avg_char_w * textbox.cursor_index as f32;
                    cursor_y = 0.0;
                    if textbox.cursor_moved {
                        if cursor_x - textbox.scroll_offset.x > visible_w { textbox.scroll_offset.x = cursor_x - visible_w; }
                        else if cursor_x < textbox.scroll_offset.x { textbox.scroll_offset.x = cursor_x; }
                    }
                } else {
                    let max_line_chars = textbox.text.split('\n').map(|l| l.chars().count()).max().unwrap_or(1).max(1);
                    let avg_char_w = (if text_w > 0.0 { text_w / max_line_chars as f32 } else { font_size * 0.5 }).max(font_size * 0.45);
                    let text_up_to = textbox.text.chars().take(textbox.cursor_index).collect::<String>();
                    let lines = text_up_to.split('\n').collect::<Vec<&str>>();
                    let line_idx = lines.len().saturating_sub(1);
                    let chars_in_line = lines.last().unwrap_or(&"").chars().count();

                    if !textbox.word_wrap {
                        cursor_x = chars_in_line as f32 * avg_char_w;
                        cursor_y = line_idx as f32 * line_height;
                    } else {
                        let chars_per_row = (visible_w / avg_char_w).floor().max(1.0) as usize;
                        let mut wrapped_lines = 0;
                        for i in 0..line_idx { wrapped_lines += lines[i].chars().count() / chars_per_row; }
                        cursor_x = (chars_in_line % chars_per_row) as f32 * avg_char_w;
                        cursor_y = (line_idx + wrapped_lines + (chars_in_line / chars_per_row)) as f32 * line_height;
                    }

                    if textbox.cursor_moved {
                        if cursor_y - textbox.scroll_offset.y > visible_h - line_height { textbox.scroll_offset.y = cursor_y - visible_h + line_height; }
                        else if cursor_y < textbox.scroll_offset.y { textbox.scroll_offset.y = cursor_y; }
                        if cursor_x - textbox.scroll_offset.x > visible_w { textbox.scroll_offset.x = cursor_x - visible_w; }
                        else if cursor_x < textbox.scroll_offset.x { textbox.scroll_offset.x = cursor_x; }
                    }
                    textbox.scroll_offset.y = textbox.scroll_offset.y.clamp(0.0, (text_h - visible_h).max(0.0));
                }
                textbox.scroll_offset.x = textbox.scroll_offset.x.clamp(0.0, (text_w - visible_w).max(0.0));
                text_node.left = Val::Px(12.0 - textbox.scroll_offset.x);
                if textbox.mode == RuiTextBoxMode::SingleLine {
                    text_node.top = Val::Px((parent_size.y - text_h) / 2.0);
                } else {
                    text_node.top = Val::Px(12.0 - textbox.scroll_offset.y);
                }

                for child in children.iter() {
                    if let Ok((mut c_node, mut c_vis)) = cursor_query.get_mut(child) {
                        if is_focused && textbox.show_cursor {
                            *c_vis = Visibility::Visible;
                            c_node.left = Val::Px(12.0 + cursor_x - textbox.scroll_offset.x);
                            if textbox.mode == RuiTextBoxMode::SingleLine {
                                c_node.top = Val::Px((parent_size.y - text_h.max(20.0)) / 2.0);
                            } else {
                                c_node.top = Val::Px(12.0 + cursor_y - textbox.scroll_offset.y);
                            }
                            c_node.height = Val::Px(if textbox.mode == RuiTextBoxMode::SingleLine { text_h.max(20.0) } else { line_height });
                        } else { *c_vis = Visibility::Hidden; }
                    }

                    if let Ok((mut s_node, mut s_vis)) = selection_query.get_mut(child) {
                        if is_focused && textbox.selection.is_some() {
                            *s_vis = Visibility::Visible;
                            let (start, end) = textbox.selection.unwrap();
                            let (min, max) = (start.min(end), start.max(end));
                            let avg_char_w = if text_w > 0.0 { text_w / char_count.max(1) as f32 } else { font_size * 0.5 };
                            s_node.left = Val::Px(12.0 + (if textbox.mode == RuiTextBoxMode::SingleLine { avg_char_w * min as f32 } else { 0.0 }) - textbox.scroll_offset.x);
                            s_node.width = Val::Px(if textbox.mode == RuiTextBoxMode::SingleLine { avg_char_w * (max - min) as f32 } else { text_w });
                            if textbox.mode == RuiTextBoxMode::SingleLine {
                                s_node.top = Val::Px((parent_size.y - text_h.max(line_height)) / 2.0);
                            } else {
                                s_node.top = Val::Px(12.0 - textbox.scroll_offset.y);
                            }
                            s_node.height = Val::Px(text_h.max(line_height));
                        } else { *s_vis = Visibility::Hidden; }
                    }

                    if let Ok((mut v_node, mut v_vis)) = v_scroll_query.get_mut(child) {
                        if textbox.mode == RuiTextBoxMode::MultiLine && text_h > visible_h && show_scrollbars {
                            *v_vis = Visibility::Visible;
                            let ratio = textbox.scroll_offset.y / (text_h - visible_h).max(1.0);
                            let h = (visible_h / text_h) * parent_size.y;
                            v_node.height = Val::Px(h.max(10.0));
                            v_node.top = Val::Px(2.0 + ratio * (parent_size.y - h - 4.0));
                        } else { *v_vis = Visibility::Hidden; }
                    }

                    if let Ok((mut h_node, mut h_vis)) = h_scroll_query.get_mut(child) {
                        if text_w > visible_w && show_scrollbars {
                            *h_vis = Visibility::Visible;
                            let ratio = textbox.scroll_offset.x / (text_w - visible_w).max(1.0);
                            let w = (visible_w / text_w) * parent_size.x;
                            h_node.width = Val::Px(w.max(10.0));
                            h_node.left = Val::Px(2.0 + ratio * (parent_size.x - w - 4.0));
                        } else { *h_vis = Visibility::Hidden; }
                    }
                }
            }
            textbox.cursor_moved = false;
        }
}

pub fn handle_textbox_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    focus: Res<InputFocus>,
    mut textbox_query: Query<(Entity, &RelativeCursorPosition, &Interaction, &ComputedNode, &Children, &mut RuiTextBox)>,
    text_query: Query<(&TextLayoutInfo, Option<&TextFont>), With<RuiTextBoxText>>,
) {
    let Some(focused_entity) = focus.get() else { return; };
    if let Ok((_, rel_pos, interaction, parent_comp, children, mut textbox)) = textbox_query.get_mut(focused_entity) {
        if !mouse.pressed(MouseButton::Left) { textbox.dragging_v_scroll = false; textbox.dragging_h_scroll = false; return; }
        if let Some(pos) = rel_pos.normalized {
            let size = parent_comp.size();
            let (lx, ly) = ((pos.x + 0.5) * size.x, (pos.y + 0.5) * size.y);
            if mouse.just_pressed(MouseButton::Left) && *interaction == Interaction::Pressed {
                if textbox.mode == RuiTextBoxMode::MultiLine && lx >= size.x - 14.0 { textbox.dragging_v_scroll = true; }
                else if ly >= size.y - 14.0 { textbox.dragging_h_scroll = true; }
            }
            if textbox.dragging_v_scroll || textbox.dragging_h_scroll {
                let text_layout = children.iter().find(|&c| text_query.contains(c))
                    .and_then(|e| text_query.get(e).ok());
                if let Some((layout, _)) = text_layout {
                        if textbox.dragging_v_scroll {
                            let v_h = (size.y - 24.0).max(10.0);
                            let h = (v_h / layout.size.y) * size.y;
                            textbox.scroll_offset.y = ((ly - 2.0 - h / 2.0) / (size.y - h - 4.0)).clamp(0.0, 1.0) * (layout.size.y - v_h).max(0.0);
                        } else {
                            let v_w = (size.x - 24.0).max(10.0);
                            let w = (v_w / layout.size.x) * size.x;
                            textbox.scroll_offset.x = ((lx - 2.0 - w / 2.0) / (size.x - w - 4.0)).clamp(0.0, 1.0) * (layout.size.x - v_w).max(0.0);
                        }
                        textbox.cursor_moved = false;
                }
                return;
            }
            let text_layout = children.iter().find(|&c| text_query.contains(c))
                .and_then(|e| text_query.get(e).ok());
            if let Some((layout, font_opt)) = text_layout {
                    let max_idx = textbox.text.chars().count();
                    let font_size = font_opt.map_or(18.0, |f| match f.font_size { bevy::prelude::FontSize::Px(v) | bevy::prelude::FontSize::Vw(v) | bevy::prelude::FontSize::Vh(v) | bevy::prelude::FontSize::VMin(v) | bevy::prelude::FontSize::VMax(v) | bevy::prelude::FontSize::Rem(v) => v });
                    let mut clicked_idx = 0;
                    if textbox.mode == RuiTextBoxMode::SingleLine {
                        let tx = lx - 14.0 + textbox.scroll_offset.x;
                        clicked_idx = (tx / (if max_idx > 0 && layout.size.x > 0.0 { layout.size.x / max_idx as f32 } else { 10.0 })).round() as usize;
                    } else {
                        let max_chars = textbox.text.split('\n').map(|l| l.chars().count()).max().unwrap_or(1).max(1);
                        let avg_w = (if layout.size.x > 0.0 { layout.size.x / max_chars as f32 } else { font_size * 0.5 }).max(font_size * 0.45);
                        let (tx, ty) = (lx - 14.0 + textbox.scroll_offset.x, ly - 12.0 + textbox.scroll_offset.y);
                        let line_h = font_size * 1.2;
                        let target_line = (ty / line_h).floor() as usize;
                        let mut abs_idx = 0; let mut cur_vis_line = 0; let mut found = false;
                        let chars_per_row = ((size.x - 24.0).max(10.0) / avg_w).floor().max(1.0) as usize;
                        for line in textbox.text.split('\n') {
                            let line_len = line.chars().count();
                            let vis_lines = if !textbox.word_wrap { 1 } else { (line_len / chars_per_row).max(1) };
                            if target_line < cur_vis_line + vis_lines {
                                clicked_idx = abs_idx + (if !textbox.word_wrap { (tx / avg_w).round() as usize } else { ((target_line - cur_vis_line) * chars_per_row) + (tx / avg_w).round() as usize }).min(line_len);
                                found = true; break;
                            }
                            abs_idx += line_len + 1; cur_vis_line += vis_lines;
                        }
                        if !found { clicked_idx = max_idx; }
                    }
                    clicked_idx = clicked_idx.clamp(0, max_idx);
                    if mouse.just_pressed(MouseButton::Left) && *interaction == Interaction::Pressed {
                        textbox.selection_anchor = Some(clicked_idx); textbox.cursor_index = clicked_idx; textbox.selection = None;
                    } else if let Some(anchor) = textbox.selection_anchor {
                        textbox.cursor_index = clicked_idx;
                        textbox.selection = if anchor != clicked_idx { Some((anchor, clicked_idx)) } else { None };
                    }
                    textbox.show_cursor = true; textbox.cursor_timer.reset();
                }
        }
    }
}