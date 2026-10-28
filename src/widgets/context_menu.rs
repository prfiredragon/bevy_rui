//! Right-click floating context menu widget.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui::RelativeCursorPosition;
use bevy::ui_widgets::Activate;
use bevy::window::PrimaryWindow;
use crate::theme::RuiThemeElement;
use crate::widgets::{menu::RuiIcon, RuiBuilderExt, RuiButtonStateColors};

/// Component attached to a target entity that has an associated right-click context menu popup.
#[derive(Component)]
pub struct RuiContextMenu {
    /// Entity ID of the floating popup container.
    pub popup_entity: Entity,
    /// Whether the context menu popup is currently open.
    pub is_open: bool,
}

/// Component attached to each clickable item inside a context menu popup.
#[derive(Component)]
pub struct RuiContextMenuItem {
    /// Entity ID of the enclosing context menu popup container.
    pub popup_entity: Entity,
}

/// Attaches a right-click context menu popup to the current parent entity.
///
/// Populate menu items inside `build_items` using [`RuiBuilderExt::context_menu_item`].
///
/// Also available via [`crate::widgets::RuiBuilderExt::context_menu`].
pub fn spawn_context_menu<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    build_items: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let target_id = parent.target_entity();

    let popup_id = parent
        .commands()
        .spawn((
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(160.0),
                width: Val::Auto,
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor::from(Color::srgb(0.3, 0.3, 0.3)),
            ZIndex(1000),
            GlobalZIndex(1000),
            Hovered::default(),
            Pickable::default(),
            RuiThemeElement::ContextMenuBg,
        ))
        .with_children(|popup| {
            let mut popup_spawner = popup.reborrow();
            popup_spawner.scrollview(
                |n| {
                    n.width = Val::Percent(100.0);
                    n.align_items = AlignItems::Stretch;
                },
                |scroll| {
                    let mut scroll_spawner = scroll.reborrow();
                    build_items(&mut scroll_spawner);
                },
            );
        })
        .id();

    parent.commands().entity(target_id).insert((
        RuiContextMenu {
            popup_entity: popup_id,
            is_open: false,
        },
        Hovered::default(),
        Pickable::default(),
        RelativeCursorPosition::default(),
    ));
}

/// Spawns a clickable option item inside a context menu with `label` and optional [`RuiIcon`].
///
/// Automatically closes the context menu when activated. Attach `.observe(|_: On<Activate>| ...)`
/// on the returned [`EntityCommands`] to execute custom logic on click.
///
/// Also available via [`crate::widgets::RuiBuilderExt::context_menu_item`].
pub fn spawn_context_menu_item<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    label: &str,
    icon: Option<RuiIcon>,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut s = Node {
        width: Val::Percent(100.0),
        height: Val::Px(28.0),
        padding: UiRect::horizontal(Val::Px(10.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        ..default()
    };
    modifier(&mut s);

    let popup_id = parent.target_entity();
    let colors = RuiButtonStateColors {
        normal: Color::NONE,
        hovered: Color::srgb(0.25, 0.25, 0.35),
        pressed: Color::srgb(0.15, 0.15, 0.2),
    };

    let mut cmds = parent.spawn((
        bevy::ui_widgets::Button,
        s,
        ImageNode::default(),
        BackgroundColor(Color::NONE),
        colors,
        crate::focus::Focusable,
        Hovered::default(),
        Pickable::default(),
        RuiContextMenuItem {
            popup_entity: popup_id,
        },
        RuiThemeElement::ListItem,
    ));

    cmds.with_children(|inner| {
        if let Some(icon_type) = icon {
            inner
                .spawn((
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(8.0)),
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|icon_box| match icon_type {
                    RuiIcon::Emoji(e, font_opt) => {
                        let mut text_font = TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        };
                        if let Some(f) = font_opt {
                            text_font.font = FontSource::Handle(f);
                        }
                        icon_box.spawn((
                            Text::new(e),
                            text_font,
                            TextColor(Color::WHITE),
                            Pickable::IGNORE,
                        ));
                    }
                    RuiIcon::Texture(h) => {
                        icon_box.spawn((
                            ImageNode::new(h),
                            Node {
                                width: Val::Px(16.0),
                                height: Val::Px(16.0),
                                ..default()
                            },
                            Pickable::IGNORE,
                        ));
                    }
                });
        }

        inner.spawn((
            Text::new(label),
            TextFont::default(),
            TextColor(Color::WHITE),
            RuiThemeElement::Text,
            Pickable::IGNORE,
        ));
    });

    cmds.observe(
        move |_trigger: On<Activate>,
              mut q_menus: Query<&mut RuiContextMenu>,
              mut q_popups: Query<&mut Node>,
              mut active_scope: ResMut<crate::focus::RuiActiveScope>| {
            for mut ctx in &mut q_menus {
                if ctx.is_open {
                    ctx.is_open = false;
                    active_scope.remove_window(ctx.popup_entity);
                    if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                        node.display = Display::None;
                    }
                }
            }
        },
    );

    cmds
}

