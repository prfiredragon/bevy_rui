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
                    padding: UiRect::all(Val::Px(8.0)), justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                Button,
                ImageNode::solid_color(Color::srgb(0.2, 0.2, 0.2)),
                RuiButtonStateColors::default(),
            )).with_children(|header| {
                header.spawn((Text::new(title), TextFont::default(), TextColor(Color::WHITE)));
                header.spawn((Text::new("▼"), TextFont::default(), TextColor(Color::WHITE)));
            }).id();

            let content_id = parent.spawn((
                Node { display: Display::None, flex_direction: FlexDirection::Column, width: Val::Percent(100.0), padding: UiRect::all(Val::Px(10.0)), ..default() },
                ImageNode::solid_color(Color::srgb(0.12, 0.12, 0.12)),
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
                    text.0 = if toggle.is_open { "▲".to_string() } else { "▼".to_string() };
                }
            }
        }
    }
}