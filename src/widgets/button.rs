use bevy::prelude::*;

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

pub fn spawn_button<'a>(
    parent: &'a mut ChildSpawnerCommands,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
    };
    modifier(&mut s);

    let mut cmds = parent.spawn((
        s,
        Button,
        bevy::ui::FocusPolicy::Block,
        Pickable::default(),
        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::default() },
        crate::theme::RuiThemeElement::Button,
        crate::focus::Focusable,
    ));
    cmds.with_children(children);
    cmds
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct RuiSelected;

pub fn handle_button_colors(
    mut query: Query<(Entity, &Interaction, &mut ImageNode, Option<&crate::theme::RuiThemeElement>, Option<&RuiButtonStateColors>, Has<RuiSelected>)>,
    theme: Res<crate::theme::RuiTheme>,
    input_focus: Res<bevy::input_focus::InputFocus>,
) {
    let _focus_changed = input_focus.is_changed();
    
    for (entity, interaction, mut image_node, theme_element, state_colors, is_selected) in &mut query {
        let _interaction_changed = true; // In a real scenario we'd track this via a local system parameter or ChangeTrackers, but checking color/image equality is cheap enough
        
        let is_focused = input_focus.get() == Some(entity);
        let is_pressed = *interaction == Interaction::Pressed || is_selected;
        let is_hovered = *interaction == Interaction::Hovered || (is_focused && !is_pressed);

        if let Some(colors) = state_colors {
            let target_color = if is_pressed { colors.pressed }
            else if is_hovered { colors.hovered }
            else { colors.normal };
            
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