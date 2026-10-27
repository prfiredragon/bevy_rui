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
        // Compatibilidad con `UiPickingSettings { require_markers: true }`:
        // con esa opción Bevy solo hace picking de nodos con `Pickable`. Se lo añadimos
        // automáticamente a los elementos de bevy_rui (sin pisar un `Pickable::IGNORE` explícito).
        fn ensure_pickable<C: Component>(trigger: On<Add<C>>, mut commands: Commands) {
            if let Ok(mut e) = commands.get_entity(trigger.entity) {
                e.try_insert_if_new(Pickable::default());
            }
        }
        app.add_observer(ensure_pickable::<bevy::ui_widgets::Button>);
        app.add_observer(ensure_pickable::<focus::Focusable>);
        app.add_observer(ensure_pickable::<theme::RuiThemeElement>);
        app.add_plugins(navigation::RuiNavigationPlugin);
        // Plugin nativo de Bevy 0.20 para MenuButton/MenuPopup/MenuItem
        //app.add_plugins(bevy::ui_widgets::MenuPlugin);
        app.add_systems(Update,  widgets::resizer::handle_resizer_drag);
        // Los widgets ahora se encargan de sus propios sistemas y recursos
        app.add_plugins(widgets::RuiWidgets);
    }
}
