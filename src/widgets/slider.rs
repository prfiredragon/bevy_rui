use bevy::prelude::*;

#[derive(Component)]
pub struct RuiSlider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub dragging: bool,
    pub last_cursor_pos: Option<Vec2>,
}

#[derive(Component)]
pub struct RuiSliderHandle {
    pub slider_entity: Entity,
}

#[derive(Component)]
pub struct RuiSliderFill {
    pub slider_entity: Entity,
}

pub fn spawn_slider<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    min: f32,
    max: f32,
    value: f32,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        width: Val::Px(200.0),
        height: Val::Px(24.0),
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    };
    modifier(&mut s);

    let mut cmds = parent_cmd.spawn((
        s,
        Interaction::None,
        Pickable::default(),
        bevy::ui::FocusPolicy::Block,
        RuiSlider {
            min,
            max,
            value: value.clamp(min, max),
            dragging: false,
            last_cursor_pos: None,
        },
        crate::focus::Focusable,
        BorderColor::all(Color::srgb(0.1, 0.1, 0.1)),
    ));

    cmds.with_children(|parent| {
        let slider_id = parent.target_entity();

        // Track background
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Px(6.0),
                left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.1, 0.1, 0.1)) },
            crate::theme::RuiThemeElement::SliderTrack,
            Interaction::None,
        ));
        
        // Fill line
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(0.0),
                height: Val::Px(6.0),
                left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.4, 0.6, 1.0)) },
            crate::theme::RuiThemeElement::ProgressBarFill,
            RuiSliderFill { slider_entity: slider_id },
            Interaction::None,
        ));

        // Handle
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                left: Val::Px(0.0),
                border_radius: BorderRadius::all(Val::Px(8.0)), // Makes it circular
                ..default()
            },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.9, 0.9, 0.9)) }, // Default color
            crate::theme::RuiThemeElement::SliderHandle,
            Interaction::None,
            Pickable::default(),
            bevy::ui::FocusPolicy::Pass,
            crate::widgets::RuiButtonStateColors {
                normal: Color::srgb(0.9, 0.9, 0.9),
                hovered: Color::srgb(1.0, 1.0, 1.0),
                pressed: Color::srgb(0.7, 0.7, 0.7),
            },
            RuiSliderHandle { slider_entity: slider_id },
        ));
    });

    cmds
}

pub fn handle_slider_interaction(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    focus: Res<bevy::input_focus::InputFocus>,
    bindings: Res<crate::navigation::RuiNavigationBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut q_slider: Query<(Entity, &mut RuiSlider, &Interaction, &ComputedNode, &mut BorderColor)>,
    mut q_handle: Query<(&mut Node, &RuiSliderHandle)>,
    mut q_fill: Query<(&mut Node, &RuiSliderFill), Without<RuiSliderHandle>>,
) {
    let just_pressed = mouse.just_pressed(MouseButton::Left);
    let pressed = mouse.pressed(MouseButton::Left);
    let just_released = mouse.just_released(MouseButton::Left);

    let cursor_pos = q_window.iter().next().and_then(|w| w.cursor_position());

    for (entity, mut slider, interaction, computed, mut border) in &mut q_slider {
        let is_focused = focus.get() == Some(entity);
        if is_focused {
            *border = BorderColor::all(Color::srgb(0.4, 0.6, 1.0));
        } else {
            *border = BorderColor::all(Color::srgb(0.1, 0.1, 0.1));
        }

        let is_hovered = *interaction == Interaction::Hovered || *interaction == Interaction::Pressed;

        if just_released {
            slider.dragging = false;
            slider.last_cursor_pos = None;
        }

        if just_pressed && is_hovered {
            slider.dragging = true;
            slider.last_cursor_pos = cursor_pos;
        }

        if is_focused {
            let mut move_x = 0.0;
            if crate::navigation::check_keys(&keys, &bindings.left_keys) || crate::navigation::check_buttons(&bindings.left_buttons, &gamepads) {
                move_x -= 1.0;
            }
            if crate::navigation::check_keys(&keys, &bindings.right_keys) || crate::navigation::check_buttons(&bindings.right_buttons, &gamepads) {
                move_x += 1.0;
            }
            if move_x != 0.0 {
                let range = slider.max - slider.min;
                // Move 5% per press
                slider.value = (slider.value + move_x * range * 0.05).clamp(slider.min, slider.max);
            }
        }

        if slider.dragging && pressed {
            if let (Some(pos), Some(last_pos)) = (cursor_pos, slider.last_cursor_pos) {
                let size_x = computed.size().x;
                if size_x > 0.0 {
                    let delta_x = pos.x - last_pos.x;
                    let delta_pct = delta_x / size_x;
                    let range = slider.max - slider.min;
                    slider.value = (slider.value + delta_pct * range).clamp(slider.min, slider.max);
                }
            }
            slider.last_cursor_pos = cursor_pos;
        }
    }

    for (mut node, handle) in &mut q_handle {
        if let Ok((_, slider, _, computed, _)) = q_slider.get(handle.slider_entity) {
            let pct = (slider.value - slider.min) / (slider.max - slider.min).max(0.0001);
            let size_x = computed.size().x;
            if size_x > 0.0 {
                let pixel_x = pct * size_x;
                node.left = Val::Px(pixel_x - 8.0); // Center the 16px handle
            }
        }
    }

    for (mut node, fill) in &mut q_fill {
        if let Ok((_, slider, _, _, _)) = q_slider.get(fill.slider_entity) {
            let pct = (slider.value - slider.min) / (slider.max - slider.min).max(0.0001);
            node.width = Val::Percent(pct * 100.0);
        }
    }
}
