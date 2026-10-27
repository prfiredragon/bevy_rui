use bevy::prelude::*;
// CORRECCIÓN DEFINITIVA DE RUTA: Traemos el trait Relationship nativo desde el preludio de Bevy 0.20
use bevy::ecs::relationship::Relationship;
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiSlider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub handle_entity: Entity,
    pub fill_entity: Entity,
    pub dragging: bool,
    pub last_cursor_pos: Option<Vec2>,
}

#[derive(Component)] pub struct RuiSliderHandle;
#[derive(Component)] pub struct RuiSliderFill;

pub fn spawn_slider<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    min: f32,
    max: f32,
    value: f32,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        width: Val::Px(200.0),
        height: Val::Px(24.0),
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        position_type: PositionType::Relative,
        ..default()
    };
    modifier(&mut s);

    let mut fill_id = Entity::PLACEHOLDER;
    let mut handle_id = Entity::PLACEHOLDER;

    let mut cmds = parent_cmd.spawn((
        s,
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
        crate::focus::Focusable,
        BorderColor::from(Color::srgb(0.1, 0.1, 0.1)),
    ));
    

    cmds.with_children(|parent| {
        // 1. Track background
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0), height: Val::Px(6.0), left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(3.0)), ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            Pickable { should_block_lower: false, is_hoverable: false },
            crate::theme::RuiThemeElement::SliderTrack,
        ));
        
        // 2. Fill line
        fill_id = parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(0.0), height: Val::Px(6.0), left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(3.0)), ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.6, 1.0)),
            Pickable { should_block_lower: false, is_hoverable: false },
            RuiSliderFill,
            crate::theme::RuiThemeElement::ProgressBarFill,
        )).id();

        // 3. Handle
        handle_id = parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(16.0), height: Val::Px(16.0), left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(8.0)), ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            RuiSliderHandle,
            Pickable { should_block_lower: true, is_hoverable: true },
            RuiButtonStateColors {
                normal: Color::srgb(0.9, 0.9, 0.9), hovered: Color::srgb(1.0, 1.0, 1.0), pressed: Color::srgb(0.7, 0.7, 0.7),
            },
            crate::theme::RuiThemeElement::SliderHandle,
        )).id();
    });

    cmds.insert(RuiSlider {
        min, max, value: value.clamp(min, max), handle_entity: handle_id, fill_entity: fill_id,
        dragging: false, last_cursor_pos: None,
    });

    // Observador reactivo de arrastre con escalado lúdico y burbujeo de eventos
    cmds.observe(
        move |on_drag: On<PointerDrag>,
              mut q_slider: Query<(&mut RuiSlider, &ComputedNode, &GlobalTransform)>,
              q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
              parents: Query<&ChildOf>| 
        {
            let mut target_entity = on_drag.event_target();
            loop {
                if q_slider.contains(target_entity) { break; }
                if let Ok(p) = parents.get(target_entity) { target_entity = p.get(); } else { return; }
            }

            if let Ok((mut slider, computed, transform)) = q_slider.get_mut(target_entity) {
                let size_x = computed.size().x;
                if size_x > 0.0 {
                    if let Ok(window) = q_window.single() {
                        if let Some(cursor_pos) = window.cursor_position() {
                            let global_pos = transform.translation();
                            let local_x = (cursor_pos.x - (global_pos.x - size_x / 2.0)).clamp(0.0, size_x);
                            let pct = local_x / size_x;
                            let range = slider.max - slider.min;
                            slider.value = slider.min + pct * range;
                        }
                    }
                }
            }
        }
    );

    cmds
}
pub fn update_slider_visuals(
    q_slider: Query<(&RuiSlider, &ComputedNode)>,
    mut q_handles: Query<&mut Node, (With<RuiSliderHandle>, Without<RuiSliderFill>)>,
    mut q_fills: Query<&mut Node, (With<RuiSliderFill>, Without<RuiSliderHandle>)>,
) {
    for (slider, computed) in &q_slider {
        let size_x = computed.size().x;
        if size_x <= 0.0 { continue; }

        let pct = (slider.value - slider.min) / (slider.max - slider.min).max(0.0001);

        if let Ok(mut handle_node) = q_handles.get_mut(slider.handle_entity) {
            let pixel_x = pct * size_x;
            handle_node.left = Val::Px(pixel_x - 8.0);
        }

        if let Ok(mut fill_node) = q_fills.get_mut(slider.fill_entity) {
            fill_node.width = Val::Percent(pct * 100.0);
        }
    }
}
pub fn handle_slider_navigation(
    focus: Res<bevy::input_focus::InputFocus>,
    bindings: Res<crate::navigation::RuiNavigationBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut q_slider: Query<(Entity, &mut RuiSlider, &mut BorderColor)>,
) {
    for (entity, mut slider, mut border) in &mut q_slider {
        let is_focused = focus.get() == Some(entity);
        
        if is_focused {
            *border = BorderColor::from(Color::srgb(0.4, 0.6, 1.0));
            
            let mut move_x = 0.0;
            if crate::navigation::check_keys(&keys, &bindings.left_keys) || crate::navigation::check_buttons(&bindings.left_buttons, &gamepads) {
                move_x -= 1.0;
            }
            if crate::navigation::check_keys(&keys, &bindings.right_keys) || crate::navigation::check_buttons(&bindings.right_buttons, &gamepads) {
                move_x += 1.0;
            }
            
            if move_x != 0.0 {
                let range = slider.max - slider.min;
                slider.value = (slider.value + move_x * range * 0.05).clamp(slider.min, slider.max);
            }
        } else {
            *border = BorderColor::from(Color::srgb(0.1, 0.1, 0.1));
        }
    }
}
