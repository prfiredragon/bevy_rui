use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// Importamos los structs de eventos unificados nativos de Bevy 0.20
use bevy::picking::events::{PointerOver, PointerOut};
use bevy::text::FontSize;

#[derive(Component)]
pub struct RuiTooltipMarker;

/// Trait para permitir añadir tooltips de forma sencilla a cualquier EntityCommands
pub trait TooltipExt {
    fn with_tooltip(
        &mut self,
        text: impl Into<String>,
        bg_color: Color,
        border_color: Color,
    ) -> &mut Self;
}

impl<'w> TooltipExt for EntityCommands<'w> {
    fn with_tooltip(
        &mut self,
        text: impl Into<String>,
        bg_color: Color,
        border_color: Color,
    ) -> &mut Self {
        let text_content = text.into();
        
        self.insert(Pickable::default());

        // Evento al entrar el ratón utilizando el evento nativo PointerOver
        self.observe(move |_: On<PointerOver>, mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>| {
            // CORRECCIÓN: .single() ahora devuelve un Result de forma segura en las versiones modernas
            let (x, y) = if let Ok(window) = q_window.single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    (cursor_pos.x, cursor_pos.y - 35.0)
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            };
            
            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(x),
                    top: Val::Px(y),
                    padding: UiRect::all(Val::Px(8.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                ImageNode::default(),
                BackgroundColor(bg_color),
                BorderColor::from(border_color),
                GlobalZIndex(1000), 
                RuiTooltipMarker,
                Pickable::IGNORE,   
                Transform::default(),
                Visibility::default(),
            )).with_children(|parent| {
                parent.spawn((
                    Text::new(text_content.clone()),
                    TextFont { 
                        font_size: FontSize::Px(14.0), 
                        ..default() 
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // Evento al salir el ratón utilizando el evento nativo PointerOut
        self.observe(|_: On<PointerOut>, mut commands: Commands, q: Query<Entity, With<RuiTooltipMarker>>| {
            for entity in &q {
                commands.entity(entity).try_despawn();
            }
        });

        self
    }
}
