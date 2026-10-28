//! Diagnostic helper system for inspecting [`RuiFileItem`] theme and image colors.

use bevy::prelude::*;
use crate::widgets::file_dialog::RuiFileItem;

/// Debug system for inspecting [`ImageNode`] and [`crate::theme::RuiThemeElement`] state on [`RuiFileItem`] entities.
pub fn debug_list_item_colors(q: Query<(Entity, &ImageNode, Option<&crate::theme::RuiThemeElement>), With<RuiFileItem>>) {
    for (_e, _img, _theme) in &q {
        // println!("ListItem {:?} color: {:?} img: {:?} theme: {:?}", _e, _img.color, _img.image, _theme);
    }
}
