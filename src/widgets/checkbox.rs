//! Toggleable checkbox widget with a text label.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::text::FontSize;
use crate::widgets::RuiButtonStateColors;

/// Component storing the boolean state of a checkbox widget and its inner checkmark entity.
#[derive(Component)]
pub struct RuiCheckbox {
    /// Whether the checkbox is currently checked.
    pub checked: bool,
    /// Entity of the inner checkmark icon whose [`Visibility`] is toggled.
    pub checkmark_entity: Entity,
}

/// Spawns a toggleable checkbox widget with `label_text` and initial `checked` state.
///
/// Automatically toggles [`RuiCheckbox::checked`] and updates checkmark visibility on [`Activate`].
/// You can attach an additional `.observe(|trigger: On<Activate>, q: Query<&RuiCheckbox>| ...)`
/// to react to state changes.
///
/// Also available via [`crate::widgets::RuiBuilderExt::checkbox`].
pub fn spawn_checkbox<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    label_text: &str,
    checked: bool,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
        let mut s = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        };
        modifier(&mut s);

        let colors = RuiButtonStateColors {
            normal: Color::NONE,
            hovered: Color::srgba(1.0, 1.0, 1.0, 0.05),
            pressed: Color::srgba(1.0, 1.0, 1.0, 0.1),
        };

        let mut cmds = parent_cmd.spawn((
            s,
            bevy::ui_widgets::Button,
            crate::focus::Focusable,
            colors,
            ImageNode::default(),
            BackgroundColor(Color::NONE),
        ));

        let mut checkmark_entity = None;
        cmds.with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(24.0), 
                    height: Val::Px(24.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center, 
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(8.0)), 
                    ..default()
                },
                BorderColor::from(Color::srgb(0.5, 0.5, 0.5)),
                ImageNode::default(),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                crate::theme::RuiThemeElement::CheckboxBg,
            )).with_children(|box_parent| {
                checkmark_entity = Some(box_parent.spawn((
                    Node {
                        width: Val::Px(12.0),
                        height: Val::Px(12.0),
                        ..default()
                    },
                    ImageNode::default(),
                    BackgroundColor(Color::WHITE),
                    crate::theme::RuiThemeElement::CheckboxIcon,
                    if checked { Visibility::Inherited } else { Visibility::Hidden },
                )).id());
            });
            
            parent.spawn((
                Text::new(label_text), 
                TextFont { font_size: FontSize::Px(18.0), ..default() }, 
                TextColor(Color::WHITE)
            ));
        });

        let final_checkmark = checkmark_entity.unwrap();

        cmds.insert(RuiCheckbox { checked, checkmark_entity: final_checkmark });

        cmds.observe(
            move |trigger: On<Activate>, 
                  mut q_checkbox: Query<&mut RuiCheckbox>, 
                  mut q_vis: Query<&mut Visibility>| 
            {
                let target_entity = trigger.entity;
                if let Ok(mut checkbox) = q_checkbox.get_mut(target_entity) {
                    checkbox.checked = !checkbox.checked;
                    if let Ok(mut vis) = q_vis.get_mut(checkbox.checkmark_entity) {
                        *vis = if checkbox.checked { Visibility::Inherited } else { Visibility::Hidden };
                    }
                }
            }
        );

        cmds
}