/// System that opens a [`RuiContextMenu`] at the cursor position on right-click and closes any other open context menus.
pub fn handle_context_menu_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_menus: Query<(Entity, Option<&Hovered>, &mut RuiContextMenu)>,
    mut q_popups: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    let Ok(window) = q_window.single() else {
        return;
    };
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Find the first target entity that is actually hovered (`hovered.get() == true`)
    let mut target_to_open: Option<Entity> = None;
    for (entity, hovered, _) in &q_menus {
        if hovered.is_some_and(|h| h.get()) {
            target_to_open = Some(entity);
            break;
        }
    }

    for (entity, _, mut ctx) in &mut q_menus {
        if Some(entity) == target_to_open {
            ctx.is_open = true;
            active_scope.push_window(ctx.popup_entity);

            if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                node.display = Display::Flex;
                node.left = Val::Px(cursor_pos.x);
                node.top = Val::Px(cursor_pos.y);

                let window_height = window.height();
                let margin_bottom = 10.0;
                if cursor_pos.y < window_height {
                    node.max_height = Val::Px(window_height - cursor_pos.y - margin_bottom);
                }
            }
        } else if ctx.is_open {
            ctx.is_open = false;
            active_scope.remove_window(ctx.popup_entity);
            if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                node.display = Display::None;
            }
        }
    }
}

/// System that closes open [`RuiContextMenu`] popups when clicking outside of them.
pub fn close_context_menus_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_menus: Query<(Option<&Hovered>, &mut RuiContextMenu)>,
    mut q_popups: Query<(Option<&Hovered>, &mut Node)>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (_target_hovered, mut ctx) in &mut q_menus {
            if !ctx.is_open {
                continue;
            }
            let popup_is_hovered = q_popups
                .get_mut(ctx.popup_entity)
                .ok()
                .and_then(|(h, _)| h)
                .is_some_and(|h| h.get());

            if !popup_is_hovered {
                ctx.is_open = false;
                active_scope.remove_window(ctx.popup_entity);
                if let Ok((_, mut node)) = q_popups.get_mut(ctx.popup_entity) {
                    node.display = Display::None;
                }
            }
        }
    } else if mouse.just_pressed(MouseButton::Right) {
        let any_target_hovered = q_menus
            .iter()
            .any(|(hovered, _)| hovered.is_some_and(|h| h.get()));

        if !any_target_hovered {
            for (_, mut ctx) in &mut q_menus {
                if ctx.is_open {
                    ctx.is_open = false;
                    active_scope.remove_window(ctx.popup_entity);
                    if let Ok((_, mut node)) = q_popups.get_mut(ctx.popup_entity) {
                        node.display = Display::None;
                    }
                }
            }
        }
    }
}
