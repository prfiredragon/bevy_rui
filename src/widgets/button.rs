use bevy::prelude::*;
use bevy::picking::hover::Hovered;
use bevy::ui::Pressed;
use bevy::input_focus::InputFocus;

#[derive(Component, Clone)]
pub struct RuiButtonStateColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl Default for RuiButtonStateColors {
    fn default() -> Self {
        Self {
            normal: Color::srgb(0.2, 0.2, 0.2),
            hovered: Color::srgb(0.3, 0.3, 0.3),
            pressed: Color::srgb(0.1, 0.1, 0.1),
        }
    }
}

// CORRECCIÓN: Introducción explícita del tiempo de vida 'w para ligar el Spawner de origen con los comandos retornados
pub fn spawn_button<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
    };
    modifier(&mut s);

    // Ajustado a los componentes headless modernos de Bevy 0.20
    let mut cmds = parent.spawn((
        s,
        bevy::ui_widgets::Button, // Componente marcador headless nativo
        ImageNode::default(),     // Ya no contiene visual_box o image_mode heredados
        crate::theme::RuiThemeElement::Button,
        crate::focus::Focusable, 
    ));
    
    cmds.with_children(children);
    cmds
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct RuiSelected;

pub fn handle_button_colors(
    // En Bevy 0.20 reemplazamos `&Interaction` por los componentes reactivos Option<&Hovered> y Option<&Pressed>
    mut query: Query<(
        Entity, 
        Option<&Hovered>, 
        Option<&Pressed>, 
        &mut ImageNode, 
        Option<&crate::theme::RuiThemeElement>, 
        Option<&RuiButtonStateColors>, 
        Has<RuiSelected>
    )>,
    theme: Res<crate::theme::RuiTheme>,
    input_focus: Res<InputFocus>,
) {
    for (entity, hovered, pressed, mut image_node, theme_element, state_colors, is_selected) in &mut query {
        let is_focused = input_focus.get() == Some(entity);
        
        // El estado se evalúa verificando si el componente respectivo está presente (is_some())
        let is_pressed = pressed.is_some() || is_selected;
        let is_hovered = hovered.is_some() || (is_focused && !is_pressed);

        if let Some(colors) = state_colors {
            let target_color = if is_pressed { colors.pressed }
            else if is_hovered { colors.hovered }
            else { colors.normal };
            
            // En Bevy 0.20, el tinte/color de un ImageNode vive directamente en su propiedad .color
            if image_node.color != target_color {
                image_node.color = target_color;
            }
        } else if let Some(element) = theme_element {
            let (target_color, target_image) = match element {
                crate::theme::RuiThemeElement::Button => {
                    if is_pressed {
                        if let Some(ref img) = theme.image_button_pressed { (Color::WHITE, Some(img.clone())) }
                        else { (theme.color_button_pressed, None) }
                    } else if is_hovered {
                        if let Some(ref img) = theme.image_button_hover { (Color::WHITE, Some(img.clone())) }
                        else { (theme.color_button_hover, None) }
                    } else {
                        if let Some(ref img) = theme.image_button_normal { (Color::WHITE, Some(img.clone())) }
                        else { (theme.color_button_normal, None) }
                    }
                },
                crate::theme::RuiThemeElement::ListItem => {
                    if is_pressed {
                        if let Some(ref img) = theme.image_list_item_pressed { (Color::WHITE, Some(img.clone())) }
                        else { (theme.color_list_item_pressed, None) }
                    } else if is_hovered {
                        if let Some(ref img) = theme.image_list_item_hover { (Color::WHITE, Some(img.clone())) }
                        else { (theme.color_list_item_hover, None) }
                    } else {
                        if let Some(ref img) = theme.image_list_item_normal { (Color::WHITE, Some(img.clone())) }
                        else { (theme.color_list_item_normal, None) }
                    }
                },
                crate::theme::RuiThemeElement::Tab => {
                    if is_pressed {
                        if let Some(ref img) = theme.image_tab_active { (Color::WHITE, Some(img.clone())) }
                        else { (Color::srgb(0.1, 0.1, 0.12), None) }
                    } else if is_hovered {
                        if let Some(ref img) = theme.image_tab_hover { (Color::WHITE, Some(img.clone())) }
                        else { (Color::srgb(0.18, 0.18, 0.2), None) }
                    } else {
                        if let Some(ref img) = theme.image_tab_normal { (Color::WHITE, Some(img.clone())) }
                        else { (Color::srgb(0.12, 0.12, 0.14), None) }
                    }
                },
                crate::theme::RuiThemeElement::TabActive => {
                    if let Some(ref img) = theme.image_tab_active { (Color::WHITE, Some(img.clone())) }
                    else { (Color::srgb(0.2, 0.2, 0.22), None) }
                },
                _ => continue,
            };

            if image_node.color != target_color { image_node.color = target_color; }
            if let Some(img) = target_image {
                if image_node.image != img { image_node.image = img; }
            }
        }
    }
}
