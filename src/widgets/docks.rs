use bevy::prelude::*;
use crate::widgets::{RuiResizerDir, spawn_resizer};
use crate::widgets::tabs::{spawn_tabs, RuiTabsBuilder};

/// Spawns a horizontal split area (left and right panes with a resizer in between)
pub fn spawn_dock_split_horizontal<'a>(
    parent: &'a mut ChildSpawnerCommands,
    left_width: Val,
    min_size: f32,
    modifier: impl FnOnce(&mut Node),
    left_children: impl FnOnce(&mut ChildSpawnerCommands),
    right_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_grow: 1.0,
        ..default()
    };
    modifier(&mut s);
    let mut cmds = parent.spawn(s);

    cmds.with_children(|p| {
        // Left pane
        p.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: left_width,
                height: Val::Percent(100.0),
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(Color::BLACK)
        )).with_children(left_children);

        // Resizer
        spawn_resizer(p, RuiResizerDir::Horizontal, min_size, |_| {});

        // Right pane
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Auto,
            height: Val::Percent(100.0),
            flex_grow: 1.0,
            ..default()
        }).with_children(right_children);
    });

    cmds
}

/// Spawns a vertical split area (top and bottom panes with a resizer in between)
pub fn spawn_dock_split_vertical<'a>(
    parent: &'a mut ChildSpawnerCommands,
    top_height: Val,
    min_size: f32,
    modifier: impl FnOnce(&mut Node),
    top_children: impl FnOnce(&mut ChildSpawnerCommands),
    bottom_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_grow: 1.0,
        ..default()
    };
    modifier(&mut s);
    let mut cmds = parent.spawn(s);

    cmds.with_children(|p| {
        // Top pane
        p.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: top_height,
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(Color::BLACK)
        )).with_children(top_children);

        // Resizer
        spawn_resizer(p, RuiResizerDir::Vertical, min_size, |_| {});

        // Bottom pane
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_grow: 1.0,
            ..default()
        }).with_children(bottom_children);
    });

    cmds
}

/// Spawns a dock panel (which is currently a wrapper around tabs)
pub fn spawn_dock_panel<'a>(
    parent: &'a mut ChildSpawnerCommands,
    active_tab: usize,
    modifier: impl FnOnce(&mut Node),
    build_tabs: impl FnOnce(&mut RuiTabsBuilder),
) -> EntityCommands<'a> {
    spawn_tabs(parent, active_tab, |s| {
        s.width = Val::Percent(100.0);
        s.height = Val::Percent(100.0);
        s.flex_grow = 1.0;
        modifier(s);
    }, build_tabs)
}
