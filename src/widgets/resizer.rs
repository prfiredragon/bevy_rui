use bevy::{ecs::relationship::Relationship, prelude::*};
use crate::widgets::RuiButtonStateColors;

#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
pub enum RuiResizerDir {
    Horizontal, // Arrastramos Izquierda/Derecha (Es una barra vertical)
    Vertical,   // Arrastramos Arriba/Abajo (Es una barra horizontal)
}

#[derive(Component)]
pub struct RuiResizer {
    pub dir: RuiResizerDir,
    pub dragging: bool,
    pub min_size: f32,
    pub is_collapsed_prev: bool,
    pub is_collapsed_next: bool,
    pub last_cursor_pos: Option<Vec2>,
}

#[derive(Component)]
pub struct RuiResizerCollapseBtn {
    pub resizer_entity: Entity,
    pub is_prev: bool,
    pub text_entity: Entity,
}

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
        hovered: Color::srgb(0.4, 0.6, 1.0), // Se ilumina azul al pasar el ratón
        pressed: Color::srgb(0.2, 0.4, 0.8),
    };

    let mut cmds = parent.spawn((
        s,
        Button,
        bevy::ui::FocusPolicy::Block,
        Interaction::None,
        ImageNode::solid_color(Color::BLACK),
        colors.clone(),
        RuiResizer { dir, dragging: false, min_size, is_collapsed_prev: false, is_collapsed_next: false, last_cursor_pos: None },
        crate::focus::Focusable,
        Pickable::default(),
    ));

    let resizer_id = cmds.id();

    cmds.with_children(|p| {
        let btn_s = Node {
            display: Display::Flex,
            width: Val::Px(14.0),
            height: Val::Px(14.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(1.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };

        // Botón flecha Prev (Izquierda o Arriba)
        let prev_text = if dir == RuiResizerDir::Horizontal { "<" } else { "^" };
        let mut prev_text_id = Entity::PLACEHOLDER;
        p.spawn((
            btn_s.clone(), Button, ImageNode::solid_color(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), colors.clone(),
            Pickable::default(),
            bevy::ui::FocusPolicy::Block,
        )).with_children(|b| {
            prev_text_id = b.spawn((Text::new(prev_text), TextFont { font_size: 14.0, ..default() }, TextColor(Color::WHITE), Pickable::IGNORE)).id();
        }).insert(RuiResizerCollapseBtn { resizer_entity: resizer_id, is_prev: true, text_entity: prev_text_id });

        // Botón flecha Next (Derecha o Abajo)
        let next_text = if dir == RuiResizerDir::Horizontal { ">" } else { "v" };
        let mut next_text_id = Entity::PLACEHOLDER;
        p.spawn((
            btn_s, Button, ImageNode::solid_color(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), colors,
            Pickable::default(),
            bevy::ui::FocusPolicy::Block,
        )).with_children(|b| {
            next_text_id = b.spawn((Text::new(next_text), TextFont { font_size: 14.0, ..default() }, TextColor(Color::WHITE), Pickable::IGNORE)).id();
        }).insert(RuiResizerCollapseBtn { resizer_entity: resizer_id, is_prev: false, text_entity: next_text_id });
    });

    cmds
}

pub fn handle_resizer_drag(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_resizer: Query<(Entity, &Interaction, &mut RuiResizer, &ChildOf)>,
    q_children: Query<&Children>,
    mut q_node: Query<(&mut Node, &ComputedNode)>,
    mut q_window: Query<(Entity, &Window, Option<&mut bevy::window::CursorIcon>), With<bevy::window::PrimaryWindow>>,
    mut commands: Commands,
    q_collapse_btns: Query<&Interaction, With<RuiResizerCollapseBtn>>,
) {
    let (just_pressed, pressed, just_released) = (mouse.just_pressed(MouseButton::Left), mouse.pressed(MouseButton::Left), mouse.just_released(MouseButton::Left));
    
    let cursor_pos = q_window.single().ok().and_then(|(_, win, _)| win.cursor_position());
    let mut desired_cursor = None;

    for interaction in &q_collapse_btns {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            desired_cursor = Some(bevy::window::SystemCursorIcon::Pointer);
        }
    }

    for (entity, interaction, mut resizer, parent) in &mut q_resizer {
        let is_hovered = *interaction == Interaction::Hovered || *interaction == Interaction::Pressed;

        if resizer.is_collapsed_prev || resizer.is_collapsed_next {
            if just_released { resizer.dragging = false; resizer.last_cursor_pos = None; }
            continue; // Evitar el arrastre mientras un panel esté colapsado
        }

        if just_released { resizer.dragging = false; resizer.last_cursor_pos = None; }
        
        if just_pressed && is_hovered { 
            resizer.dragging = true; 
            resizer.last_cursor_pos = cursor_pos;
            
            // Al iniciar el arrastre, fijamos el flex_grow al tamaño exacto actual en píxeles.
            // Esto evita el ciclo de retraso y permite que el resizer se mueva 1:1 con el ratón.
            if let Ok(children) = q_children.get(parent.get()) {
                if let Some(pos) = children.iter().position(|e| e == entity) {
                    if pos > 0 && pos + 1 < children.len() {
                        let prev_entity = children[pos - 1];
                        let next_entity = children[pos + 1];
                        if let Ok([(mut prev_node, prev_comp), (mut next_node, next_comp)]) = q_node.get_many_mut([prev_entity, next_entity]) {
                            if resizer.dir == RuiResizerDir::Vertical {
                                prev_node.flex_grow = prev_comp.size().y;
                                next_node.flex_grow = next_comp.size().y;
                                prev_node.flex_basis = Val::Percent(0.0); next_node.flex_basis = Val::Percent(0.0);
                                prev_node.height = Val::Auto; next_node.height = Val::Auto;
                            } else {
                                prev_node.flex_grow = prev_comp.size().x;
                                next_node.flex_grow = next_comp.size().x;
                                prev_node.flex_basis = Val::Percent(0.0); next_node.flex_basis = Val::Percent(0.0);
                                prev_node.width = Val::Auto; next_node.width = Val::Auto;
                            }
                        }
                    }
                }
            }
        }

        if resizer.dragging || is_hovered {
            desired_cursor = Some(if resizer.dir == RuiResizerDir::Vertical {
                bevy::window::SystemCursorIcon::RowResize
            } else {
                bevy::window::SystemCursorIcon::ColResize
            });
        }

        if resizer.dragging && pressed {
            let mut delta = Vec2::ZERO;
            if let Some(cur_pos) = cursor_pos {
                if let Some(last_pos) = resizer.last_cursor_pos {
                    delta = cur_pos - last_pos;
                }
                resizer.last_cursor_pos = Some(cur_pos);
            }

            if delta != Vec2::ZERO {
            if let Ok(children) = q_children.get(parent.get()) {
                if let Some(pos) = children.iter().position(|e| e == entity) {
                    // Tomamos el panel de la izquierda (prev) y el de la derecha (next)
                    if pos > 0 && pos + 1 < children.len() {
                        let prev_entity = children[pos - 1];
                        let next_entity = children[pos + 1];

                        if let Ok([(mut prev_node, _), (mut next_node, _)]) = q_node.get_many_mut([prev_entity, next_entity]) {
                            
                            let delta_x = delta.x;
                            let delta_y = delta.y;
                            
                            let min_size = resizer.min_size; // Límite dinámico definido por el usuario
                            
                            if resizer.dir == RuiResizerDir::Vertical {
                                let total_h = prev_node.flex_grow + next_node.flex_grow;
                                if total_h > 0.0 {
                                    let mut new_prev_h = prev_node.flex_grow + delta_y;
                                    let mut new_next_h = next_node.flex_grow - delta_y;
                                    
                                    if new_prev_h < min_size { new_prev_h = min_size; new_next_h = total_h - min_size; }
                                    if new_next_h < min_size { new_next_h = min_size; new_prev_h = total_h - min_size; }
                                    
                                    prev_node.flex_grow = new_prev_h;
                                    next_node.flex_grow = new_next_h;
                                }
                            } else {
                                let total_w = prev_node.flex_grow + next_node.flex_grow;
                                if total_w > 0.0 {
                                    let mut new_prev_w = prev_node.flex_grow + delta_x;
                                    let mut new_next_w = next_node.flex_grow - delta_x;
                                    
                                    if new_prev_w < min_size { new_prev_w = min_size; new_next_w = total_w - min_size; }
                                    if new_next_w < min_size { new_next_w = min_size; new_prev_w = total_w - min_size; }
                                    
                                    prev_node.flex_grow = new_prev_w;
                                    next_node.flex_grow = new_next_w;
                                }
                            }
                        }
                    }
                }
            }
            }
        }
    }

    // Aplicar el cursor del sistema
    if let Ok((window_entity, _window, mut opt_cursor)) = q_window.single_mut() {
        if let Some(cursor) = desired_cursor {
            let new_icon = bevy::window::CursorIcon::System(cursor);
            if let Some(ref mut cursor_icon) = opt_cursor {
                if **cursor_icon != new_icon {
                    **cursor_icon = new_icon;
                }
            } else {
                commands.entity(window_entity).insert(new_icon);
            }
        } else {
            let default_icon = bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::Default);
            if let Some(ref mut cursor_icon) = opt_cursor {
                let is_col = matches!(**cursor_icon, bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::ColResize));
                let is_row = matches!(**cursor_icon, bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::RowResize));
                let is_ptr = matches!(**cursor_icon, bevy::window::CursorIcon::System(bevy::window::SystemCursorIcon::Pointer));
                if is_col || is_row || is_ptr {
                    **cursor_icon = default_icon;
                }
            }
        }
    }
}

