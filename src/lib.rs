/* #[cfg(feature = "bevy-0-18-1")]
extern crate bevy_0181 as bevy;
 */


/* #[cfg(feature = "bevy-0-19-0")]
extern crate bevy_0190 as bevy; */

// pub mod builder;
pub mod focus;
pub mod navigation;
pub mod widgets;
pub mod theme;

use bevy::prelude::*;
pub use focus::*;
pub use navigation::*;
pub use crate::widgets::*;
pub use crate::theme::*;

pub mod prelude {
    pub use crate::widgets::*;
    pub use crate::navigation::*;
}
pub struct RuiPlugin;

impl Plugin for RuiPlugin {
    fn build(&self, app: &mut App) {
        // El foco es independiente de los widgets
        app.add_plugins(focus::RuiFocusPlugin);
        app.add_plugins(navigation::RuiNavigationPlugin);
        app.add_systems(Update, widgets::tabs::handle_tab_clicks)
            .add_systems(Update, widgets::resizer::handle_resizer_drag);
        // Los widgets ahora se encargan de sus propios sistemas y recursos
        app.add_plugins(widgets::RuiWidgets);
    }
}