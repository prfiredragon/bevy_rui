use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::text::FontSize;
use crate::widgets::RuiButtonStateColors;

use bevy::ecs::relationship::Relationship; 

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
    pub fn tab(
        &mut self, 
        label: &str, 
        closable: bool, 
        modifier: impl FnOnce(&mut Node, &mut ImageNode) + 'a, 
        content_builder: impl FnOnce(&mut ChildSpawnerCommands) + 'a
    ) {
        self.tabs.push((label.to_string(), closable, Box::new(modifier), Box::new(content_builder)));
    }
}

pub fn spawn_tabs<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    active_tab: usize,
    modifier: impl FnOnce(&mut Node),
    build_tabs: impl FnOnce(&mut RuiTabsBuilder),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        min_width: Val::Px(0.0),
        min_height: Val::Px(0.0),
        flex_grow: 1.0,
        ..default()
    };
    modifier(&mut s);

    let mut builder = RuiTabsBuilder { tabs: Vec::new() };
    build_tabs(&mut builder);

    let mut cmds = parent.spawn((s, RuiTabContainer { active_tab }));
    let container_entity = cmds.id();

    cmds.with_children(|p| {
        // Cinta superior de pestañas (Header)
        p.spawn((
            Node {
                display: Display::Flex, flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0), height: Val::Px(30.0), align_items: AlignItems::FlexEnd,
                ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.09)),
        )).with_children(|header| {
            for (i, (label, closable, _, _)) in builder.tabs.iter().enumerate() {
                let is_active = i == active_tab;
                
                let btn_id = header.spawn((
                    bevy::ui_widgets::Button,
                    Node {
                        height: Val::Percent(100.0), padding: UiRect::horizontal(Val::Px(12.0)),
                        align_items: AlignItems::Center, justify_content: JustifyContent::Center,
                        border: UiRect::top(Val::Px(2.0)), ..default()
                    },
                    ImageNode::default(),
                    BackgroundColor(if is_active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) }),
                    BorderColor::from(if is_active { Color::srgb(0.4, 0.6, 1.0) } else { Color::NONE }),
                    if is_active { crate::theme::RuiThemeElement::TabActive } else { crate::theme::RuiThemeElement::Tab },
                    RuiTabButton { container_entity, tab_index: i },
                    crate::focus::Focusable,
                    Pickable::default(),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                        TextColor(if is_active { Color::WHITE } else { Color::srgb(0.6, 0.6, 0.6) }),
                        Pickable::IGNORE,
                    ));

                    if *closable {
                        let close_id = btn.spawn((
                            Node { margin: UiRect::left(Val::Px(8.0)), ..default() },
                            bevy::ui_widgets::Button,
                            RuiButtonStateColors { normal: Color::NONE, hovered: Color::srgb(0.8, 0.2, 0.2), pressed: Color::srgb(0.6, 0.1, 0.1) },
                            ImageNode::default(), BackgroundColor(Color::NONE),
                            RuiTabCloseButton { container_entity, tab_index: i },
                            crate::focus::Focusable, Pickable::default(),
                        )).with_children(|close_btn| {
                            close_btn.spawn((
                                Text::new("x"), TextFont { font_size: FontSize::Px(14.0), ..default() }, 
                                TextColor(Color::srgb(0.8, 0.4, 0.4)), Pickable::IGNORE,
                            ));
                        }).id();

                        // OBSERVADOR REACTIVO: Cierre de pestaña
                        btn.commands().entity(close_id).observe(
                            move |trigger: On<Activate>,
                                  mut commands: Commands,
                                  q_close: Query<&RuiTabCloseButton>,
                                  mut q_containers: Query<&mut RuiTabContainer>,
                                  q_buttons: Query<(Entity, &RuiTabButton)>,
                                  mut q_contents: Query<(Entity, &mut Node, &RuiTabContent), Without<RuiTabButton>>,
                                  mut q_btn_styles: Query<(&mut crate::theme::RuiThemeElement, &mut BorderColor, &mut BackgroundColor, &RuiTabButton), Without<RuiTabContent>>,
                                  mut q_texts: Query<(&mut TextColor, &ChildOf)>| // CORRECCIÓN: &ChildOf nativo
                            {
                                if let Ok(close_btn) = q_close.get(trigger.entity) {
                                    for (entity, btn) in &q_buttons {
                                        if btn.container_entity == close_btn.container_entity && btn.tab_index == close_btn.tab_index { 
                                            commands.entity(entity).try_despawn(); 
                                        }
                                    }
                                    for (entity, _, content) in &q_contents {
                                        if content.container_entity == close_btn.container_entity && content.tab_index == close_btn.tab_index { 
                                            commands.entity(entity).try_despawn(); 
                                        }
                                    }

                                    if let Ok(mut container) = q_containers.get_mut(close_btn.container_entity) {
                                        if container.active_tab == close_btn.tab_index {
                                            let next_active = q_buttons.iter().find_map(|(_, btn)| {
                                                if btn.container_entity == close_btn.container_entity && btn.tab_index != close_btn.tab_index { 
                                                    Some(btn.tab_index) 
                                                } else { None }
                                            });

                                            if let Some(idx) = next_active {
                                                container.active_tab = idx;
                                                for (_, mut node, content) in &mut q_contents {
                                                    if content.container_entity == close_btn.container_entity {
                                                        node.display = if content.tab_index == idx { Display::Flex } else { Display::None };
                                                    }
                                                }
                                                for (mut element, mut border, mut bg, btn) in &mut q_btn_styles {
                                                    if btn.container_entity == close_btn.container_entity {
                                                        let active = btn.tab_index == idx;
                                                        *element = if active { crate::theme::RuiThemeElement::TabActive } else { crate::theme::RuiThemeElement::Tab };
                                                        *border = BorderColor::from(if active { Color::srgb(0.4, 0.6, 1.0) } else { Color::NONE });
                                                        *bg = BackgroundColor(if active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) });
                                                    }
                                                }
                                                for (mut text_color, parent) in &mut q_texts {
                                                    // CORRECCIÓN: .find_map retorna un Option, evaluamos con `if let Some`
                                                    if let Some(btn) = q_buttons.iter().find_map(|(e, b)| if e == parent.get() { Some(b) } else { None }) {
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
                        );
                    }
                }).id();
                // OBSERVADOR REACTIVO: Intercambio normal de pestañas activas
                header.commands().entity(btn_id).observe(
                    move |trigger: On<Activate>,
                          mut q_containers: Query<&mut RuiTabContainer>,
                          q_buttons: Query<&RuiTabButton>,
                          mut q_contents: Query<(&mut Node, &RuiTabContent), Without<RuiTabButton>>,
                          mut q_btn_styles: Query<(&mut crate::theme::RuiThemeElement, &mut BorderColor, &mut BackgroundColor, &RuiTabButton), Without<RuiTabContent>>,
                          mut q_texts: Query<(&mut TextColor, &ChildOf)>| // CORRECCIÓN: &ChildOf nativo
                    {
                        if let Ok(button) = q_buttons.get(trigger.entity) {
                            if let Ok(mut container) = q_containers.get_mut(button.container_entity) {
                                if container.active_tab == button.tab_index { return; }
                                container.active_tab = button.tab_index;

                                for (mut node, content) in &mut q_contents {
                                    if content.container_entity == button.container_entity {
                                        node.display = if content.tab_index == button.tab_index { Display::Flex } else { Display::None };
                                    }
                                }

                                for (mut element, mut border, mut bg, btn) in &mut q_btn_styles {
                                    if btn.container_entity == button.container_entity {
                                        let active = btn.tab_index == button.tab_index;
                                        *element = if active { crate::theme::RuiThemeElement::TabActive } else { crate::theme::RuiThemeElement::Tab };
                                        *border = BorderColor::from(if active { Color::srgb(0.4, 0.6, 1.0) } else { Color::NONE });
                                        *bg = BackgroundColor(if active { Color::srgb(0.2, 0.2, 0.22) } else { Color::srgb(0.12, 0.12, 0.14) });
                                    }
                                }

                                for (mut text_color, parent) in &mut q_texts {
                                    if let Ok(btn) = q_buttons.get(parent.get()) {
                                        if btn.container_entity == button.container_entity {
                                            text_color.0 = if btn.tab_index == button.tab_index { Color::WHITE } else { Color::srgb(0.6, 0.6, 0.6) };
                                        }
                                    }
                                }
                            }
                        }
                    }
                );
            }
        });

        // 2. Tab Content Area (El cuerpo donde va el contenido de los sub-paneles)
        p.spawn((
            Node {
                display: Display::Flex, flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0), height: Val::Percent(100.0),
                min_width: Val::Px(0.0), min_height: Val::Px(0.0),
                flex_grow: 1.0, flex_basis: Val::Px(0.0), flex_shrink: 1.0,
                overflow: Overflow::clip(), ..default()
            },
        )).with_children(|content_parent| {
            for (i, (_, _, modifier, content_builder)) in builder.tabs.into_iter().enumerate() {
                let is_active = i == active_tab;

                let mut node = Node {
                    display: if is_active { Display::Flex } else { Display::None },
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0), height: Val::Percent(100.0),
                    min_width: Val::Px(0.0), min_height: Val::Px(0.0),
                    flex_grow: 1.0, flex_basis: Val::Px(0.0), flex_shrink: 1.0,
                    ..default()
                };
                let mut bg = ImageNode::default();
                modifier(&mut node, &mut bg);

                content_parent.spawn((
                    node,
                    bg,
                    BackgroundColor(Color::srgb(0.12, 0.12, 0.14)),
                    RuiTabContent { container_entity, tab_index: i },
                )).with_children(|c| {
                    content_builder(c);
                });
            }
        });
    });

    cmds
}
