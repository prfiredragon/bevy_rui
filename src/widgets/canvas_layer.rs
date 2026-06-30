use bevy::prelude::*;

pub fn spawn_canvas_layer<'a>(
    parent: &'a mut ChildSpawnerCommands,
    camera_entity: Entity,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    
    let mut root_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Absolute, // To overlay across the entire screen
        ..default()
    };
    
    modifier(&mut root_node);
    
    let mut cmds = parent.spawn((
        root_node,
        UiTargetCamera(camera_entity),
        // By default we ignore clicks on the canvas layer itself so it doesn't block underlying interaction
        Pickable::IGNORE,
    ));
    
    cmds.with_children(children);
    
    cmds
}
