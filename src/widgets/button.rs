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
        ImageNode::default(),
        crate::theme::RuiThemeElement::Button,
    ));
    cmds.with_children(children);
    cmds
}

pub fn handle_button_colors(
    mut query: Query<(&Interaction, &mut ImageNode, Option<&crate::theme::RuiThemeElement>, Option<&RuiButtonStateColors>), Changed<Interaction>>,
    theme: Res<crate::theme::RuiTheme>,
) {
    for (interaction, mut image_node, theme_element, state_colors) in &mut query {
        if let Some(colors) = state_colors {
            match *interaction { 
                Interaction::Pressed => { image_node.color = colors.pressed; }
                Interaction::Hovered => { image_node.color = colors.hovered; }
                Interaction::None => { image_node.color = colors.normal; }
            }
        } else if let Some(crate::theme::RuiThemeElement::Button) = theme_element {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(ref img) = theme.image_button_pressed {
                        image_node.image = img.clone();
                        image_node.color = Color::WHITE;
                    } else {
                        image_node.color = theme.color_button_pressed;
                    }
                }
                Interaction::Hovered => {
                    if let Some(ref img) = theme.image_button_hover {
                        image_node.image = img.clone();
                        image_node.color = Color::WHITE;
                    } else {
                        image_node.color = theme.color_button_hover;
                    }
                }
                Interaction::None => {
                    if let Some(ref img) = theme.image_button_normal {
                        image_node.image = img.clone();
                        image_node.color = Color::WHITE;
                    } else {
                        image_node.color = theme.color_button_normal;
                    }
                }
            }
        }
    }
}