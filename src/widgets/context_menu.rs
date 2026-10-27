use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy::ui_widgets::Activate;
use bevy::text::FontSize;
use crate::widgets::{RuiButtonStateColors, RuiBuilderExt, menu::RuiIcon};
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

// CORRECCIÓN: Tiempo de vida 'w unificado para evitar fallos de varianza
pub fn spawn_context_menu<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
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
        ImageNode::default(),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor::from(Color::srgb(0.3, 0.3, 0.3)), 
        ZIndex(1000), 
        GlobalZIndex(1000),
        RuiThemeElement::ContextMenuBg,
    )).with_children(|popup| {
        // SOLUCIÓN AL LIFETIME: Forzamos reborrow local para satisfacer la varianza del framework
        let mut popup_spawner = popup.reborrow();
        popup_spawner.scrollview(|_|{}, |scroll| {
            let mut scroll_spawner = scroll.reborrow();
            build_items(&mut scroll_spawner);
        });
    }).id();

    // Adjuntar al nodo padre con el observador reactivo de clic derecho de Bevy 0.20
    parent.commands().entity(target_id)
        .insert((
            RuiContextMenu { popup_entity: popup_id, is_open: false },
            RelativeCursorPosition::default(),
        ));
}

// CORRECCIÓN: Firma unificada con el tiempo de vida global 'w
pub fn spawn_context_menu_item<'w>(
    parent: &'w mut ChildSpawnerCommands<'w>,
    label: &str,
    icon: Option<RuiIcon>,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut s = Node {
        width: Val::Percent(100.0), height: Val::Px(30.0), padding: UiRect::horizontal(Val::Px(8.0)),
        align_items: AlignItems::Center, justify_content: JustifyContent::FlexStart, ..default()
    };
    modifier(&mut s);

    let popup_id = parent.target_entity();
    let colors = RuiButtonStateColors {
        normal: Color::NONE, hovered: Color::srgb(0.25, 0.25, 0.35), pressed: Color::srgb(0.15, 0.15, 0.2),
    };

    let mut cmds = parent.spawn((
        bevy::ui_widgets::Button, // Widget headless nativo de Bevy 0.20
        s, ImageNode::default(), BackgroundColor(Color::NONE),
        colors, crate::focus::Focusable,
        RuiContextMenuItem { popup_entity: popup_id },
        RuiThemeElement::ListItem,
    ));

    cmds.with_children(|inner| {
        if let Some(icon_type) = icon {
            inner.spawn(Node {
                width: Val::Px(16.0), height: Val::Px(16.0), justify_content: JustifyContent::Center,
                align_items: AlignItems::Center, margin: UiRect::right(Val::Px(8.0)), ..default()
            }).with_children(|icon_box| {
                match icon_type {
                    RuiIcon::Emoji(e, font_opt) => {
                        let mut text_font = TextFont { font_size: FontSize::Px(14.0), ..default() };
                        if let Some(f) = font_opt { text_font.font = FontSource::Handle(f); }
                        icon_box.spawn((Text::new(e), text_font, TextColor(Color::WHITE)));
                    },
                    RuiIcon::Texture(h) => {
                        icon_box.spawn((ImageNode::new(h), Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() }));
                    }
                }
            });
        }

        inner.spawn((Text::new(label), TextFont::default(), TextColor(Color::WHITE), RuiThemeElement::Text));
    });

    // CORRECCIÓN REACTIVA: Al hacer clic en una opción del menú, este se cierra de forma autocontenida
    cmds.observe(
        move |trigger: On<Activate>,
              q_items: Query<&RuiContextMenuItem>,
              mut q_menus: Query<&mut RuiContextMenu>,
              mut q_popups: Query<&mut Node>,
              mut active_scope: ResMut<crate::focus::RuiActiveScope>|
        {
            if let Ok(item) = q_items.get(trigger.entity) {
                for mut ctx in &mut q_menus {
                    if ctx.popup_entity == item.popup_entity {
                        ctx.is_open = false;
                        active_scope.remove_window(ctx.popup_entity);
                        if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) { node.display = Display::None; }
                    }
                }
            }
        }
    );

    cmds
}



use bevy::window::PrimaryWindow;
use bevy::picking::hover::Hovered;

pub fn handle_context_menu_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    // CORRECCIÓN: Pedimos Option<&Hovered> para verificar si el componente existe en la entidad
    mut q_menus: Query<(Option<&Hovered>, &mut RuiContextMenu)>,
    mut q_popups: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    let Ok(window) = q_window.single() else { return; };
    if !mouse.just_pressed(MouseButton::Right) { return; }
    
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let mut opened_any = false;

    // 1. Abrir el menú si el cursor está sobre el elemento asignado
    for (hovered, mut ctx) in &mut q_menus {
        if hovered.is_some() {
            ctx.is_open = true;
            opened_any = true;
            active_scope.push_window(ctx.popup_entity);
            
            if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                node.display = Display::Flex;
                node.left = Val::Px(cursor_pos.x);
                node.top = Val::Px(cursor_pos.y);
                
                let window_height = window.height();
                let margin_bottom = 10.0;
                if cursor_pos.y < window_height {
                    node.max_height = Val::Px(window_height - cursor_pos.y - margin_bottom);
                }
            }
        }
    }

    // 2. Cerrar los otros menús si se abrió uno nuevo
    if opened_any {
        for (hovered, mut ctx) in &mut q_menus {
            if hovered.is_none() && ctx.is_open {
                ctx.is_open = false;
                active_scope.remove_window(ctx.popup_entity);
                if let Ok(mut node) = q_popups.get_mut(ctx.popup_entity) {
                    node.display = Display::None;
                }
            }
        }
    }
}
pub fn close_context_menus_on_outside_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_menus: Query<(Option<&Hovered>, &mut RuiContextMenu)>,
    mut q_popups: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
        let mut hovered_any = false;
        if mouse.just_pressed(MouseButton::Right) {
            for (hovered, _) in &q_menus {
                if hovered.is_some() { hovered_any = true; }
            }
        }

        if !hovered_any {
            for (hovered, mut ctx) in &mut q_menus {
                if ctx.is_open && hovered.is_none() {
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
