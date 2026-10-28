//! Interactive splitter/resizer bar for adjusting adjacent flex panes and collapsing/expanding panels.

use bevy::{ecs::relationship::Relationship, prelude::*};
use crate::widgets::RuiButtonStateColors;

/// Resize axis direction for a [`RuiResizer`].
#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
pub enum RuiResizerDir {
    /// Dragged left/right (rendered as a vertical bar between horizontal panes).
    Horizontal,
    /// Dragged up/down (rendered as a horizontal bar between vertical panes).
    Vertical,
}

/// Component attached to a splitter bar between two sibling flex panes.
#[derive(Component)]
pub struct RuiResizer {
    pub dir: RuiResizerDir,
    pub dragging: bool,
    pub min_size: f32,
    pub is_collapsed_prev: bool,
    pub is_collapsed_next: bool,
    pub last_cursor_pos: Option<Vec2>,
}

/// Component attached to the collapse/expand toggle button inside a [`RuiResizer`].
#[derive(Component)]
pub struct RuiResizerCollapseBtn {
    pub resizer_entity: Entity,
    pub is_prev: bool,
    pub text_entity: Entity,
}

/// Spawns a draggable splitter bar between two sibling flex panes with minimum pane size `min_size`.
///
/// Uses a `PointerDrag` observer to resize adjacent panes proportionally and includes a quick-collapse button.
///
/// Also available via [`crate::widgets::RuiBuilderExt::resizer`].
pub fn spawn_resizer<'a>(
    parent: &'a mut ChildSpawnerCommands,
    dir: RuiResizerDir,
    min_size: f32,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        width: if dir == RuiResizerDir::Horizontal { Val::Px(6.0) } else { Val::Percent(100.0) },
        height: if dir == RuiResizerDir::Vertical { Val::Px(6.0) } else { Val::Percent(100.0) },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: if dir == RuiResizerDir::Horizontal { FlexDirection::Column } else { FlexDirection::Row },
        ..default()
    };
    modifier(&mut s);

    let colors = RuiButtonStateColors {
        normal: Color::BLACK,
        hovered: Color::srgb(0.4, 0.6, 1.0), 
        pressed: Color::srgb(0.2, 0.4, 0.8),
    };

    let mut cmds = parent.spawn((
        s,
        bevy::ui_widgets::Button,
        BackgroundColor(Color::BLACK),
        colors.clone(),
        RuiResizer { dir, dragging: false, min_size, is_collapsed_prev: false, is_collapsed_next: false, last_cursor_pos: None },
        crate::focus::Focusable,
        Pickable::default(),
    ));

    let resizer_id = cmds.id();

        cmds.with_children(|p| {
        let btn_s = Node {
            display: Display::Flex, width: Val::Px(14.0), height: Val::Px(14.0),
            justify_content: JustifyContent::Center, align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(1.0)), border: UiRect::all(Val::Px(1.0)), ..default()
        };

        // Botón flecha Prev
        let next_text = if dir == RuiResizerDir::Horizontal { ">" } else { "v" };        
        let mut next_text_id = Entity::PLACEHOLDER;
        let mut next_btn = p.spawn((
            btn_s, bevy::ui_widgets::Button, BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), colors.clone(), Pickable::default(),
        ));
        next_btn.with_children(|b| {
            next_text_id = b.spawn((Text::new(next_text), TextFont { font_size: bevy::prelude::FontSize::Px(14.0), ..default() }, TextColor(Color::WHITE), Pickable::IGNORE)).id();
        });
        next_btn.insert(RuiResizerCollapseBtn { resizer_entity: resizer_id, is_prev: false, text_entity: next_text_id });
    });
    // OBSERVADOR REACTIVO: Sincroniza dinámicamente el flex_grow de los paneles adyacentes usando el delta liso del PointerDrag de Bevy 0.20
    cmds.observe(
        move |on_drag: On<PointerDrag>,
              mut q_resizer: Query<(Entity, &mut RuiResizer, &ChildOf)>,
              q_children: Query<&Children>,
              mut q_node: Query<(&mut Node, &ComputedNode)>| 
        {
            let Ok((entity, resizer, parent)) = q_resizer.get_mut(on_drag.event_target()) else { return; };
            if resizer.is_collapsed_prev || resizer.is_collapsed_next { return; }

            let Ok(children) = q_children.get(parent.get()) else { return; };
            let Some(pos) = children.iter().position(|e| e == entity) else { return; };
            if pos == 0 || pos + 1 >= children.len() { return; }

            let (prev_entity, next_entity) = (children[pos - 1], children[pos + 1]);

            if let Ok([(mut prev_node, prev_comp), (mut next_node, next_comp)]) = q_node.get_many_mut([prev_entity, next_entity]) {
                let min_size = resizer.min_size;
                
                // Fijación atómica inicial de flex containers a píxeles estables
                if prev_node.flex_basis != Val::Percent(0.0) {
                    prev_node.flex_basis = Val::Percent(0.0);
                    next_node.flex_basis = Val::Percent(0.0);
                    if resizer.dir == RuiResizerDir::Vertical {
                        prev_node.flex_grow = prev_comp.size().y; next_node.flex_grow = next_comp.size().y;
                        prev_node.height = Val::Auto; next_node.height = Val::Auto;
                    } else {
                        prev_node.flex_grow = prev_comp.size().x; next_node.flex_grow = next_comp.size().x;
                        prev_node.width = Val::Auto; next_node.width = Val::Auto;
                    }
                }

                // Desplazamiento elástico proporcional usando deltas instantáneos del ratón
                if resizer.dir == RuiResizerDir::Vertical {
                    let total_h = prev_node.flex_grow + next_node.flex_grow;
                    if total_h > 0.0 {
                        let mut new_prev_h = (prev_node.flex_grow + on_drag.delta.y).max(min_size);
                        let mut new_next_h = (next_node.flex_grow - on_drag.delta.y).max(min_size);
                        if new_prev_h == min_size { new_next_h = total_h - min_size; }
                        if new_next_h == min_size { new_prev_h = total_h - min_size; }
                        prev_node.flex_grow = new_prev_h; next_node.flex_grow = new_next_h;
                    }
                } else {
                    let total_w = prev_node.flex_grow + next_node.flex_grow;
                    if total_w > 0.0 {
                        let mut new_prev_w = (prev_node.flex_grow + on_drag.delta.x).max(min_size);
                        let mut new_next_w = (next_node.flex_grow - on_drag.delta.x).max(min_size);
                        if new_prev_w == min_size { new_next_w = total_w - min_size; }
                        if new_next_w == min_size { new_prev_w = total_w - min_size; }
                        prev_node.flex_grow = new_prev_w; next_node.flex_grow = new_next_w;
                    }
                }
            }
        }
    );

    cmds
}
/// System that updates the window cursor icon (`ColResize` / `RowResize`) when hovering over a [`RuiResizer`].
pub fn handle_resizer_drag(
    mut q_window: Query<(Entity, &Window, Option<&mut bevy::window::CursorIcon>), With<bevy::window::PrimaryWindow>>,
    mut commands: Commands,
    q_resizer: Query<&RuiResizer>,
    q_hovered: Query<&bevy::picking::hover::Hovered>,
) {
    let mut desired_cursor = None;

    // Evaluamos los estados basándonos en el componente Hovered unificado oficial de Bevy 0.20
    for (resizer, hovered) in q_resizer.iter().zip(q_hovered.iter()) {
        if hovered.0 {
            desired_cursor = Some(if resizer.dir == RuiResizerDir::Vertical {
                bevy::window::SystemCursorIcon::RowResize
            } else {
                bevy::window::SystemCursorIcon::ColResize
            });
        }
    }

    if let Ok((window_entity, _window, mut opt_cursor)) = q_window.single_mut() {
        if let Some(cursor) = desired_cursor {
            let new_icon = bevy::window::CursorIcon::System(cursor);
            if let Some(ref mut cursor_icon) = opt_cursor {
                if **cursor_icon != new_icon { **cursor_icon = new_icon; }
            } else {
                commands.entity(window_entity).insert(new_icon);
            }
        } else {
            let default_icon = bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::Default);
            if let Some(ref mut cursor_icon) = opt_cursor {
                let is_col = matches!(**cursor_icon, bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::ColResize));
                let is_row = matches!(**cursor_icon, bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::RowResize));
                if is_col || is_row { **cursor_icon = default_icon; }
            }
        }
    }
}
/// Función pura aislada para resolver el carácter de la flecha sin saturar el buffer
fn get_arrow_char(is_prev: bool, is_collapsed: bool, is_hor: bool) -> String {
    if is_prev {
        if is_collapsed { if is_hor { ">" } else { "v" } } else { if is_hor { "<" } else { "^" } }
    } else {
        if is_collapsed { if is_hor { "<" } else { "^" } } else { if is_hor { ">" } else { "v" } }
    }
    .to_string()
}

