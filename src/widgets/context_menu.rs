use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::widgets::{RuiButtonStateColors, menu::RuiIcon};
use crate::theme::RuiThemeElement;

#[derive(Component)]
pub struct RuiContextMenu {
    pub popup_entity: Entity,
    pub is_open: bool,
}

#[derive(Component)]
pub struct RuiContextMenuItem {
    pub popup_entity: Entity,
}

pub fn spawn_context_menu<'a>(
    parent: &'a mut ChildSpawnerCommands,
    build_items: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let target_id = parent.target_entity();

    let popup_id = parent.commands().spawn((
        Node { 
            display: Display::None, 
            position_type: PositionType::Absolute, 
            flex_direction: FlexDirection::Column, 
            width: Val::Px(150.0), 
            border: UiRect::all(Val::Px(1.0)), 
            ..default() 
        },
        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.15, 0.15, 0.15)) },
        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), 
        ZIndex(1000), 
        GlobalZIndex(1000),
        RuiThemeElement::ContextMenuBg,
        bevy::ui::FocusPolicy::Block,
    )).with_children(build_items).id();

    // Attach to the parent
    parent.commands().entity(target_id)
        .insert((
            Interaction::None, 
            RuiContextMenu { popup_entity: popup_id, is_open: false },
            RelativeCursorPosition::default(),
        ));
}

pub fn spawn_context_menu_item<'a>(
    parent: &'a mut ChildSpawnerCommands,
    label: &str,
    icon: Option<RuiIcon>,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        width: Val::Percent(100.0),
        height: Val::Px(30.0),
        padding: UiRect::horizontal(Val::Px(8.0)),
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
        Button,
        s,
        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::NONE) },
        colors,
        crate::focus::Focusable,
        RuiContextMenuItem { popup_entity: popup_id },
        RuiThemeElement::ListItem,
    ));

    cmds.with_children(|inner| {
        if let Some(icon_type) = icon {
            inner.spawn(Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(8.0)),
                ..default()
            }).with_children(|icon_box| {
                match icon_type {
                    RuiIcon::Emoji(e, font_opt) => {
                        let mut text_font = TextFont { font_size: bevy::prelude::FontSize::Px(14.0), ..default() };
                        if let Some(f) = font_opt {
                            text_font.font = bevy::prelude::FontSource::Handle(f);
                        }
                        icon_box.spawn((Text::new(e), text_font, TextColor(Color::WHITE)));
                    },
                    RuiIcon::Texture(h) => {
                        icon_box.spawn((ImageNode::new(h), Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
                    }
                }
            });
        }

        inner.spawn((
            Text::new(label),
            TextFont::default(),
            TextColor(Color::WHITE),
            RuiThemeElement::Text,
        ));
    });

    cmds
}

pub fn handle_context_menu_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q_menus: Query<(&Interaction, &mut RuiContextMenu)>,
    mut q_popups: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    let Some(window) = q_window.iter().next() else { return; };

    if mouse.just_pressed(MouseButton::Right) {
        if let Some(cursor_pos) = window.cursor_position() {
            let mut opened_any = false;
            for (interaction, mut ctx) in &mut q_menus {
                if *interaction == Interaction::Hovered {
                    ctx.is_open = true;
                    opened_any = true;
                    active_scope.push_window(ctx.popup_entity);
                    if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                        node.display = Display::Flex;
                        
                        let scale_factor = window.scale_factor();
                        let logical_pos_x = cursor_pos.x / scale_factor;
                        let logical_pos_y = cursor_pos.y / scale_factor;

                        node.left = Val::Px(logical_pos_x);
                        node.top = Val::Px(logical_pos_y);
                    }
                }
            }

            if opened_any {
                for (interaction, mut ctx) in &mut q_menus {
                    if *interaction != Interaction::Hovered && ctx.is_open {
                        ctx.is_open = false;
                        active_scope.remove_window(ctx.popup_entity);
                        if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                            node.display = Display::None;
                        }
                    }
                }
            }
        }
    }
}

pub fn close_context_menus_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_menus: Query<(&Interaction, &mut RuiContextMenu)>,
    mut q_popups: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
        let mut hovered_any = false;
        if mouse.just_pressed(MouseButton::Right) {
            for (interaction, _) in &q_menus {
                if *interaction == Interaction::Hovered {
                    hovered_any = true;
                }
            }
        }

        if !hovered_any {
            for (interaction, mut ctx) in &mut q_menus {
                if ctx.is_open && *interaction == Interaction::None {
                    ctx.is_open = false;
                    active_scope.remove_window(ctx.popup_entity);
                    if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) { 
                        node.display = Display::None; 
                    }
                }
            }
        }
    }
}

pub fn handle_context_menu_item_clicks(
    q_interactions: Query<(&Interaction, &RuiContextMenuItem), Changed<Interaction>>,
    mut q_menus: Query<&mut RuiContextMenu>,
    mut q_popups: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    for (interaction, item) in &q_interactions {
        if *interaction == Interaction::Pressed {
            for mut ctx in &mut q_menus {
                if ctx.popup_entity == item.popup_entity {
                    ctx.is_open = false;
                    active_scope.remove_window(ctx.popup_entity);
                    if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                        node.display = Display::None;
                    }
                }
            }
        }
    }
}
