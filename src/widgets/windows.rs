use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy::window::PrimaryWindow;
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiWindow;

#[derive(Component)]
pub struct RuiWindowHeader {
    pub window_entity: Entity,
    pub dragging: bool,
    pub drag_offset: Vec2,
}

#[derive(Component)]
pub struct RuiWindowCloseButton {
    pub window_entity: Entity,
}

pub fn spawn_window<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    title: &str,
    closable: bool,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands, Entity),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        position_type: PositionType::Absolute,
        left: Val::Px(100.0),
        top: Val::Px(100.0),
        width: Val::Px(300.0),
        // Se mantiene el border de layout, pero el color será dibujado por el ninepatch (o ImageNode)
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect::all(Val::Px(2.0)), 
        overflow: Overflow::clip(),
        ..default()
    };
    modifier(&mut s);

    let mut cmds = parent_cmd.spawn((
        s,
        ImageNode::default(),
        crate::theme::RuiThemeElement::Panel,
        ZIndex(100),
        Interaction::None,
        Pickable::default(),
        bevy::ui::FocusPolicy::Block,
        BorderColor::all(Color::BLACK),
        RuiWindow,
    ));

    cmds.with_children(|parent| {
        let window_id = parent.target_entity();
        parent.spawn((
            Node { display: Display::Flex, align_self: AlignSelf::Stretch, height: Val::Px(35.0), padding: UiRect::all(Val::Px(6.0)), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
            ImageNode::default(),
            crate::theme::RuiThemeElement::WindowHeader,
            Interaction::None, Pickable::default(), RelativeCursorPosition::default(),
            RuiWindowHeader { window_entity: window_id, dragging: false, drag_offset: Vec2::ZERO }
        )).with_children(|header| {
            header.spawn((Text::new(title), TextFont::default(), TextColor(Color::WHITE), crate::theme::RuiThemeElement::WindowHeaderText));
            if closable {
                header.spawn((
                    Node { display: Display::Flex, width: Val::Px(24.0), height: Val::Px(24.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    Button, 
                    ImageNode::solid_color(Color::srgb(0.8, 0.2, 0.2)),
                    RuiButtonStateColors { normal: Color::srgb(0.8, 0.2, 0.2), hovered: Color::srgb(0.9, 0.3, 0.3), pressed: Color::srgb(0.6, 0.1, 0.1) },
                    RuiWindowCloseButton { window_entity: window_id },
                )).with_children(|btn| {
                    btn.spawn((Text::new("X"), TextFont { font_size: 16.0, ..default() }, TextColor(Color::WHITE)));
                });
            }
        });
        parent.spawn((
            Node { display: Display::Flex, flex_direction: FlexDirection::Column, flex_grow: 1.0, overflow: Overflow::clip(), width: Val::Percent(100.0), padding: UiRect::all(Val::Px(10.0)), ..default() },
            Interaction::None, bevy::ui::FocusPolicy::Block, Pickable::default(),
        )).with_children(|p| children(p, window_id));
    });
    cmds
}

pub fn handle_window_close_clicks(
    mut commands: Commands,
    query: Query<(&Interaction, &RuiWindowCloseButton), Changed<Interaction>>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    for (interaction, btn) in &query {
        if *interaction == Interaction::Pressed {
            active_scope.remove_window(btn.window_entity);
            if let Ok(mut entity_cmds) = commands.get_entity(btn.window_entity) { entity_cmds.despawn(); }
        }
    }
}

pub fn handle_new_windows(
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
    query: Query<Entity, Added<RuiWindow>>,
) {
    for entity in &query {
        active_scope.push_window(entity);
    }
}

pub fn handle_window_drag(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_header: Query<(&RelativeCursorPosition, &Interaction, &mut RuiWindowHeader, &ComputedNode)>,
    mut q_window: Query<&mut Node, Without<RuiWindowHeader>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let (just_pressed, pressed, just_released) = (mouse.just_pressed(MouseButton::Left), mouse.pressed(MouseButton::Left), mouse.just_released(MouseButton::Left));
    for (rel_pos, interaction, mut header, comp) in &mut q_header {
        if just_released { header.dragging = false; }
        if just_pressed && *interaction == Interaction::Pressed {
            header.dragging = true;
            if let Some(pos) = rel_pos.normalized { let size = comp.size(); header.drag_offset = Vec2::new((pos.x + 0.5) * size.x, (pos.y + 0.5) * size.y); }
        }
        if header.dragging && pressed {
            if let Ok(win) = windows.single() {
                if let Some(cursor) = win.cursor_position() {
                    if let Ok(mut node) = q_window.get_mut(header.window_entity) { node.left = Val::Px(cursor.x - header.drag_offset.x); node.top = Val::Px(cursor.y - header.drag_offset.y); }
                }
            }
        }
    }
}

pub fn handle_window_focus(
    mouse: Res<ButtonInput<MouseButton>>,
    q_interactions: Query<(Entity, &Interaction)>,
    parents: Query<&ChildOf>,
    mut q_windows: Query<&mut ZIndex, With<RuiWindow>>,
    mut counter: Local<i32>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut clicked = None;
        for (entity, interaction) in &q_interactions {
            if *interaction == Interaction::Pressed {
                let mut cur = entity;
                loop {
                    if q_windows.contains(cur) { clicked = Some(cur); break; }
                    if let Ok(p) = parents.get(cur) { cur = p.get(); } else { break; }
                }
            }
        }
        if let Some(win) = clicked {
            *counter += 1;
            if let Ok(mut z) = q_windows.get_mut(win) { z.0 = 100 + *counter; }
            active_scope.set_active_window(win);
        }
    }
}