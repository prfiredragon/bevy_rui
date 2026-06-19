/* 
#[cfg(feature = "bevy-0-18-1")]
extern crate bevy_0181 as bevy; */

use bevy::prelude::*;

/// Recurso global y estricto. Solo una (1) entidad puede tener el foco a la vez.
/// Si este valor es `Some(entity)`, solo esa entidad escuchará el teclado.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct RuiFocus {
    pub entity: Option<Entity>,
}

/// Componente marcador para decirle al sistema que este widget (ej: TextBox) puede recibir foco
#[derive(Component, Default)]
pub struct Focusable;

pub struct RuiFocusPlugin;

impl Plugin for RuiFocusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RuiFocus>();
        app.register_type::<RuiFocus>();

        app.add_systems(Update, handle_focus_clicks);
    }
}

/// Escucha los clics del ratón para decidir a quién darle el foco del teclado.
pub fn handle_focus_clicks(
    mut focus: ResMut<RuiFocus>,
    mouse: Res<ButtonInput<MouseButton>>,
    focusables: Query<(Entity, &Interaction), With<Focusable>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut clicked_entity = None;
        for (entity, interaction) in &focusables {
            // Interaction respeta el Z-Index y la oclusión de ventanas automáticamente
            if *interaction == Interaction::Pressed {
                clicked_entity = Some(entity);
            }
        }
        focus.entity = clicked_entity;
    }
}