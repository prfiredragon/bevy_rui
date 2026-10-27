use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::text::FontSize;
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

pub fn spawn_menu_bar<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'w> {
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
        ImageNode::default(),
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        RuiMenuBar::default(),
    ));

    cmds.with_children(children);
    cmds
}
pub fn spawn_submenu<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    label: &str,
    icon: Option<RuiIcon>,
    depth: u32,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'w> {
    let mut s = Node {
        width: Val::Auto,
        height: Val::Px(30.0),
        padding: UiRect::horizontal(Val::Px(12.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        position_type: PositionType::Relative, // Actúa como anclaje espacial de Bevy 0.20
        ..default()
    };
    modifier(&mut s);

    let normal_color = Color::NONE;
    let hover_color = Color::srgb(0.25, 0.25, 0.35);

    let mut cmds = parent.spawn((
        bevy::ui_widgets::Button, 
        s,
        ImageNode::default(),
        BackgroundColor(normal_color),
        RuiButtonStateColors { normal: normal_color, hovered: hover_color, pressed: Color::srgb(0.15, 0.15, 0.2) },
        Focusable,
        Pickable::default(),
    ));

    let header_id = cmds.id();
    let mut popup_id = Entity::PLACEHOLDER;

    cmds.with_children(|inner| {
        // Contenedor Plano Horizontal para la etiqueta e icono internos
        inner.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|row| {
            if let Some(icon_type) = icon {
                row.spawn(Node {
                    width: Val::Px(16.0), height: Val::Px(16.0),
                    justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default()
                }).with_children(|icon_box| {
                    match icon_type {
                        RuiIcon::Emoji(e, font_opt) => {
                            let mut text_font = TextFont { font_size: FontSize::Px(14.0), ..default() };
                            if let Some(f) = font_opt { text_font.font = FontSource::Handle(f); }
                            icon_box.spawn((Text::new(e), text_font, TextColor(Color::WHITE)));
                        },
                        RuiIcon::Texture(h) => {
                            icon_box.spawn((ImageNode::new(h), Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
                        }
                    }
                });
                row.spawn(Node { width: Val::Px(8.0), ..default() });
            } else {
                row.spawn(Node { width: Val::Px(12.0), ..default() });
            }

            row.spawn((
                Text::new(label),
                TextFont { font_size: FontSize::Px(14.0), ..default() },
                TextColor(Color::WHITE),
            ));
        });

        if depth > 0 {
            inner.spawn((
                Text::new(">"),
                TextFont { font_size: FontSize::Px(12.0), ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        }

        // CORRECCIÓN DE ANCLAJE: Determinamos las coordenadas CSS-in-Rust exactas basadas en la profundidad
        let popup_left = if depth == 0 { Val::Px(0.0) } else { Val::Percent(100.0) };
        let popup_top = if depth == 0 { Val::Percent(100.0) } else { Val::Px(0.0) };

        // CORRECCIÓN DE COLAPSO: Spawneamos el popup como entidad flotante paralela con GlobalZIndex
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
            ImageNode::default(),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.16)),
            BorderColor::from(Color::BLACK),
            ZIndex(500 + depth as i32),
            GlobalZIndex(500 + depth as i32), // Forzamos elevación absoluta en la GPU
        )).with_children(children).id();
    });
    cmds.insert(RuiSubmenuTrigger { is_open: false, depth, popup_entity: popup_id });

    // Observador de Sobrevuelo (Hover In)
    cmds.observe(move |_: On<PointerOver>, mut query: Query<&mut BackgroundColor>| {
        if let Ok(mut bg) = query.get_mut(header_id) { bg.0 = hover_color; }
    });

    // Observador de Sobrevuelo (Hover Out)
    cmds.observe(move |_: On<PointerOut>, mut query: Query<&mut BackgroundColor>| {
        if let Ok(mut bg) = query.get_mut(header_id) { bg.0 = normal_color; }
    });

    // CORRECCIÓN REACTIVA DE CIERRE: Validamos basándonos en la entidad del trigger para evitar bucles de desvanecimiento automáticos
    cmds.observe(
        move |trigger: On<Activate>,
              mut q_triggers: Query<(Entity, &mut RuiSubmenuTrigger)>,
              mut q_popups: Query<&mut Node>| 
        {
            let target_entity = trigger.entity;
            let mut pressed_depth = None;
            
            if let Ok((_, mut trigger_data)) = q_triggers.get_mut(target_entity) {
                trigger_data.is_open = !trigger_data.is_open;
                pressed_depth = Some(trigger_data.depth);
                
                if let Ok(mut node) = q_popups.get_mut(trigger_data.popup_entity) {
                    node.display = if trigger_data.is_open { Display::Flex } else { Display::None };
                }
            }

            // CORRECCIÓN DEFINITIVA: Filtramos comparando la entidad de control (e != target_entity) para no auto-cerrarnos
            if let Some(depth) = pressed_depth {
                for (e, mut other_trigger) in &mut q_triggers {
                    if other_trigger.depth == depth && e != target_entity {
                        if other_trigger.is_open {
                            other_trigger.is_open = false;
                            if let Ok(mut node) = q_popups.get_mut(other_trigger.popup_entity) {
                                node.display = Display::None;
                            }
                        }
                    }
                }
            }
        }
    );

    cmds
}

pub fn close_menus_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_triggers: Query<&mut RuiSubmenuTrigger>,
    mut q_popups: Query<&mut Node>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for mut trigger in &mut q_triggers {
            if trigger.is_open {
                trigger.is_open = false;
                if let Ok(mut node) = q_popups.get_mut(trigger.popup_entity) {
                    node.display = Display::None;
                }
            }
        }
    }
}
pub fn spawn_menu_item<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    label: &str,
    icon: Option<RuiIcon>,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut s = Node {
        width: Val::Percent(100.0), // Forzamos estiramiento completo dentro del vbox flotante
        height: Val::Px(30.0),
        padding: UiRect::horizontal(Val::Px(8.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        ..default()
    };
    modifier(&mut s);

    let normal_item = Color::NONE;
    let hover_item = Color::srgb(0.25, 0.25, 0.35);

    let mut cmds = parent.spawn((
        bevy::ui_widgets::Button, 
        s,
        ImageNode::default(),
        BackgroundColor(normal_item),
        RuiButtonStateColors { normal: normal_item, hovered: hover_item, pressed: Color::srgb(0.15, 0.15, 0.2) },
        Focusable,
        Pickable::default(),
    ));

    let item_id = cmds.id();

    cmds.observe(move |_: On<PointerOver>, mut query: Query<&mut BackgroundColor>| {
        if let Ok(mut bg) = query.get_mut(item_id) { bg.0 = hover_item; }
    });

    cmds.observe(move |_: On<PointerOut>, mut query: Query<&mut BackgroundColor>| {
        if let Ok(mut bg) = query.get_mut(item_id) { bg.0 = normal_item; }
    });

    cmds.with_children(|row| {
        if let Some(icon_type) = icon {
            row.spawn(Node {
                width: Val::Px(16.0), height: Val::Px(16.0),
                justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default()
            }).with_children(|icon_box| {
                match icon_type {
                    RuiIcon::Emoji(e, font_opt) => {
                        let mut text_font = TextFont { font_size: FontSize::Px(14.0), ..default() };
                        if let Some(f) = font_opt { text_font.font = FontSource::Handle(f); }
                        icon_box.spawn((Text::new(e), text_font, TextColor(Color::WHITE)));
                    },
                    RuiIcon::Texture(h) => {
                        icon_box.spawn((ImageNode::new(h), Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
                    }
                }
            });
            row.spawn(Node { width: Val::Px(8.0), ..default() });
        } else {
            row.spawn(Node { width: Val::Px(24.0), ..default() });
        }

        row.spawn((
            Text::new(label),
            TextFont { font_size: FontSize::Px(14.0), ..default() },
            TextColor(Color::WHITE),
        ));
    });

    cmds
}
