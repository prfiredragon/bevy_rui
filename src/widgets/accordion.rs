use bevy::prelude::*;
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiAccordionToggle {
    pub content_entity: Entity,
    pub is_open: bool,
}

pub fn spawn_accordion<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    title: &str,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
        let mut s = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            ..default()
        };
        modifier(&mut s);

        let mut cmds = parent_cmd.spawn(s);
        cmds.with_children(|parent| {
            let header_id = parent.spawn((
                Node {
                    display: Display::Flex, width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)), justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                Button,
                crate::focus::Focusable,
                ImageNode::solid_color(Color::srgb(0.16, 0.16, 0.17)),
                BorderColor::all(Color::srgb(0.1, 0.1, 0.11)),
                RuiButtonStateColors {
                    normal: Color::srgb(0.16, 0.16, 0.17),
                    hovered: Color::srgb(0.2, 0.2, 0.21),
                    pressed: Color::srgb(0.12, 0.12, 0.13),
                },
            )).with_children(|header| {
                header.spawn((Text::new(title), TextFont { font_size: 14.0, ..default() }, TextColor(Color::WHITE)));
                header.spawn((Text::new("▶"), TextFont { font_size: 12.0, ..default() }, TextColor(Color::srgb(0.6, 0.6, 0.6))));
            }).id();

            let content_id = parent.spawn((
                Node { 
                    display: Display::None, flex_direction: FlexDirection::Column, width: Val::Percent(100.0), 
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)), 
                    border: UiRect::left(Val::Px(2.0)),
                    ..default() 
                },
                ImageNode::solid_color(Color::srgb(0.12, 0.12, 0.12)),
                BorderColor::all(Color::srgb(0.2, 0.2, 0.22)),
            )).with_children(children).id();

            parent.commands().entity(header_id).insert(RuiAccordionToggle { content_entity: content_id, is_open: false });
        });
        cmds
}

pub fn handle_accordion_clicks(
    mut query: Query<(&Interaction, &mut RuiAccordionToggle, &Children), Changed<Interaction>>,
    mut q_content: Query<&mut Node>,
    mut q_text: Query<&mut Text>,
) {
    for (interaction, mut toggle, children) in &mut query {
        if *interaction == Interaction::Pressed {
            toggle.is_open = !toggle.is_open;
            if let Ok(mut content_node) = q_content.get_mut(toggle.content_entity) {
                content_node.display = if toggle.is_open { Display::Flex } else { Display::None };
            }
            if children.len() > 1 {
                if let Ok(mut text) = q_text.get_mut(children[1]) {
                    text.0 = if toggle.is_open { "▼".to_string() } else { "▶".to_string() };
                }
            }
        }
    }
}