use bevy::prelude::*;
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiDropdown {
    pub popup_entity: Entity,
    pub text_entity: Entity,
    pub is_open: bool,
}

#[derive(Component)]
pub struct RuiDropdownOption {
    pub dropdown_entity: Entity,
    pub value: String,
}

pub fn spawn_dropdown<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    default_value: &str,
    options: &[&str],
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
        let mut s = Node { display: Display::Flex, justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, padding: UiRect::all(Val::Px(8.0)), width: Val::Px(150.0), height: Val::Px(35.0), ..default() };
        modifier(&mut s);
        let colors = RuiButtonStateColors::default();
        let mut cmds = parent_cmd.spawn((s, Button, bevy::ui::FocusPolicy::Block, colors.clone(), ImageNode::solid_color(colors.normal)));
        cmds.with_children(|parent| {
            let header_id = parent.target_entity();
            let text_id = parent.spawn((Text::new(default_value), TextFont::default(), TextColor(Color::WHITE))).id();
            parent.spawn((Text::new("▼"), TextFont::default(), TextColor(Color::WHITE)));
            let popup_id = parent.commands().spawn((
                Node { display: Display::None, position_type: PositionType::Absolute, flex_direction: FlexDirection::Column, width: Val::Px(150.0), border: UiRect::all(Val::Px(1.0)), ..default() },
                ImageNode::solid_color(Color::srgb(0.15, 0.15, 0.15)), BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), ZIndex(500), bevy::ui::FocusPolicy::Block,
                  GlobalZIndex(100), 
            )).with_children(|popup| {
                for opt in options {
                    popup.spawn((
                        Node { width: Val::Percent(100.0), padding: UiRect::all(Val::Px(8.0)), ..default() },
                        Button, bevy::ui::FocusPolicy::Block, Pickable::default(),
                        RuiButtonStateColors { normal: Color::srgb(0.15, 0.15, 0.15), hovered: Color::srgb(0.25, 0.25, 0.35), pressed: Color::srgb(0.1, 0.1, 0.2) },
                        ImageNode::solid_color(Color::srgb(0.15, 0.15, 0.15)),
                        RuiDropdownOption { dropdown_entity: header_id, value: opt.to_string() }
                    )).with_children(|opt_btn| {
                        opt_btn.spawn((Text::new(*opt), TextFont::default(), TextColor(Color::WHITE)));
                    });
                }
            }).id();
            parent.commands().entity(header_id).insert(RuiDropdown { popup_entity: popup_id, text_entity: text_id, is_open: false });
        });
        cmds
}

pub fn handle_dropdown_clicks(
    q_interactions: Query<(Entity, &Interaction, Option<&RuiDropdownOption>), Changed<Interaction>>,
    mut q_dropdowns: Query<&mut RuiDropdown>,
    mut q_popup: Query<&mut Node>,
    mut q_text: Query<&mut Text>,
) {
    for (entity, interaction, opt_option) in &q_interactions {
        if *interaction == Interaction::Pressed {
            println!("Dropdown clicked! GlobalTransform: {:?}", q_dropdowns.get_mut(entity).map(|_| "found"));
            if let Ok(mut dropdown) = q_dropdowns.get_mut(entity) {
                dropdown.is_open = !dropdown.is_open;
                if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { node.display = if dropdown.is_open { Display::Flex } else { Display::None }; }
            }
            if let Some(option) = opt_option {
                if let Ok(mut dropdown) = q_dropdowns.get_mut(option.dropdown_entity) {
                    dropdown.is_open = false;
                    if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { node.display = Display::None; }
                    if let Ok(mut text) = q_text.get_mut(dropdown.text_entity) { text.0 = option.value.clone(); }
                }
            }
        }
    }
}

pub fn close_dropdowns_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_dropdowns: Query<(&Interaction, &mut RuiDropdown)>,
    mut q_popup: Query<&mut Node>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (interaction, mut dropdown) in &mut q_dropdowns {
            if dropdown.is_open && *interaction == Interaction::None {
                dropdown.is_open = false;
                if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { node.display = Display::None; }
            }
        }
    }
}

pub fn update_dropdown_positions(
    q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_dropdowns: Query<(&RuiDropdown, &bevy::ui::UiGlobalTransform, &ComputedNode)>,
    mut q_popups: Query<&mut Node, Without<RuiDropdown>>,
) {
    let Some(window) = q_window.iter().next() else { return; };

    for (dropdown, transform, computed) in &q_dropdowns {
        if dropdown.is_open {
            if let Ok(mut popup_node) = q_popups.get_mut(dropdown.popup_entity) {
                let pos = transform.to_scale_angle_translation().2;
                let size = computed.size();
                
                let scale_factor = window.scale_factor();
                
                let logical_pos_x = pos.x / scale_factor;
                let logical_pos_y = pos.y / scale_factor;
                let logical_size_x = size.x / scale_factor;
                let logical_size_y = size.y / scale_factor;
                
                popup_node.left = Val::Px(logical_pos_x - logical_size_x / 2.0);
                popup_node.top = Val::Px(logical_pos_y + logical_size_y / 2.0);
                popup_node.width = Val::Px(logical_size_x);
            }
        }
    }
}