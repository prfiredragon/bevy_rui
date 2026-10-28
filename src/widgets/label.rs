//! Themed text label widget.

use bevy::prelude::*;

/// Spawns a themed text label with [`Pickable::IGNORE`] and [`crate::theme::RuiThemeElement::Text`].
///
/// Also available via [`crate::widgets::RuiBuilderExt::label`].
pub fn spawn_label<'a>(
    parent: &'a mut ChildSpawnerCommands,
    text: &str,
    modifier: impl FnOnce(&mut TextFont, &mut TextColor),
) -> EntityCommands<'a> {
    let mut font = TextFont::default();
    let mut color = TextColor(Color::WHITE);
    modifier(&mut font, &mut color);

    parent.spawn((Text::new(text), font, color, Pickable::IGNORE, crate::theme::RuiThemeElement::Text))
}