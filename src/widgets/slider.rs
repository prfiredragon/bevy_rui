//! Interactive horizontal numeric slider widget with mouse drag and keyboard/gamepad support.

use bevy::prelude::*;
use bevy::ecs::relationship::Relationship;
use crate::widgets::RuiButtonStateColors;

/// Component storing the numeric range, current `value`, and child entities of a slider widget.
#[derive(Component)]
pub struct RuiSlider {
    /// Minimum slider value.
    pub min: f32,
    /// Maximum slider value.
    pub max: f32,
    /// Current clamped value in `[min, max]`.
    pub value: f32,
    /// Entity ID of the draggable thumb handle.
    pub handle_entity: Entity,
    /// Entity ID of the progress fill bar.
    pub fill_entity: Entity,
    pub dragging: bool,
    pub last_cursor_pos: Option<Vec2>,
}

/// Marker component for the draggable thumb handle of a [`RuiSlider`].
#[derive(Component)] pub struct RuiSliderHandle;
/// Marker component for the filled bar portion of a [`RuiSlider`].
#[derive(Component)] pub struct RuiSliderFill;

/// Spawns an interactive horizontal slider with range `[min, max]` and initial `value`.
///
/// Updates [`RuiSlider::value`] automatically on `PointerDrag` and when focused via keyboard/gamepad left/right keys.
///
/// Also available via [`crate::widgets::RuiBuilderExt::slider`].
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
        border_radius: BorderRadius::all(Val::Px(6.0)),
        position_type: PositionType::Relative,
        ..default()
    };
    modifier(&mut s);

    let mut fill_id = Entity::PLACEHOLDER;
    let mut handle_id = Entity::PLACEHOLDER;

    let mut cmds = parent_cmd.spawn((
        s,
        Pickable::default(),
        crate::focus::Focusable,
        BorderColor::from(Color::NONE),
    ));
    

    cmds.with_children(|parent| {
        // 1. Track background
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0), height: Val::Px(10.0), left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(5.0)), ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgb(0.08, 0.10, 0.15)),
            Pickable { should_block_lower: false, is_hoverable: false },
            crate::theme::RuiThemeElement::SliderTrack,
        ));
        
        // 2. Fill line
        fill_id = parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(0.0), height: Val::Px(10.0), left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(5.0)), ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgb(1.0, 0.82, 0.18)),
            Pickable { should_block_lower: false, is_hoverable: false },
            RuiSliderFill,
            crate::theme::RuiThemeElement::ProgressBarFill,
        )).id();

        // 3. Handle
        handle_id = parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(20.0), height: Val::Px(20.0), left: Val::Px(0.0),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)), ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::WHITE),
            BorderColor::from(Color::srgb(0.15, 0.18, 0.25)),
            RuiSliderHandle,
            Pickable { should_block_lower: false, is_hoverable: true },
            RuiButtonStateColors {
                normal: Color::WHITE, hovered: Color::srgb(1.0, 0.95, 0.7), pressed: Color::srgb(0.95, 0.8, 0.2),
            },
            crate::theme::RuiThemeElement::SliderHandle,
        )).id();
    });

    cmds.insert(RuiSlider {
        min, max, value: value.clamp(min, max), handle_entity: handle_id, fill_entity: fill_id,
        dragging: false, last_cursor_pos: None,
    });

    // Observador reactivo de clic directo en la barra del slider
    cmds.observe(
        move |on_press: On<PointerPress>,
              mut q_slider: Query<(&mut RuiSlider, &ComputedNode, &bevy::ui::UiGlobalTransform)>,
              q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
              parents: Query<&ChildOf>,
              mut input_focus: ResMut<bevy::input_focus::InputFocus>|
        {
            let mut target_entity = on_press.event_target();
            loop {
                if q_slider.contains(target_entity) { break; }
                if let Ok(p) = parents.get(target_entity) { target_entity = p.get(); } else { return; }
            }

            input_focus.set(target_entity, bevy::input_focus::FocusCause::Navigated);

            if let Ok((mut slider, computed, transform)) = q_slider.get_mut(target_entity) {
                let size_x = computed.size().x;
                if size_x > 0.0 {
                    if let Ok(window) = q_window.single() {
                        if let Some(cursor_pos) = window.physical_cursor_position() {
                            let center_x = transform.to_scale_angle_translation().2.x;
                            let local_x = (cursor_pos.x - (center_x - size_x / 2.0)).clamp(0.0, size_x);
                            let pct = local_x / size_x;
                            let range = slider.max - slider.min;
                            slider.value = (slider.min + pct * range).clamp(slider.min, slider.max);
                        }
                    }
                }
            }
        }
    );

    // Observador reactivo de arrastre con soporte de UiGlobalTransform en píxeles físicos
    cmds.observe(
        move |on_drag: On<PointerDrag>,
              mut q_slider: Query<(&mut RuiSlider, &ComputedNode, &bevy::ui::UiGlobalTransform)>,
              q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
              parents: Query<&ChildOf>,
              mut input_focus: ResMut<bevy::input_focus::InputFocus>| 
        {
            let mut target_entity = on_drag.event_target();
            loop {
                if q_slider.contains(target_entity) { break; }
                if let Ok(p) = parents.get(target_entity) { target_entity = p.get(); } else { return; }
            }

            input_focus.set(target_entity, bevy::input_focus::FocusCause::Navigated);

            if let Ok((mut slider, computed, transform)) = q_slider.get_mut(target_entity) {
                let size_x = computed.size().x;
                if size_x > 0.0 {
                    if let Ok(window) = q_window.single() {
                        if let Some(cursor_pos) = window.physical_cursor_position() {
                            let center_x = transform.to_scale_angle_translation().2.x;
                            let local_x = (cursor_pos.x - (center_x - size_x / 2.0)).clamp(0.0, size_x);
                            let pct = local_x / size_x;
                            let range = slider.max - slider.min;
                            slider.value = (slider.min + pct * range).clamp(slider.min, slider.max);
                        }
                    }
                }
            }
        }
    );

    cmds
}
/// System that updates the position of [`RuiSliderHandle`] and width of [`RuiSliderFill`] to match [`RuiSlider::value`].
pub fn update_slider_visuals(
    q_slider: Query<(&RuiSlider, &ComputedNode)>,
    mut q_handles: Query<&mut Node, (With<RuiSliderHandle>, Without<RuiSliderFill>)>,
    mut q_fills: Query<&mut Node, (With<RuiSliderFill>, Without<RuiSliderHandle>)>,
) {
    for (slider, computed) in &q_slider {
        let size_x = computed.size().x * computed.inverse_scale_factor();
        if size_x <= 0.0 { continue; }

        let pct = ((slider.value - slider.min) / (slider.max - slider.min).max(0.0001)).clamp(0.0, 1.0);

        if let Ok(mut handle_node) = q_handles.get_mut(slider.handle_entity) {
            let pixel_x = pct * size_x;
            handle_node.left = Val::Px(pixel_x - 10.0);
        }

        if let Ok(mut fill_node) = q_fills.get_mut(slider.fill_entity) {
            fill_node.width = Val::Percent(pct * 100.0);
        }
    }
}
/// System that adjusts [`RuiSlider::value`] when the slider has [`bevy::input_focus::InputFocus`]
/// and left/right keyboard keys, gamepad D-Pad buttons, or the gamepad left stick are used.
pub fn handle_slider_navigation(
    mut focus: ResMut<bevy::input_focus::InputFocus>,
    bindings: Res<crate::navigation::RuiNavigationBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    mut hold_timer: Local<f32>,
    mut q_slider: Query<(Entity, &mut RuiSlider, &mut BorderColor)>,
) {
    // Si ningún widget tiene foco aún y solo hay un slider en escena, darle foco automáticamente
    if focus.get().is_none() && q_slider.iter().count() == 1 {
        if let Some((entity, _, _)) = q_slider.iter().next() {
            focus.set(entity, bevy::input_focus::FocusCause::Navigated);
        }
    }

    let dt = time.delta_secs();

    for (entity, mut slider, mut border) in &mut q_slider {
        let is_focused = focus.get() == Some(entity);
        
        if is_focused {
            *border = BorderColor::from(Color::srgba(1.0, 0.9, 0.35, 0.85));
            
            let mut step_x = 0.0;
            if crate::navigation::check_keys(&keys, &bindings.left_keys)
                || crate::navigation::check_buttons(&bindings.left_buttons, &gamepads)
            {
                step_x -= 1.0;
                *hold_timer = 0.0;
            }
            if crate::navigation::check_keys(&keys, &bindings.right_keys)
                || crate::navigation::check_buttons(&bindings.right_buttons, &gamepads)
            {
                step_x += 1.0;
                *hold_timer = 0.0;
            }
            
            if step_x != 0.0 {
                let range = slider.max - slider.min;
                slider.value = (slider.value + step_x * range * 0.05).clamp(slider.min, slider.max);
                continue;
            }

            // Soporte continuo: mantener pulsadas teclas/D-Pad o mover el stick izquierdo del gamepad
            let held_left = bindings.left_keys.iter().any(|k| keys.pressed(*k))
                || bindings.left_buttons.iter().any(|b| gamepads.iter().any(|g| g.pressed(*b)));
            let held_right = bindings.right_keys.iter().any(|k| keys.pressed(*k))
                || bindings.right_buttons.iter().any(|b| gamepads.iter().any(|g| g.pressed(*b)));

            let mut stick_x = 0.0f32;
            for g in &gamepads {
                let lx = g.left_stick().x;
                if lx.abs() > 0.2 {
                    stick_x += lx;
                }
            }
            stick_x = stick_x.clamp(-1.0, 1.0);

            if stick_x.abs() > 0.2 {
                let range = slider.max - slider.min;
                slider.value = (slider.value + stick_x * range * 0.6 * dt).clamp(slider.min, slider.max);
            } else if held_left || held_right {
                *hold_timer += dt;
                if *hold_timer > 0.25 {
                    let dir = (if held_right { 1.0 } else { 0.0 }) - (if held_left { 1.0 } else { 0.0 });
                    let range = slider.max - slider.min;
                    slider.value = (slider.value + dir * range * 0.5 * dt).clamp(slider.min, slider.max);
                }
            } else {
                *hold_timer = 0.0;
            }
        } else {
            *border = BorderColor::from(Color::NONE);
        }
    }
}
