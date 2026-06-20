use bevy::prelude::*;
use bevy::ecs::relationship::Relationship;
use bevy::input_focus::InputFocus;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;

/// Tracks the currently active window or scope to restrict directional navigation
#[derive(Resource, Default)]
pub struct RuiActiveScope {
    pub window_stack: Vec<Entity>,
}

impl RuiActiveScope {
    pub fn active_window(&self) -> Option<Entity> {
        self.window_stack.last().copied()
    }

    pub fn push_window(&mut self, window: Entity) {
        self.window_stack.retain(|&w| w != window);
        self.window_stack.push(window);
    }

    pub fn pop_window(&mut self) -> Option<Entity> {
        self.window_stack.pop()
    }

    pub fn remove_window(&mut self, window: Entity) {
        self.window_stack.retain(|&w| w != window);
    }

    pub fn set_active_window(&mut self, window: Entity) {
        self.push_window(window);
    }
}

/// Component marker to tell the system this widget can receive focus
#[derive(Component, Default)]
pub struct Focusable;

pub struct RuiFocusPlugin;

impl Plugin for RuiFocusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RuiActiveScope>();
        app.add_systems(Update, (
            sync_mouse_to_focus,
            cleanup_despawned_windows,
            manage_focus_scopes,
        ));
    }
}

pub fn is_in_active_window(
    entity: Entity,
    active_scope: &RuiActiveScope,
    parents: &Query<&ChildOf>,
    no_windows: bool,
) -> bool {
    if let Some(active_window) = active_scope.active_window() {
        let mut current = entity;
        loop {
            if current == active_window { return true; }
            if let Ok(parent) = parents.get(current) {
                current = parent.get();
            } else {
                return false;
            }
        }
    }
    no_windows
}

pub fn sync_mouse_to_focus(
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Focusable>)>,
    mut input_focus: ResMut<InputFocus>,
    active_scope: Res<RuiActiveScope>,
    parents: Query<&ChildOf>,
    windows: Query<Entity, With<crate::widgets::windows::RuiWindow>>,
    config: Res<crate::navigation::RuiNavigationConfig>,
) {
    let no_windows = windows.is_empty();
    for (entity, interaction) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Explicit click always steals focus
            if input_focus.0 != Some(entity) {
                input_focus.set(entity);
            }
        } else if *interaction == Interaction::Hovered {
            // Hover steals focus only if it's in the active window OR if gamepad navigation is disabled
            if !config.enabled || is_in_active_window(entity, &active_scope, &parents, no_windows) {
                if input_focus.0 != Some(entity) {
                    input_focus.set(entity);
                }
            }
        }
    }
}

pub fn cleanup_despawned_windows(
    mut active_scope: ResMut<RuiActiveScope>,
    query: Query<Entity>,
) {
    let mut i = 0;
    while i < active_scope.window_stack.len() {
        if query.get(active_scope.window_stack[i]).is_err() {
            active_scope.window_stack.remove(i);
        } else {
            i += 1;
        }
    }
}

pub fn manage_focus_scopes(
    mut commands: Commands,
    mut active_scope: ResMut<RuiActiveScope>,
    mut input_focus: ResMut<InputFocus>,
    focusable_query: Query<(Entity, Option<&AutoDirectionalNavigation>), With<Focusable>>,
    new_focusables: Query<Entity, Added<Focusable>>,
    parents: Query<&ChildOf>,
    windows: Query<Entity, With<crate::widgets::windows::RuiWindow>>,
) {
    if active_scope.active_window().is_none() {
        if let Some(win) = windows.iter().next() {
            active_scope.push_window(win);
        }
    }

    let scope_changed = active_scope.is_changed();
    if !scope_changed && new_focusables.is_empty() { return; }

    if scope_changed {
        input_focus.clear();
    }

    let no_windows = windows.is_empty();

    for (entity, auto_nav) in &focusable_query {
        let in_active = is_in_active_window(entity, &active_scope, &parents, no_windows);

        if in_active && auto_nav.is_none() {
            commands.entity(entity).insert(AutoDirectionalNavigation::default());
        } else if !in_active && auto_nav.is_some() {
            commands.entity(entity).remove::<AutoDirectionalNavigation>();
        }

        if in_active && scope_changed && input_focus.0.is_none() {
            input_focus.set(entity);
        }
    }
}