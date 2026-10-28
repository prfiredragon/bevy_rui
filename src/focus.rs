//! Focus management and modal/window scope isolation for `bevy_rui`.
//!
//! Restricts directional navigation and hover focus to the currently active window
//! tracked in [`RuiActiveScope`].

use bevy::prelude::*;
use bevy::ecs::relationship::Relationship;
use bevy::input_focus::{InputFocus, FocusCause};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;

/// Resource tracking the stack of open windows/modals to restrict directional navigation and focus.
#[derive(Resource, Default)]
pub struct RuiActiveScope {
    /// Stack of window/popup entities, where the last element is the currently active scope.
    pub window_stack: Vec<Entity>,
}

impl RuiActiveScope {
    /// Returns the currently active (topmost) window entity, if any.
    pub fn active_window(&self) -> Option<Entity> {
        self.window_stack.last().copied()
    }

    /// Pushes a window entity to the top of the focus stack, deduplicating if already present.
    pub fn push_window(&mut self, window: Entity) {
        self.window_stack.retain(|&w| w != window);
        self.window_stack.push(window);
    }

    /// Pops and returns the topmost window entity from the focus stack.
    pub fn pop_window(&mut self) -> Option<Entity> {
        self.window_stack.pop()
    }

    /// Removes a specific window entity from anywhere in the focus stack.
    pub fn remove_window(&mut self, window: Entity) {
        self.window_stack.retain(|&w| w != window);
    }

    /// Marks the given window entity as the active topmost window.
    pub fn set_active_window(&mut self, window: Entity) {
        self.push_window(window);
    }
}

/// Component marker indicating that a UI widget can receive [`InputFocus`] and directional navigation.
#[derive(Component, Default)]
pub struct Focusable;

/// Plugin that initializes [`RuiActiveScope`] and runs focus synchronization systems.
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

/// Checks whether `entity` is a descendant of the currently active window in [`RuiActiveScope`].
/// If no windows exist (`no_windows == true`), returns `true`.
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

/// Synchronizes mouse [`bevy::ui::Pressed`] and [`bevy::picking::hover::Hovered`] states with [`InputFocus`].
pub fn sync_mouse_to_focus(
    interaction_query: Query<(Entity, Option<&bevy::ui::Pressed>, Option<&bevy::picking::hover::Hovered>), (With<Focusable>, Or<(Changed<bevy::ui::Pressed>, Changed<bevy::picking::hover::Hovered>)>)>,
    mut input_focus: ResMut<InputFocus>,
    active_scope: Res<RuiActiveScope>,
    parents: Query<&ChildOf>,
    windows: Query<Entity, With<crate::widgets::windows::RuiWindow>>,
    config: Res<crate::navigation::RuiNavigationConfig>,
) {
    let no_windows = windows.is_empty();
    for (entity, pressed, hovered) in &interaction_query {
        if pressed.is_some() {
            if input_focus.get() != Some(entity) {
                input_focus.set(entity, FocusCause::Navigated);
            }
        } else if let Some(h_state) = hovered {
            if h_state.0 {
                if !config.enabled || is_in_active_window(entity, &active_scope, &parents, no_windows) {
                    if input_focus.get() != Some(entity) {
                        input_focus.set(entity, FocusCause::Navigated);
                    }
                }
            }
        }
    }
}

/// Removes despawned window entities from [`RuiActiveScope::window_stack`].
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

/// Dynamically adds or removes [`AutoDirectionalNavigation`] on [`Focusable`] entities
/// so that keyboard/gamepad navigation stays confined to the active window scope.
pub fn manage_focus_scopes(
    mut commands: Commands,
    mut active_scope: ResMut<RuiActiveScope>,
    mut input_focus: ResMut<InputFocus>,
    focusable_query: Query<(Entity, Option<&AutoDirectionalNavigation>), (With<Focusable>, Without<crate::widgets::textbox::RuiTextBox>)>,
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
            if let Ok(mut e_cmds) = commands.get_entity(entity) {
                e_cmds.try_insert(AutoDirectionalNavigation::default());
            }
        } else if !in_active && auto_nav.is_some() {
            if let Ok(mut e_cmds) = commands.get_entity(entity) {
                e_cmds.remove::<AutoDirectionalNavigation>();
            }
        }

        if in_active && scope_changed && input_focus.get().is_none() {
            input_focus.set(entity, FocusCause::Navigated);
        }
    }
}