pub fn handle_resizer_collapse_clicks(
    mut interactions: Query<(&Interaction, &RuiResizerCollapseBtn), Changed<Interaction>>,
    mut q_resizer: Query<(&mut RuiResizer, &ChildOf)>,
    q_children: Query<&Children>,
    mut q_node: Query<(&mut Node, &ComputedNode)>,
    mut q_text: Query<&mut Text>,
    q_all_btns: Query<(Entity, &RuiResizerCollapseBtn)>,
) {
    for (interaction, btn) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if let Ok((mut resizer, parent)) = q_resizer.get_mut(btn.resizer_entity) {
                
                // Si el panel opuesto está colapsado, ignoramos el clic por seguridad
                if btn.is_prev && resizer.is_collapsed_next { continue; }
                if !btn.is_prev && resizer.is_collapsed_prev { continue; }

                if let Ok(children) = q_children.get(parent.get()) {
                    if let Some(pos) = children.iter().position(|e| e == btn.resizer_entity) {
                        if pos > 0 && pos + 1 < children.len() {
                            let prev_entity = children[pos - 1];
                            let next_entity = children[pos + 1];

                            // Inicializar flex_grow si es la primera vez (al igual que al arrastrar),
                            // para asegurar que el panel no colapsado se expanda al 100%
                            if let Ok([(mut prev_node, prev_comp), (mut next_node, next_comp)]) = q_node.get_many_mut([prev_entity, next_entity]) {
                                if prev_node.flex_basis != Val::Percent(0.0) {
                                    if resizer.dir == RuiResizerDir::Vertical {
                                        prev_node.flex_grow = prev_comp.size().y;
                                        next_node.flex_grow = next_comp.size().y;
                                        prev_node.flex_basis = Val::Percent(0.0); next_node.flex_basis = Val::Percent(0.0);
                                        prev_node.height = Val::Auto; next_node.height = Val::Auto;
                                    } else {
                                        prev_node.flex_grow = prev_comp.size().x;
                                        next_node.flex_grow = next_comp.size().x;
                                        prev_node.flex_basis = Val::Percent(0.0); next_node.flex_basis = Val::Percent(0.0);
                                        prev_node.width = Val::Auto; next_node.width = Val::Auto;
                                    }
                                }
                            }

                            if btn.is_prev {
                                resizer.is_collapsed_prev = !resizer.is_collapsed_prev;
                                if let Ok((mut node, _)) = q_node.get_mut(prev_entity) {
                                    node.display = if resizer.is_collapsed_prev { Display::None } else { Display::Flex };
                                }
                            } else {
                                resizer.is_collapsed_next = !resizer.is_collapsed_next;
                                if let Ok((mut node, _)) = q_node.get_mut(next_entity) {
                                    node.display = if resizer.is_collapsed_next { Display::None } else { Display::Flex };
                                }
                            }

                            // Actualizamos el icono de la flecha para indicar en qué dirección expandir
                            if let Ok(mut text) = q_text.get_mut(btn.text_entity) {
                                if btn.is_prev {
                                    if resizer.dir == RuiResizerDir::Horizontal {
                                        text.0 = if resizer.is_collapsed_prev { ">".to_string() } else { "<".to_string() };
                                    } else {
                                        text.0 = if resizer.is_collapsed_prev { "v".to_string() } else { "^".to_string() };
                                    }
                                } else {
                                    if resizer.dir == RuiResizerDir::Horizontal {
                                        text.0 = if resizer.is_collapsed_next { "<".to_string() } else { ">".to_string() };
                                    } else {
                                        text.0 = if resizer.is_collapsed_next { "^".to_string() } else { "v".to_string() };
                                    }
                                }
                            }

                            // Ocultamos/Mostramos la flecha contraria para evitar que el usuario colapse ambos lados
                            for (sibling_entity, sibling_btn) in &q_all_btns {
                                if sibling_btn.resizer_entity == btn.resizer_entity {
                                    if let Ok((mut sibling_node, _)) = q_node.get_mut(sibling_entity) {
                                        if sibling_btn.is_prev {
                                            // Ocultar la flecha previa (izq/arriba) si la siguiente está colapsada
                                            sibling_node.display = if resizer.is_collapsed_next { Display::None } else { Display::Flex };
                                        } else {
                                            // Ocultar la flecha siguiente (der/abajo) si la previa está colapsada
                                            sibling_node.display = if resizer.is_collapsed_prev { Display::None } else { Display::Flex };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}