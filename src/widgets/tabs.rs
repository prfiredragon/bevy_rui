use bevy::{ecs::relationship::Relationship, prelude::*};
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiTabContainer {
    pub active_tab: usize,
}

#[derive(Component)]
pub struct RuiTabButton {
    pub container_entity: Entity,
    pub tab_index: usize,
}

#[derive(Component)]
pub struct RuiTabContent {
    pub container_entity: Entity,
    pub tab_index: usize,
}

#[derive(Component)]
pub struct RuiTabCloseButton {
    pub container_entity: Entity,
    pub tab_index: usize,
}

pub struct RuiTabsBuilder<'a> {
    tabs: Vec<(String, bool, Box<dyn FnOnce(&mut Node, &mut ImageNode) + 'a>, Box<dyn FnOnce(&mut ChildSpawnerCommands) + 'a>)>,
}

impl<'a> RuiTabsBuilder<'a> {
    pub fn tab(&mut self, label: &str, closable: bool, modifier: impl FnOnce(&mut Node, &mut ImageNode) + 'a, content_builder: impl FnOnce(&mut ChildSpawnerCommands) + 'a) {
        self.tabs.push((label.to_string(), closable, Box::new(modifier), Box::new(content_builder)));
    }
}

pub fn spawn_tabs<'a>(
    parent: &'a mut ChildSpawnerCommands,
    active_tab: usize,
    modifier: impl FnOnce(&mut Node),
    build_tabs: impl FnOnce(&mut RuiTabsBuilder),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        ..default()
    };
    modifier(&mut s);

    let mut builder = RuiTabsBuilder { tabs: Vec::new() };
    build_tabs(&mut builder);

    let mut cmds = parent.spawn((
        s,
        RuiTabContainer { active_tab },
    ));
    
    let container_entity = cmds.id();

    cmds.with_children(|p| {
        // 1. Tab Header (Cinta superior de pestañas)
        p.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            ImageNode::solid_color(Color::srgb(0.08, 0.08, 0.09)),
        )).with_children(|header| {
            for (i, (label, closable, _, _)) in builder.tabs.iter().enumerate() {
                let is_active = i == active_tab;
                header.spawn((
                    Button,
                    Node {
                        height: Val::Percent(100.0),
                        padding: UiRect::horizontal(Val::Px(12.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::top(Val::Px(2.0)),
                        ..default()
                    },
                    ImageNode::solid_color(if is_active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) }),
                    BorderColor::all(if is_active { Color::srgb(0.4, 0.6, 1.0) } else { Color::NONE }),
                    RuiButtonStateColors {
                        normal: if is_active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) },
                        hovered: if is_active { Color::srgb(0.25, 0.25, 0.27) } else { Color::srgb(0.18, 0.18, 0.2) },
                        pressed: Color::srgb(0.1, 0.1, 0.12),
                    },
                    RuiTabButton { container_entity, tab_index: i },
                    crate::focus::Focusable,
                    Pickable::default(),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(if is_active { Color::WHITE } else { Color::srgb(0.6, 0.6, 0.6) }),
                        Pickable::IGNORE,
                    ));

                    if *closable {
                        btn.spawn((
                            Node { margin: UiRect::left(Val::Px(8.0)), ..default() },
                            Button,
                            RuiButtonStateColors { normal: Color::NONE, hovered: Color::srgb(0.8, 0.2, 0.2), pressed: Color::srgb(0.6, 0.1, 0.1) },
                            ImageNode::solid_color(Color::NONE),
                            RuiTabCloseButton { container_entity, tab_index: i },
                            crate::focus::Focusable,
                            Pickable::default(),
                            bevy::ui::FocusPolicy::Block,
                        )).with_children(|close_btn| {
                            close_btn.spawn((
                                Text::new("x"), TextFont { font_size: 14.0, ..default() }, TextColor(Color::srgb(0.8, 0.4, 0.4)),
                                Pickable::IGNORE,
                            ));
                        });
                    }
                });
            }
        });

        // 2. Tab Content Area (El cuerpo donde va el contenido)
        p.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                flex_shrink: 1.0,
                overflow: Overflow::clip(),
                ..default()
            },
        )).with_children(|content_parent| {
            for (i, (_, _, modifier, content_builder)) in builder.tabs.into_iter().enumerate() {
                let is_active = i == active_tab;

                let mut node = Node {
                    display: if is_active { Display::Flex } else { Display::None },
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    flex_shrink: 1.0,
                    ..default()
                };
                let mut bg = ImageNode::solid_color(Color::srgb(0.12, 0.12, 0.14));
                modifier(&mut node, &mut bg);

                content_parent.spawn((
                    node,
                    bg,
                    RuiTabContent { container_entity, tab_index: i },
                )).with_children(|c| {
                    content_builder(c);
                });
            }
        });
    });

    cmds
}

