use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use crate::widgets::{RuiButtonStateColors, RuiBuilderExt};

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

pub fn spawn_dropdown<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    default_value: &str,
    options: &[&str],
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut s = Node {
        padding: UiRect::all(Val::Px(4.0)),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        position_type: PositionType::Relative,
        ..default()
    };
    modifier(&mut s);
    
    let colors = RuiButtonStateColors::default();
    
    let mut cmds = parent_cmd.spawn((
        s, 
        bevy::ui_widgets::Button, 
        crate::focus::Focusable, 
        colors.clone(), 
        ImageNode::default(),
        BackgroundColor(colors.normal),
    ));
    
    let header_id = cmds.id();
    cmds.with_children(|parent| {
        let text_id = parent.spawn((
            Text::new(default_value), 
            TextFont::default(), 
            TextColor(Color::WHITE),
            TextLayout { justify: Justify::Left, ..default() },
            Node { overflow: Overflow::clip(), flex_shrink: 1.0, min_width: Val::Px(0.0), ..default() }
        )).id();
        
        parent.spawn((Text::new("▼"), TextFont::default(), TextColor(Color::WHITE)));
        
        let popup_id = parent.spawn((
            Node { 
                display: Display::None, 
                position_type: PositionType::Absolute, 
                flex_direction: FlexDirection::Column, 
                top: Val::Percent(100.0), 
                left: Val::Px(0.0),       
                width: Val::Percent(100.0), 
                border: UiRect::all(Val::Px(1.0)), 
                ..default() 
            },
            ImageNode::default(), 
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)), 
            BorderColor::from(Color::srgb(0.3, 0.3, 0.3)), 
            ZIndex(999), 
            GlobalZIndex(999), 
            crate::theme::RuiThemeElement::DropdownBg,
        )).with_children(|popup| {
            let mut popup_spawner = popup.reborrow();
            
            popup_spawner.vbox(|s| { s.width = Val::Percent(100.0); s.height = Val::Auto; }, |vbox| {
                let mut vbox_spawner = vbox.reborrow();
                
                for opt in options {
                    let opt_string = opt.to_string();
                    let mut current_item = vbox_spawner.reborrow();
                    
                    let normal_color = Color::srgb(0.18, 0.18, 0.18);
                    let hover_color = Color::srgb(0.25, 0.25, 0.35);

                    // CORRECCIÓN MAGISTRAL DE TEMAS: Eliminamos 'ListItem' para que el sistema de temas 
                    // no pise los colores de forma agresiva y permita el parpadeo fluido del hover.
                    let opt_btn_id = current_item.spawn((
                        Node { 
                            width: Val::Percent(100.0), 
                            height: Val::Px(35.0), 
                            padding: UiRect::all(Val::Px(8.0)), 
                            align_items: AlignItems::Center,
                            ..default() 
                        },
                        bevy::ui_widgets::Button, 
                        crate::focus::Focusable, 
                        RuiButtonStateColors { normal: normal_color, hovered: hover_color, pressed: hover_color },
                        ImageNode::default(),
                        BackgroundColor(normal_color),
                        RuiDropdownOption { dropdown_entity: header_id, value: opt_string.clone() },
                    )).with_children(|opt_btn| {
                        opt_btn.spawn((
                            Text::new(*opt), 
                            TextFont::default(), 
                            TextColor(Color::WHITE),
                            TextLayout { justify: Justify::Left, ..default() },
                            Node { overflow: Overflow::clip(), flex_shrink: 1.0, min_width: Val::Px(0.0), ..default() },
                            crate::theme::RuiThemeElement::Text,
                        ));
                    }).id();

                    // Observadores de picking directos
                    current_item.commands().entity(opt_btn_id).observe(move |_: On<PointerOver>, mut query: Query<&mut BackgroundColor>| {
                        if let Ok(mut bg) = query.get_mut(opt_btn_id) { bg.0 = hover_color; }
                    });

                    current_item.commands().entity(opt_btn_id).observe(move |_: On<PointerOut>, mut query: Query<&mut BackgroundColor>| {
                        if let Ok(mut bg) = query.get_mut(opt_btn_id) { bg.0 = normal_color; }
                    });

                    current_item.commands().entity(opt_btn_id).observe(
                        move |trigger: On<Activate>,
                              q_opt: Query<&RuiDropdownOption>,
                              mut q_dropdowns: Query<&mut RuiDropdown>,
                              mut q_popup: Query<&mut Node>,
                              mut q_text: Query<&mut Text>,
                              mut active_scope: ResMut<crate::focus::RuiActiveScope>| 
                        {
                            if let Ok(option) = q_opt.get(trigger.entity) {
                                if let Ok(mut dropdown) = q_dropdowns.get_mut(option.dropdown_entity) {
                                    dropdown.is_open = false;
                                    active_scope.remove_window(dropdown.popup_entity);
                                    if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { 
                                        node.display = Display::None; 
                                    }
                                    if let Ok(mut text) = q_text.get_mut(dropdown.text_entity) { 
                                        text.0 = option.value.clone(); 
                                    }
                                }
                            }
                        }
                    );
                }
            });
        }).id();
        
        parent.commands().entity(header_id).insert(RuiDropdown { popup_entity: popup_id, text_entity: text_id, is_open: false });

        parent.commands().entity(header_id).observe(
            move |trigger: On<Activate>,
                  mut q_dropdowns: Query<&mut RuiDropdown>,
                  mut q_popup: Query<&mut Node>,
                  mut active_scope: ResMut<crate::focus::RuiActiveScope>|
            {
                if let Ok(mut dropdown) = q_dropdowns.get_mut(trigger.entity) {
                    dropdown.is_open = !dropdown.is_open;
                    if dropdown.is_open {
                        active_scope.push_window(dropdown.popup_entity);
                    } else {
                        active_scope.remove_window(dropdown.popup_entity);
                    }
                    if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { 
                        node.display = if dropdown.is_open { Display::Flex } else { Display::None }; 
                    }
                }
            }
        );
    });
    cmds
}

pub fn close_dropdowns_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_dropdowns: Query<&mut RuiDropdown>,
    mut q_popup: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for mut dropdown in &mut q_dropdowns {
            if dropdown.is_open {
                dropdown.is_open = false;
                active_scope.remove_window(dropdown.popup_entity);
                if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { 
                    node.display = Display::None; 
                }
            }
        }
    }
}

pub fn update_dropdown_positions() {}
