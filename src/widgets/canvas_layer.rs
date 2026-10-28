//! Full-screen HUD overlay layer targeted at a specific camera.

use bevy::prelude::*;

/// Spawns an absolute `100% x 100%` overlay container rendered on `camera_entity` via [`UiTargetCamera`].
///
/// The root node has [`Pickable::IGNORE`] so it does not block pointer clicks on underlying 3D/2D scenes
/// or lower UI layers, while child widgets inside it remain interactive.
///
/// Also available via [`crate::widgets::RuiBuilderExt::canvas_layer`].
pub fn spawn_canvas_layer<'a>(
    parent: &'a mut ChildSpawnerCommands,
    camera_entity: Entity,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut root_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Absolute,
        ..default()
    };
    
    modifier(&mut root_node);
    
    let mut cmds = parent.spawn((
        root_node,
        UiTargetCamera(camera_entity),
        Pickable::IGNORE,
    ));
    
    cmds.with_children(children);
    
    cmds
}