/// System that collapses or restores adjacent flex panes when a [`RuiResizerCollapseBtn`] is clicked.
pub fn handle_resizer_collapse_clicks(
    q_clicks: Query<&RuiResizerCollapseBtn, With<bevy::ui::Pressed>>,
    mut q_resizer: Query<(&mut RuiResizer, &ChildOf)>,
    q_children: Query<&Children>,
    mut q_node: Query<(&mut Node, &ComputedNode)>,
    mut q_text: Query<&mut Text>,
    q_all_btns: Query<(Entity, &RuiResizerCollapseBtn)>,
) {
    for btn in &q_clicks {
        let Ok((mut resizer, parent)) = q_resizer.get_mut(btn.resizer_entity) else { continue; };
        if (btn.is_prev && resizer.is_collapsed_next) || (!btn.is_prev && resizer.is_collapsed_prev) { continue; }

        let Ok(children) = q_children.get(parent.get()) else { continue; };
        let Some(pos) = children.iter().position(|e| e == btn.resizer_entity) else { continue; };
        if pos == 0 || pos + 1 >= children.len() { continue; }

        let (prev_e, next_e) = (children[pos - 1], children[pos + 1]);

        if let Ok((mut prev_n, prev_c)) = q_node.get_mut(prev_e) {
            if prev_n.flex_basis != Val::Percent(0.0) {
                prev_n.flex_basis = Val::Percent(0.0);
                prev_n.flex_grow = if resizer.dir == RuiResizerDir::Vertical { prev_c.size().y } else { prev_c.size().x };
                if resizer.dir == RuiResizerDir::Vertical { prev_n.height = Val::Auto; } else { prev_n.width = Val::Auto; }
            }
        }
        if let Ok((mut next_n, next_c)) = q_node.get_mut(next_e) {
            if next_n.flex_basis != Val::Percent(0.0) {
                next_n.flex_basis = Val::Percent(0.0);
                next_n.flex_grow = if resizer.dir == RuiResizerDir::Vertical { next_c.size().y } else { next_c.size().x };
                if resizer.dir == RuiResizerDir::Vertical { next_n.height = Val::Auto; } else { next_n.width = Val::Auto; }
            }
        }

        if btn.is_prev {
            resizer.is_collapsed_prev = !resizer.is_collapsed_prev;
            if let Ok((mut n, _)) = q_node.get_mut(prev_e) { n.display = if resizer.is_collapsed_prev { Display::None } else { Display::Flex }; }
        } else {
            resizer.is_collapsed_next = !resizer.is_collapsed_next;
            if let Ok((mut n, _)) = q_node.get_mut(next_e) { n.display = if resizer.is_collapsed_next { Display::None } else { Display::Flex }; }
        }

        if let Ok(mut text) = q_text.get_mut(btn.text_entity) {
            let collapsed = if btn.is_prev { resizer.is_collapsed_prev } else { resizer.is_collapsed_next };
            text.0 = get_arrow_char(btn.is_prev, collapsed, resizer.dir == RuiResizerDir::Horizontal);
        }

        for (sibling_entity, sibling_btn) in &q_all_btns {
            if sibling_btn.resizer_entity == btn.resizer_entity {
                if let Ok((mut sibling_node, _)) = q_node.get_mut(sibling_entity) {
                    sibling_node.display = if (sibling_btn.is_prev && resizer.is_collapsed_next) || (!sibling_btn.is_prev && resizer.is_collapsed_prev) { Display::None } else { Display::Flex };
                }
            }
        }
    }
}
