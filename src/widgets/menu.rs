use bevy::prelude::*;
use crate::focus::Focusable;
use crate::widgets::RuiButtonStateColors;

#[derive(Component, Default)]
pub struct RuiMenuBar;

#[derive(Component)]
pub struct RuiMenuItem {
    pub action_id: String,
}

#[derive(Component)]
pub struct RuiSubmenuTrigger {
    pub is_open: bool,
    pub depth: u32,
    pub popup_entity: Entity,
}

pub enum RuiIcon {
    Emoji(String, Option<Handle<Font>>),
    Texture(Handle<Image>),
}

/// Sistema para manejar la apertura de submenús y el cierre de otros.
pub fn handle_menu_interactions(
    mouse: Res<ButtonInput<MouseButton>>,
    mut triggers: Query<(&Interaction, &mut RuiSubmenuTrigger)>,
    mut popups: Query<&mut Node>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut clicked_outside = true;
        let mut pressed_depth = None;

        for (interaction, trigger) in &triggers {
            if *interaction == Interaction::Pressed {
                clicked_outside = false;
                pressed_depth = Some(trigger.depth);
            }
        }
        
        for (interaction, mut trigger) in &mut triggers {
            if *interaction == Interaction::Pressed {
                trigger.is_open = !trigger.is_open;
                if let Ok(mut node) = popups.get_mut(trigger.popup_entity) {
                    node.display = if trigger.is_open { Display::Flex } else { Display::None };
                }
            } else {
                // Cierra si se hizo clic fuera, o si se hizo clic en un submenú del mismo nivel (cierra hermanos)
                if clicked_outside || (pressed_depth.is_some() && trigger.depth == pressed_depth.unwrap()) {
                    if trigger.is_open {
                        trigger.is_open = false;
                        if let Ok(mut node) = popups.get_mut(trigger.popup_entity) {
                            node.display = Display::None;
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_menu_bar<'a>(
    parent: &'a mut ChildSpawnerCommands,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Px(30.0),
        align_items: AlignItems::Center,
        padding: UiRect::horizontal(Val::Px(4.0)),
        ..default()
    };
    modifier(&mut s);

    let mut cmds = parent.spawn((
        s,
        ImageNode::solid_color(Color::srgb(0.12, 0.12, 0.12)),
        RuiMenuBar::default(),
    ));

    cmds.with_children(children);
    cmds
}

pub fn spawn_submenu<'a>(
    parent: &'a mut ChildSpawnerCommands,
    label: &str,
    icon: Option<RuiIcon>,
    depth: u32,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        width: Val::Auto,
        height: Val::Px(30.0),
        padding: UiRect::horizontal(Val::Px(8.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    };
    modifier(&mut s);

    let mut popup_id = Entity::PLACEHOLDER;
    let colors = RuiButtonStateColors {
        normal: Color::NONE,
        hovered: Color::srgb(0.25, 0.25, 0.35),
        pressed: Color::srgb(0.15, 0.15, 0.2),
    };

    let mut cmds = parent.spawn((
        Button,
        s,
        ImageNode::solid_color(Color::NONE),
        colors,
        Focusable,
    ));

    cmds.with_children(|inner| {
        inner.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|row| {
            // Spawn Icono
            if let Some(icon_type) = icon {
                row.spawn(Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }).with_children(|icon_box| {
                    match icon_type {
                        RuiIcon::Emoji(e, font_opt) => {
                            let mut text_font = TextFont { font_size: 14.0, ..default() };
                            if let Some(f) = font_opt {
                                text_font.font = f;
                            }
                            icon_box.spawn((Text::new(e), text_font, TextColor(Color::WHITE)));
                        },
                        RuiIcon::Texture(h) => {
                            icon_box.spawn((ImageNode::new(h), Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
                        }
                    }
                });
                // Espacio entre icono y texto
                row.spawn(Node { width: Val::Px(8.0), ..default() });
            } else {
                // Espacio extra si no hay icono para alinear los textos
                row.spawn(Node { width: Val::Px(24.0), ..default() });
            }

            // Label
            row.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Flecha si es submenú y estamos dentro de otro menú (depth > 0)
        if depth > 0 {
            inner.spawn((
                Text::new(">"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        }

        let popup_left = if depth == 0 { Val::Px(0.0) } else { Val::Percent(100.0) };
        let popup_top = if depth == 0 { Val::Percent(100.0) } else { Val::Px(0.0) };

        popup_id = inner.spawn((
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                left: popup_left,
                top: popup_top,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                border: UiRect::all(Val::Px(1.0)),
                min_width: Val::Px(160.0),
                ..default()
            },
            ImageNode::solid_color(Color::srgb(0.15, 0.15, 0.16)),
            BorderColor::all(Color::BLACK),
            ZIndex(100 + depth as i32),
            GlobalZIndex(100 + depth as i32),
        )).with_children(children).id();
    });

    cmds.insert(RuiSubmenuTrigger { is_open: false, depth, popup_entity: popup_id });
    cmds
}

/// Helper para spawnear un ítem de menú (hoja sin submenú) con icono opcional
pub fn spawn_menu_item<'a>(
    parent: &'a mut ChildSpawnerCommands,
    label: &str,
    icon: Option<RuiIcon>,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        width: Val::Auto,
        height: Val::Px(30.0),
        padding: UiRect::horizontal(Val::Px(8.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        ..default()
    };
    modifier(&mut s);

    let colors = RuiButtonStateColors {
        normal: Color::NONE,
        hovered: Color::srgb(0.25, 0.25, 0.35),
        pressed: Color::srgb(0.15, 0.15, 0.2),
    };

    let mut cmds = parent.spawn((
        Button,
        s,
        ImageNode::solid_color(Color::NONE),
        colors,
        Focusable,
    ));

    cmds.with_children(|row| {
        // Spawn Icono
        if let Some(icon_type) = icon {
            row.spawn(Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|icon_box| {
                match icon_type {
                    RuiIcon::Emoji(e, font_opt) => {
                        let mut text_font = TextFont { font_size: 14.0, ..default() };
                        if let Some(f) = font_opt {
                            text_font.font = f;
                        }
                        icon_box.spawn((Text::new(e), text_font, TextColor(Color::WHITE)));
                    },
                    RuiIcon::Texture(h) => {
                        icon_box.spawn((ImageNode::new(h), Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
                    }
                }
            });
            // Espacio entre icono y texto
            row.spawn(Node { width: Val::Px(8.0), ..default() });
        } else {
            // Espacio extra si no hay icono para alinear el texto de este item con otros que sí tengan
            row.spawn(Node { width: Val::Px(24.0), ..default() });
        }

        // Label
        row.spawn((
            Text::new(label),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });

    cmds
}
#[derive(Bundle)]
pub struct RuiMenuBarBundle {
    pub node: Node,
    pub background: ImageNode,
    pub marker: RuiMenuBar,
}