use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy::ui_widgets::Activate;
use bevy::text::FontSize;
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiWindow;

#[derive(Component)]
pub struct RuiWindowHeader {
    pub window_entity: Entity,
}

#[derive(Component)]
pub struct RuiWindowCloseButton {
    pub window_entity: Entity,
}

pub fn spawn_window<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    title: &str,
    closable: bool,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands, Entity),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        position_type: PositionType::Absolute,
        left: Val::Px(100.0),
        top: Val::Px(100.0),
        width: Val::Px(300.0),
        // Espesor físico visible para el contorno de la ventana
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect::all(Val::Px(2.0)), 
        border_radius: BorderRadius::all(Val::Px(5.0)),
        overflow: Overflow::clip(),
        ..default()
    };
    modifier(&mut s);

    // CORRECCIÓN DE BORDES: Removemos ImageNode para permitir que el contorno y el fondo convivan de forma nativa
    let mut cmds = parent_cmd.spawn((
        s,
        BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
        BorderColor::from(Color::BLACK), // Contorno negro sólido garantizado
        crate::theme::RuiThemeElement::Window,
        ZIndex(100),
        Pickable::default(),
        UiTransform::default(), 
        RuiWindow,
    ));

    let window_id = cmds.id();
    cmds.with_children(|parent| {
        let header_id = parent.spawn((
            Node { display: Display::Flex, align_self: AlignSelf::Stretch, height: Val::Px(35.0), padding: UiRect::all(Val::Px(6.0)), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
            ImageNode::default(),
            crate::theme::RuiThemeElement::WindowHeader,
            Pickable::default(), 
            RelativeCursorPosition::default(),
            RuiWindowHeader { window_entity: window_id }
        )).with_children(|header| {
            header.spawn((Text::new(title), TextFont::default(), TextColor(Color::WHITE), crate::theme::RuiThemeElement::WindowHeaderText));
            if closable {
                let close_btn_id = header.spawn((
                    Node { display: Display::Flex, width: Val::Px(24.0), height: Val::Px(24.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    bevy::ui_widgets::Button, 
                    ImageNode::default(),
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    RuiButtonStateColors { normal: Color::srgb(0.8, 0.2, 0.2), hovered: Color::srgb(0.9, 0.3, 0.3), pressed: Color::srgb(0.6, 0.1, 0.1) },
                    RuiWindowCloseButton { window_entity: window_id },
                )).with_children(|btn| {
                    btn.spawn((Text::new("X"), TextFont { font_size: FontSize::Px(16.0), ..default() }, TextColor(Color::WHITE)));
                }).id();

                header.commands().entity(close_btn_id).observe(
                    move |trigger: On<Activate>, 
                          mut commands: Commands,
                          q_close: Query<&RuiWindowCloseButton>,
                          mut active_scope: ResMut<crate::focus::RuiActiveScope>| 
                    {
                        if let Ok(btn) = q_close.get(trigger.entity) {
                            active_scope.remove_window(btn.window_entity);
                            if let Ok(mut entity_cmds) = commands.get_entity(btn.window_entity) { 
                                entity_cmds.try_despawn(); 
                            }
                        }
                    }
                );
            }
        }).id();

        // Sistema de arrastre fluido por deltas relativos sobre UiTransform
        parent.commands().entity(header_id).observe(
            move |on_drag: On<PointerDrag>, 
                  mut q_window: Query<&mut UiTransform, With<RuiWindow>>,
                  q_header: Query<&RuiWindowHeader>| 
            {
                if let Ok(header) = q_header.get(on_drag.event_target()) {
                    if let Ok(mut transform) = q_window.get_mut(header.window_entity) {
                        let current_x = if let Val::Px(px_val) = transform.translation.x { px_val } else { 0.0 };
                        let current_y = if let Val::Px(px_val) = transform.translation.y { px_val } else { 0.0 };

                        transform.translation.x = Val::Px(current_x + on_drag.delta.x);
                        transform.translation.y = Val::Px(current_y + on_drag.delta.y);
                    }
                }
            }
        );

        parent.spawn((
            Node { display: Display::Flex, flex_direction: FlexDirection::Column, flex_grow: 1.0, overflow: Overflow::clip(), width: Val::Percent(100.0), padding: UiRect::all(Val::Px(10.0)), ..default() },
            Pickable::default(),
        )).with_children(|p| children(p, window_id));
    });
    cmds
}
pub fn handle_new_windows(
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
    query: Query<Entity, Added<RuiWindow>>,
) {
    for entity in &query {
        active_scope.push_window(entity);
    }
}

pub fn handle_window_focus(
    mouse: Res<ButtonInput<MouseButton>>,
    q_interactions: Query<(Entity, &bevy::ui::Pressed)>,
    parents: Query<&ChildOf>,
    mut q_windows: Query<&mut ZIndex, With<RuiWindow>>,
    mut counter: Local<i32>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut clicked = None;
        for (entity, _) in &q_interactions {
            let mut cur = entity;
            loop {
                if q_windows.contains(cur) { clicked = Some(cur); break; }
                if let Ok(p) = parents.get(cur) { cur = p.get(); } else { break; }
            }
        }
        if let Some(win) = clicked {
            *counter += 1;
            if let Ok(mut z) = q_windows.get_mut(win) { 
                *z = ZIndex(*counter); 
            }
            active_scope.set_active_window(win);
        }
    }
}
