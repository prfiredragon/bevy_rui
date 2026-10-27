use bevy::prelude::*;
use bevy::ui_widgets::{Activate, MenuButton, MenuPopup, MenuLayout, MenuItem};
use crate::widgets::RuiButtonStateColors;

/// Estado del dropdown (abierto/cerrado) y referencias a entidades internas.
#[derive(Component)]
pub struct RuiDropdown {
    pub popup_entity: Entity,
    pub text_entity: Entity,
    pub is_open: bool,
}

/// Componente en cada opción del dropdown.
#[derive(Component)]
pub struct RuiDropdownOption {
    pub dropdown_entity: Entity,
    pub value: String,
}

/// Crea un dropdown nativo usando MenuButton + MenuPopup + MenuItem de Bevy 0.20.
pub fn spawn_dropdown<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    default_value: &str,
    options: &[&str],
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut header_node = Node {
        padding: UiRect::all(Val::Px(4.0)),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        overflow: Overflow::visible(),
        ..default()
    };
    modifier(&mut header_node);

    let colors = RuiButtonStateColors::default();

    let mut cmds = parent_cmd.spawn((
        header_node,
        bevy::ui_widgets::Button,
        MenuButton,
        crate::focus::Focusable,
        colors.clone(),
        ImageNode::default(),
        BackgroundColor(colors.normal),
        crate::theme::RuiThemeElement::Button,
    ));

    let header_id = cmds.id();
    let mut text_id = Entity::PLACEHOLDER;
    let mut popup_id = Entity::PLACEHOLDER;

    cmds.with_children(|parent| {
        // Texto que muestra el valor seleccionado
        text_id = parent.spawn((
            Text::new(default_value),
            TextFont::default(),
            TextColor(Color::WHITE),
            TextLayout { linebreak: LineBreak::NoWrap, justify: Justify::Left },
            Node {
                overflow: Overflow::clip(),
                flex_shrink: 1.0,
                min_width: Val::Px(0.0),
                flex_grow: 1.0,
                ..default()
            },
            crate::theme::RuiThemeElement::Text,
        )).id();

        // Flecha indicadora
        parent.spawn((
            Text::new("▼"),
            TextFont { font_size: bevy::prelude::FontSize::Px(12.0), ..default() },
            TextColor(Color::WHITE),
            crate::theme::RuiThemeElement::Text,
        ));

        // ── POPUP (hijo del header, posicionado justo abajo) ──────────────────
        let options_owned: Vec<String> = options.iter().map(|s| s.to_string()).collect();
        popup_id = parent.spawn((
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                bottom: Val::Percent(100.0),
                left: Val::Px(0.0),
                min_width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            ZIndex(500),
            MenuPopup { layout: MenuLayout::Column },
            GlobalZIndex(500),
            ImageNode::default(),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor::from(Color::srgb(0.3, 0.3, 0.3)),
            crate::theme::RuiThemeElement::DropdownBg,
            bevy::ui::FocusPolicy::Block,
        )).with_children(move |popup| {
            // Cada opción es un MenuItem nativo
            for (i, opt) in options_owned.iter().enumerate() {
                let opt_str = opt.clone();
                let item_id = popup.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        min_width: Val::Px(0.0),
                        height: Val::Px(32.0),
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    MenuItem,
                    bevy::input_focus::tab_navigation::TabIndex(i as i32),
                    bevy::ui_widgets::Button,
                    RuiButtonStateColors {
                        normal: Color::NONE,
                        hovered: Color::srgb(0.25, 0.25, 0.35),
                        pressed: Color::srgb(0.1, 0.1, 0.15),
                    },
                    ImageNode::default(),
                    BackgroundColor(Color::NONE),
                    RuiDropdownOption { dropdown_entity: header_id, value: opt_str.clone() },
                    crate::theme::RuiThemeElement::ListItem,
                    bevy::ui::FocusPolicy::Block,
                    Pickable::default(),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(opt_str),
                        TextFont::default(),
                        TextColor(Color::WHITE),
                        TextLayout { linebreak: LineBreak::NoWrap, justify: Justify::Left },
                        Node { overflow: Overflow::clip(), flex_shrink: 1.0, min_width: Val::Px(0.0), ..default() },
                        crate::theme::RuiThemeElement::Text,
                    ));
                }).id();

                // Observer en cada ítem: Activate se dispara al hacer clic o presionar Enter/Space
                popup.commands().entity(item_id).observe(
                    move |trigger: On<Activate>,
                          q_opt: Query<&RuiDropdownOption>,
                          mut q_dropdown: Query<&mut RuiDropdown>,
                          mut q_node: Query<&mut Node>,
                          mut commands: Commands,
                          mut q_dialog: Query<&mut crate::widgets::file_dialog::RuiFileDialog>|
                    {   
                        let item_entity = trigger.entity;
                        if let Ok(option) = q_opt.get(item_entity) {
                            let val2 = option.value.clone();
                            let dropdown_entity = option.dropdown_entity;

                            // Actualizar el texto del header y cerrar el popup
                            if let Ok(mut dropdown) = q_dropdown.get_mut(dropdown_entity) {
                                let te = dropdown.text_entity;
                                commands.entity(te).insert(Text::new(val2));

                                dropdown.is_open = false;
                                let popup_e = dropdown.popup_entity;
                                if let Ok(mut node) = q_node.get_mut(popup_e) {
                                    node.display = Display::None;
                                }
                            }

                            // Notificar file_dialog si corresponde
                            for mut dialog in q_dialog.iter_mut() {
                                if dialog.filter_dropdown == Some(dropdown_entity) {
                                    dialog.needs_refresh = true;
                                }
                            }
                        }
                    }
                );
            }
        }).id();
    });

    // Insertar RuiDropdown con el ID real del popup
    cmds.insert(RuiDropdown { popup_entity: popup_id, text_entity: text_id, is_open: false });

    // Observador en el botón del encabezado para alternar (abrir/cerrar) el popup al hacer clic
    cmds.observe(|trigger: On<Activate>, mut q_dropdown: Query<&mut RuiDropdown>, mut q_node: Query<&mut Node>| {
        let header_entity = trigger.entity;
        if let Ok(mut dropdown) = q_dropdown.get_mut(header_entity) {
            dropdown.is_open = !dropdown.is_open;
            let is_open = dropdown.is_open;
            let popup_e = dropdown.popup_entity;

            if let Ok(mut node) = q_node.get_mut(popup_e) {
                node.display = if is_open { Display::Flex } else { Display::None };
            }
        }
    });

    cmds
}

pub fn update_dropdown_positions() {}
pub fn close_dropdowns_on_outside_click() {}
