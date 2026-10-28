//! # bevy_rui
//!
//! A Retained UI (RUI) widget library for Bevy 0.20 built on top of Bevy's native UI and Feathers controls.
//!
//! ## Core Modules
//! - [`widgets`]: 21 ready-to-use UI widgets (`button`, `window`, `tabs`, `docks`, `code_editor`, `file_dialog`, `menu`, `dropdown`, `slider`, `scrollview`, `textbox`, `color_picker`, etc.) and builder traits ([`RuiBuilderExt`], [`RuiRootBuilderExt`]).
//! - [`theme`]: Centralized styling resource ([`RuiTheme`]) and element role markers ([`RuiThemeElement`]) supporting solid colors, borders, and 9-slice (`NodeImageMode::Sliced`) textures.
//! - [`focus`]: Window focus stack ([`RuiActiveScope`]) and focusable widget marker ([`Focusable`]).
//! - [`navigation`]: Keyboard and gamepad directional UI navigation ([`RuiNavigationPlugin`], [`RuiNavigationBindings`], [`RuiNavigationConfig`]).
//!
//! ## Quick Start
//! Register [`RuiPlugin`] in your Bevy [`App`] and use [`RuiRootBuilderExt::rui_root`] along with [`RuiBuilderExt`] methods inside systems.

pub mod focus;
pub mod navigation;
pub mod widgets;
pub mod theme;

use bevy::prelude::*;
pub use focus::*;
pub use navigation::*;
pub use crate::widgets::*;
pub use crate::theme::*;

/// Common imports for building UI with `bevy_rui`.
pub mod prelude {
    pub use crate::widgets::*;
    pub use crate::navigation::*;
    pub use crate::focus::*;
    pub use crate::theme::*;
    pub use crate::RuiPlugin;
}

/// Main plugin for `bevy_rui`.
///
/// Registers:
/// - `FeathersPlugins` and a default dark `UiTheme` (inserted before `FeathersPlugins` so `FeathersCorePlugin` does not initialize an empty default theme).
/// - [`focus::RuiFocusPlugin`] for focus and active window management.
/// - Automatic [`Pickable`] insertion observers for [`bevy::ui_widgets::Button`], [`focus::Focusable`], and [`theme::RuiThemeElement`].
/// - [`navigation::RuiNavigationPlugin`] for keyboard/gamepad UI navigation.
/// - [`widgets::RuiWidgets`] for all widget systems, resources, and messages.
pub struct RuiPlugin;

impl Plugin for RuiPlugin {
    fn build(&self, app: &mut App) {
        // Insert `UiTheme(create_dark_theme())` BEFORE `FeathersPlugins` so `FeathersCorePlugin`
        // does not initialize an empty `UiTheme::default()` (which renders controls with fallback magenta).
        app.insert_resource(bevy::feathers::theme::UiTheme(
            bevy::feathers::dark_theme::create_dark_theme(),
        ));
        if !app.is_plugin_added::<bevy::feathers::FeathersCorePlugin>() {
            app.add_plugins(bevy::feathers::FeathersPlugins);
        }

        // Focus management is independent of individual widgets
        app.add_plugins(focus::RuiFocusPlugin);
        // Compatibility with `UiPickingSettings { require_markers: true }`:
        // Automatically adds `Pickable::default()` to RUI elements without overriding an explicit `Pickable::IGNORE`.
        fn ensure_pickable<C: Component>(trigger: On<Add<C>>, mut commands: Commands) {
            if let Ok(mut e) = commands.get_entity(trigger.entity) {
                e.try_insert_if_new(Pickable::default());
            }
        }
        app.add_observer(ensure_pickable::<bevy::ui_widgets::Button>);
        app.add_observer(ensure_pickable::<focus::Focusable>);
        app.add_observer(ensure_pickable::<theme::RuiThemeElement>);
        app.add_plugins(navigation::RuiNavigationPlugin);
        app.add_systems(Update, widgets::resizer::handle_resizer_drag);
        app.add_plugins(widgets::RuiWidgets);
    }
}
