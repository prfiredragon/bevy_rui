//! Hover tooltip extension trait ([`TooltipExt`]) for [`EntityCommands`].

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::picking::events::{PointerOver, PointerOut};
use bevy::text::FontSize;

/// Marker component attached to active floating tooltip popup entities.
#[derive(Component)]
pub struct RuiTooltipMarker;

/// Extension trait allowing any UI entity to display a floating tooltip on pointer hover (`PointerOver` / `PointerOut`).
pub trait TooltipExt {
    /// Attaches a hover tooltip popup with `text`, `bg_color`, and `border_color` to this UI entity.
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

        self.observe(move |_: On<PointerOver>, mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>| {
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

        self.observe(|_: On<PointerOut>, mut commands: Commands, q: Query<Entity, With<RuiTooltipMarker>>| {
            for entity in &q {
                commands.entity(entity).try_despawn();
            }
        });

        self
    }
}
