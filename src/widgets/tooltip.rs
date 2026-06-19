use bevy::prelude::*;

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

impl TooltipExt for EntityCommands<'_> {
    fn with_tooltip(
        &mut self,
        text: impl Into<String>,
        bg_color: Color,
        border_color: Color,
    ) -> &mut Self {
        let text_content = text.into();
        self.insert((Pickable::default(), Interaction::None));

        // Evento al entrar el ratón: Crea el tooltip en coordenadas globales
        self.observe(move |over: On<Pointer<Over>>, mut commands: Commands| {
            // Obtenemos la posición lógica del puntero (ventana) desde el evento
            let mouse_pos = over.pointer_location.position;
            let x = mouse_pos.x;
            let y = mouse_pos.y - 35.0; // Lo posicionamos ligeramente arriba del cursor
            
            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(x),
                    top: Val::Px(y),
                    padding: UiRect::all(Val::Px(8.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                ImageNode::solid_color(bg_color),
                BorderColor::all(border_color),
                GlobalZIndex(1000), // Encima de todo
                RuiTooltipMarker,
                Pickable::IGNORE, // El tooltip no debe interferir con el puntero
                Transform::default(),
                Visibility::default(),
            )).with_child((
                Text::new(text_content.clone()),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Evento al salir el ratón: Elimina el tooltip
        self.observe(|_: On<Pointer<Out>>, mut commands: Commands, q: Query<Entity, With<RuiTooltipMarker>>| {
            for entity in &q {
                commands.entity(entity).despawn();
            }
        });

        self
    }
}