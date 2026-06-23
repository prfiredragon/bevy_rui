use bevy::prelude::*;
use crate::widgets::file_dialog::RuiFileItem;
pub fn debug_list_item_colors(q: Query<(Entity, &ImageNode, Option<&crate::theme::RuiThemeElement>), With<RuiFileItem>>) {
    for (_e, _img, _theme) in &q {
        // println!("ListItem {:?} color: {:?} img: {:?} theme: {:?}", _e, _img.color, _img.image, _theme);
    }
}
