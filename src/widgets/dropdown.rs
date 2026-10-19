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
    let mut text_id = Entity::PLACEHOLDER;

    cmds.with_children(|parent| {
        text_id = parent.spawn((
            Text::new(default_value), 
            TextFont::default(), 
            TextColor(Color::WHITE),
            TextLayout { linebreak: LineBreak::NoWrap, justify: Justify::Left },
            Node { overflow: Overflow::clip(), flex_shrink: 0.0, min_width: Val::Px(0.0), ..default() }
        )).id();
        
        parent.spawn((Text::new("▼"), TextFont::default(), TextColor(Color::WHITE)));
    });

    // CORRECCIÓN RADICAL CONTRA E0499: Extraemos los comandos globales a través de cmds de forma síncrona
    let mut global_commands = cmds.commands();
    
    // Spawneamos el popup de manera 100% independiente en la raíz del mundo usando global_commands
    let popup_id = global_commands.spawn((
        Node { 
            display: Display::None, 
            position_type: PositionType::Absolute, 
            flex_direction: FlexDirection::Column, 
            width: Val::Px(200.0), 
            border: UiRect::all(Val::Px(1.0)), 
            ..default() 
        },
        ImageNode::default(), 
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)), 
        BorderColor::from(Color::srgb(0.3, 0.3, 0.3)), 
        ZIndex(2000), 
        GlobalZIndex(2000), // ELEVACIÓN ABSOLUTA EN LA GPU: Flota de forma limpia ignorando el Overflow de los padres
        ChildOf(header_id),
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

                let opt_btn_id = current_item.spawn((
                    Node { 
                        width: Val::Percent(100.0), 
                        height: Val::Px(35.0), 
                        padding: UiRect::all(Val::Px(8.0)), 
                        align_items: AlignItems::Center,
                        ..default() 
                    },
                    Pickable::default(), 
                    ImageNode::default(),
                    BackgroundColor(normal_color),
                    RuiDropdownOption { dropdown_entity: header_id, value: opt_string.clone() },
                )).with_children(|opt_btn| {
                    opt_btn.spawn((
                        Text::new(*opt), 
                        TextFont::default(), 
                        TextColor(Color::WHITE),
                        TextLayout { linebreak: LineBreak::NoWrap, justify: Justify::Left },
                        Node { overflow: Overflow::clip(), flex_shrink: 0.0, min_width: Val::Px(0.0), ..default() },
                        crate::theme::RuiThemeElement::Text,
                    ));
                }).id();

                current_item.commands().entity(opt_btn_id).observe(move |_: On<PointerOver>, mut query: Query<&mut BackgroundColor>| {
                    if let Ok(mut bg) = query.get_mut(opt_btn_id) { bg.0 = hover_color; }
                });

                current_item.commands().entity(opt_btn_id).observe(move |_: On<PointerOut>, mut query: Query<&mut BackgroundColor>| {
                    if let Ok(mut bg) = query.get_mut(opt_btn_id) { bg.0 = normal_color; }
                });

                current_item.commands().entity(opt_btn_id).observe(
                    move |trigger: On<PointerClick>,
                          q_opt: Query<&RuiDropdownOption>,
                          mut q_dropdowns: Query<&mut RuiDropdown>,
                          mut q_popup: Query<&mut Node>,
                          mut q_text: Query<&mut Text>,
                          mut q_dialog: Query<&mut crate::widgets::file_dialog::RuiFileDialog>| 
                    {
                        if let Ok(option) = q_opt.get(trigger.entity) {
                            if let Ok(mut dropdown) = q_dropdowns.get_mut(option.dropdown_entity) {
                                dropdown.is_open = false;
                                // SANEAMIENTO CONTRA CRASHES: Eliminamos active_scope para no colisionar con AutoDirectionalNavigation
                                if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { 
                                    node.display = Display::None; 
                                }
                                if let Ok(mut text) = q_text.get_mut(dropdown.text_entity) { 
                                    text.0 = option.value.clone(); 
                                }
                                for mut dialog in &mut q_dialog {
                                    if dialog.filter_dropdown == Some(option.dropdown_entity) {
                                        dialog.needs_refresh = true;
                                    }
                                }
                            }
                        }
                    }
                );
            }
        });
    }).id();
    
    // Inyectamos el componente de datos y el observador sobre el handle original
    cmds.insert(RuiDropdown { popup_entity: popup_id, text_entity: text_id, is_open: false });

    cmds.observe(
        move |trigger: On<Activate>,
              mut q_dropdowns: Query<&mut RuiDropdown>,
              mut q_popup: Query<&mut Node>,
              q_transform: Query<(&GlobalTransform, &ComputedNode)>|
        {
            let dropdown_ent = trigger.entity;
            if let Ok(mut dropdown) = q_dropdowns.get_mut(dropdown_ent) {
                dropdown.is_open = !dropdown.is_open;
                if dropdown.is_open {
                    if let Ok((global_transform, computed_node)) = q_transform.get(dropdown_ent) {
                        let size = computed_node.size();
                        let translation = global_transform.translation();
                        
                        if let Ok(mut popup_node) = q_popup.get_mut(dropdown.popup_entity) {
                            // Posicionamiento absoluto flotante pixel-perfect
                            popup_node.width = Val::Px(size.x);
                            popup_node.left = Val::Px(translation.x - size.x / 2.0);
                            popup_node.top = Val::Px(translation.y + size.y / 2.0);
                        }
                    }
                }
                
                if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { 
                    node.display = if dropdown.is_open { Display::Flex } else { Display::None }; 
                }
            }
        }
    );

    cmds
}
pub fn close_dropdowns_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_dropdowns: Query<&mut RuiDropdown>,
    mut q_popup: Query<&mut Node>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for mut dropdown in &mut q_dropdowns {
            if dropdown.is_open {
                dropdown.is_open = false;
                if let Ok(mut node) = q_popup.get_mut(dropdown.popup_entity) { 
                    node.display = Display::None; 
                }
            }
        }
    }
}

pub fn update_dropdown_positions() {}
