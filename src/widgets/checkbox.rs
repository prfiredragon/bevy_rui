use bevy::prelude::*;
use crate::widgets::RuiButtonStateColors;

#[derive(Component)]
pub struct RuiCheckbox {
    pub checked: bool,
    pub checkmark_entity: Entity,
}

pub fn spawn_checkbox<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    label_text: &str,
    checked: bool,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
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
            Button,
            crate::focus::Focusable,
            bevy::ui::FocusPolicy::Block,
            Pickable::default(),
            colors,
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::NONE) },
        ));

        let mut checkmark_entity = None;
        cmds.with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(24.0), height: Val::Px(24.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center, align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(8.0)), ..default()
                },
                BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.1, 0.1, 0.1)) },
            )).with_children(|box_parent| {
                checkmark_entity = Some(box_parent.spawn((
                    Node {
                        width: Val::Px(12.0),
                        height: Val::Px(12.0),
                        ..default()
                    },
                    ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::WHITE) },
                    if checked { Visibility::Inherited } else { Visibility::Hidden },
                )).id());
            });
            parent.spawn((Text::new(label_text), TextFont { font_size: bevy::prelude::FontSize::Px(18.0), ..default() }, TextColor(Color::WHITE)));
        });

        cmds.insert(RuiCheckbox { checked, checkmark_entity: checkmark_entity.unwrap() });
        cmds
}

pub fn handle_checkbox_clicks(
    mut query: Query<(&Interaction, &mut RuiCheckbox), Changed<Interaction>>,
    mut q_vis: Query<&mut Visibility>,
) {
    for (interaction, mut checkbox) in &mut query {
        if *interaction == Interaction::Pressed {
            checkbox.checked = !checkbox.checked;
            if let Ok(mut vis) = q_vis.get_mut(checkbox.checkmark_entity) {
                *vis = if checkbox.checked { Visibility::Inherited } else { Visibility::Hidden };
            }
        }
    }
}