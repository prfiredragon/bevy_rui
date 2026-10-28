//! Flex layout containers (`vbox`, `hbox`, and `rui_root`).

use bevy::prelude::*;
use crate::widgets::RuiRootBuilderExt;

/// Spawns a vertical flex container (`FlexDirection::Column`, `100% x 100%`).
///
/// Also available via [`crate::widgets::RuiBuilderExt::vbox`].
pub fn spawn_vbox<'a>(
    parent: &'a mut ChildSpawnerCommands,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        min_width: Val::Px(0.0),
        min_height: Val::Px(0.0),
        ..default()
    };
    modifier(&mut s);
    let mut cmds = parent.spawn(s);
    cmds.with_children(children);
    cmds
}

/// Spawns a horizontal flex container (`FlexDirection::Row`, `100% x 100%`).
///
/// Also available via [`crate::widgets::RuiBuilderExt::hbox`].
pub fn spawn_hbox<'a>(
    parent: &'a mut ChildSpawnerCommands,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        min_width: Val::Px(0.0),
        min_height: Val::Px(0.0),
        ..default()
    };
    modifier(&mut s);
    let mut cmds = parent.spawn(s);
    cmds.with_children(children);
    cmds
}

impl RuiRootBuilderExt for Commands<'_, '_> {
    fn rui_root(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        let mut s = Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            min_width: Val::Px(0.0),
            min_height: Val::Px(0.0),
            ..default()
        };
        modifier(&mut s);
        let mut cmds = self.spawn(s);
        cmds.with_children(children);
        cmds
    }
}