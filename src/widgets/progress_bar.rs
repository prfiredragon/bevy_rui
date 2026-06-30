use bevy::prelude::*;

pub fn spawn_progress_bar<'a>(
    parent: &'a mut ChildSpawnerCommands,
    min: f32,
    max: f32,
    value: f32,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        width: Val::Percent(100.0),
        height: Val::Px(16.0),
        align_items: AlignItems::Center,
        ..default()
    };
    modifier(&mut s);

    let pct = if max > min {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let mut cmds = parent.spawn((
        s,
        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.1, 0.1, 0.1)) },
        crate::theme::RuiThemeElement::ProgressBarTrack,
    ));

    cmds.with_children(|p| {
        p.spawn((
            Node {
                width: Val::Percent(pct * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.4, 0.8, 0.4)) },
            crate::theme::RuiThemeElement::ProgressBarFill,
        ));
    });

    cmds
}
