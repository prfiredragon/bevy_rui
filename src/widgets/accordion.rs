//! Collapsible accordion widget for `bevy_rui`.
//!
//! Spawns a clickable header button followed by a collapsible content container.
//! Toggles open/closed state automatically via an `On<Activate>` observer.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::text::FontSize;
use crate::widgets::RuiButtonStateColors;

/// Component stored on an accordion header entity tracking its collapsible content container.
#[derive(Component)]
pub struct RuiAccordionToggle {
    /// The child container entity whose `Node::display` is toggled between `Flex` and `None`.
    pub content_entity: Entity,
    /// Whether the accordion section is currently expanded.
    pub is_open: bool,
}

/// Spawns a collapsible accordion section with `title`, a `modifier` for the outer node,
/// and `children` populated inside the collapsible body.
///
/// Also available via [`crate::widgets::RuiBuilderExt::accordion`].
pub fn spawn_accordion<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    title: &str,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'w> {
        let mut s = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            ..default()
        };
        modifier(&mut s);

        let mut cmds = parent_cmd.spawn(s);
        cmds.with_children(|parent| {
            // Header is spawned first, followed by the content container so it expands downward in a column layout.
            let header_id = parent.spawn((
                Node {
                    display: Display::Flex, 
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)), 
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                bevy::ui_widgets::Button,
                crate::focus::Focusable,
                ImageNode::default(),
                BackgroundColor(Color::srgb(0.16, 0.16, 0.17)),
                BorderColor::from(Color::srgb(0.1, 0.1, 0.11)),
                RuiButtonStateColors {
                    normal: Color::srgb(0.16, 0.16, 0.17),
                    hovered: Color::srgb(0.2, 0.2, 0.21),
                    pressed: Color::srgb(0.12, 0.12, 0.13),
                },
            )).with_children(|header| {
                header.spawn((
                    Text::new(title), 
                    TextFont { font_size: FontSize::Px(14.0), ..default() }, 
                    TextColor(Color::WHITE)
                ));
                header.spawn((
                    Text::new("▶"), 
                    TextFont { font_size: FontSize::Px(12.0), ..default() }, 
                    TextColor(Color::srgb(0.6, 0.6, 0.6))
                ));
            }).id();

            let content_id = parent.spawn((
                Node { 
                    display: Display::None, 
                    flex_direction: FlexDirection::Column, 
                    width: Val::Percent(100.0), 
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)), 
                    border: UiRect::left(Val::Px(2.0)),
                    ..default() 
                },
                ImageNode::default(),
                BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
                BorderColor::from(Color::srgb(0.2, 0.2, 0.22)),
            )).with_children(children).id();

            parent.commands().entity(header_id).insert(RuiAccordionToggle {
                content_entity: content_id,
                is_open: false,
            });

            // Reactive observer toggling content visibility and arrow indicator on click/activation.
            parent.commands().entity(header_id).observe(
                |trigger: On<Activate>, 
                mut q_toggle: Query<&mut RuiAccordionToggle>, 
                q_children: Query<&Children>,
                mut q_content: Query<&mut Node>,
                mut q_text: Query<&mut Text>| 
                {
                    let target_entity = trigger.entity; 
                    
                    if let Ok(mut toggle) = q_toggle.get_mut(target_entity) {
                        toggle.is_open = !toggle.is_open;
                        
                        if let Ok(mut content_node) = q_content.get_mut(toggle.content_entity) {
                            content_node.display = if toggle.is_open { Display::Flex } else { Display::None };
                        }
                        
                        if let Ok(children) = q_children.get(target_entity) {
                            if children.len() > 1 {
                                if let Ok(mut text) = q_text.get_mut(children[1]) {
                                    text.0 = if toggle.is_open { "▼".to_string() } else { "▶".to_string() };
                                }
                            }
                        }
                    }
                }
            );

        });
        cmds
}