/// Sistema para gestionar los clics en las pestañas y la ocultación del contenido
pub fn handle_tab_clicks(
    mut interactions: Query<(Entity, &Interaction, &RuiTabButton), Changed<Interaction>>,
    mut containers: Query<&mut RuiTabContainer>,
    mut contents: Query<(&mut Node, &RuiTabContent)>,
    mut buttons: Query<(&mut ImageNode, &mut BorderColor, &mut RuiButtonStateColors, &RuiTabButton)>,
    mut texts: Query<(&mut TextColor, &ChildOf)>,
    mut input_focus: ResMut<bevy::input_focus::InputFocus>,
) {
    for (entity, interaction, button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if let Ok(mut container) = containers.get_mut(button.container_entity) {
                if container.active_tab == button.tab_index { continue; }
                container.active_tab = button.tab_index;
                
                input_focus.set(entity);
                
                // Actualizar la visibilidad del contenido
                for (mut node, content) in &mut contents {
                    if content.container_entity == button.container_entity {
                        node.display = if content.tab_index == button.tab_index { Display::Flex } else { Display::None };
                    }
                }

                // Actualizar el estado visual del botón
                for (mut bg, mut border, mut state_colors, btn) in &mut buttons {
                    if btn.container_entity == button.container_entity {
                        let is_active = btn.tab_index == button.tab_index;
                        *border = BorderColor::all(if is_active { Color::srgb(0.4, 0.6, 1.0) } else { Color::NONE });
                        state_colors.normal = if is_active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) };
                        state_colors.hovered = if is_active { Color::srgb(0.25, 0.25, 0.27) } else { Color::srgb(0.18, 0.18, 0.2) };
                        bg.color = state_colors.normal;
                    }
                }

                // Actualizar el color del texto
                for (mut text_color, parent) in &mut texts {
                    if let Ok((_, _, _, btn)) = buttons.get(parent.get()) {
                        if btn.container_entity == button.container_entity {
                            text_color.0 = if btn.tab_index == button.tab_index { Color::WHITE } else { Color::srgb(0.6, 0.6, 0.6) };
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_tab_close_clicks(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &RuiTabCloseButton), Changed<Interaction>>,
    mut containers: Query<&mut RuiTabContainer>,
    buttons: Query<(Entity, &RuiTabButton)>,
    mut contents: Query<(Entity, &mut Node, &RuiTabContent)>,
    mut button_visuals: Query<(&mut ImageNode, &mut BorderColor, &mut RuiButtonStateColors, &RuiTabButton)>,
    mut texts: Query<(&mut TextColor, &ChildOf)>,
) {
    for (interaction, close_btn) in &mut interactions {
        if *interaction == Interaction::Pressed {
            for (entity, btn) in &buttons {
                if btn.container_entity == close_btn.container_entity && btn.tab_index == close_btn.tab_index { commands.entity(entity).despawn(); }
            }
            for (entity, _, content) in &contents {
                if content.container_entity == close_btn.container_entity && content.tab_index == close_btn.tab_index { commands.entity(entity).despawn(); }
            }

            // Pick new active tab
            if let Ok(mut container) = containers.get_mut(close_btn.container_entity) {
                if container.active_tab == close_btn.tab_index {
                    let next_active = buttons.iter().find_map(|(_, btn)| {
                        if btn.container_entity == close_btn.container_entity && btn.tab_index != close_btn.tab_index { Some(btn.tab_index) } else { None }
                    });
                    if let Some(idx) = next_active {
                        container.active_tab = idx;
                        
                        // Update visual states for the new active tab manually
                        for (_, mut node, content) in &mut contents {
                            if content.container_entity == close_btn.container_entity {
                                node.display = if content.tab_index == idx { Display::Flex } else { Display::None };
                            }
                        }
                        for (mut bg, mut border, mut state_colors, btn) in &mut button_visuals {
                            if btn.container_entity == close_btn.container_entity {
                                let is_active = btn.tab_index == idx;
                                *border = BorderColor::all(if is_active { Color::srgb(0.4, 0.6, 1.0) } else { Color::NONE });
                                state_colors.normal = if is_active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) };
                                state_colors.hovered = if is_active { Color::srgb(0.25, 0.25, 0.27) } else { Color::srgb(0.18, 0.18, 0.2) };
                                bg.color = state_colors.normal;
                            }
                        }
                        for (mut text_color, parent) in &mut texts {
                            if let Ok((_, _, _, btn)) = button_visuals.get(parent.get()) {
                                if btn.container_entity == close_btn.container_entity {
                                    text_color.0 = if btn.tab_index == idx { Color::WHITE } else { Color::srgb(0.6, 0.6, 0.6) };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}