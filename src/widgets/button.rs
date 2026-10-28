//! Interactive button widget for `bevy_rui`.
//!
//! Spawns a headless Bevy [`bevy::ui_widgets::Button`] styled via [`crate::theme::RuiThemeElement::Button`]
//! or per-entity [`RuiButtonStateColors`]. Listen for clicks by attaching `.observe(|_: On<Activate>| ...)`.

use bevy::prelude::*;
use bevy::picking::hover::Hovered;
use bevy::ui::Pressed;
use bevy::input_focus::InputFocus;

/// Optional component overriding the default theme colors for a button's normal, hovered, and pressed states.
#[derive(Component, Clone)]
pub struct RuiButtonStateColors {
    /// Background/tint color in the normal idle state.
    pub normal: Color,
    /// Background/tint color when hovered or focused.
    pub hovered: Color,
    /// Background/tint color when pressed or selected ([`RuiSelected`]).
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

/// Spawns an interactive button with [`crate::theme::RuiThemeElement::Button`] and [`crate::focus::Focusable`].
///
/// Attach `.observe(|trigger: On<bevy::ui_widgets::Activate>| ...)` to the returned [`EntityCommands`]
/// to handle click and keyboard/gamepad activation events.
///
/// Also available via [`crate::widgets::RuiBuilderExt::button`].
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

    let mut cmds = parent.spawn((
        s,
        bevy::ui_widgets::Button,
        ImageNode::default(),
        crate::theme::RuiThemeElement::Button,
        crate::focus::Focusable, 
    ));
    
    cmds.with_children(children);
    cmds
}

/// Marker component indicating that a button or list item is currently in a selected state.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct RuiSelected;

/// System that updates button and list item colors/textures based on [`Hovered`], [`Pressed`], [`InputFocus`], and [`RuiSelected`].
pub fn handle_button_colors(
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
        
        let is_pressed = pressed.is_some() || is_selected;
        let is_hovered = hovered.is_some() || (is_focused && !is_pressed);

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
