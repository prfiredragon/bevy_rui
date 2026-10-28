use bevy::prelude::*;
use bevy::feathers::controls::{
    color_input_self_update, ColorInputValue, FeathersColorInput, FeathersColorInputProps,
};
use bevy::ui_widgets::ValueChange;

#[derive(Component)]
pub struct RuiColorPicker {
    pub color: Color,
}

impl Default for RuiColorPicker {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
        }
    }
}

pub fn setup_color_picker_images() {}
pub fn apply_color_picker_images() {}
pub fn close_color_picker_on_outside_click() {}
pub fn update_color_picker_popups() {}
pub fn handle_color_picker_2d_interaction() {}

pub fn spawn_color_picker<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    initial_color: Color,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'w> {
    let mut s = Node::default();
    modifier(&mut s);

    let mut cmds = parent_cmd.spawn_empty();
    cmds.apply_scene(FeathersColorInput::scene(FeathersColorInputProps::default()));
    cmds.insert((
        ColorInputValue(initial_color),
        RuiColorPicker {
            color: initial_color,
        },
        crate::focus::Focusable,
        Pickable::default(),
    ));
    // Parcheamos únicamente las propiedades de Node modificadas por el caller (ej. margin, width, height)
    cmds.entry::<Node>().and_modify(move |mut node| {
        if s.width != Val::Auto {
            node.width = s.width;
        }
        if s.height != Val::Auto {
            node.height = s.height;
        }
        if s.margin != UiRect::default() {
            node.margin = s.margin;
        }
        if s.padding != UiRect::default() {
            node.padding = s.padding;
        }
        if s.position_type != PositionType::Relative {
            node.position_type = s.position_type;
        }
        if s.left != Val::Auto {
            node.left = s.left;
        }
        if s.right != Val::Auto {
            node.right = s.right;
        }
        if s.top != Val::Auto {
            node.top = s.top;
        }
        if s.bottom != Val::Auto {
            node.bottom = s.bottom;
        }
        if s.flex_grow != 0.0 {
            node.flex_grow = s.flex_grow;
        }
        if s.flex_shrink != 1.0 {
            node.flex_shrink = s.flex_shrink;
        }
    });

    // Observer nativo de Bevy para actualizar ColorInputValue automáticamente al elegir un color
    cmds.observe(color_input_self_update);

    // Sincronizamos el evento ValueChange<Color> inmediatamente hacia RuiColorPicker
    cmds.observe(
        |change: On<ValueChange<Color>>, mut q_picker: Query<&mut RuiColorPicker>| {
            if let Ok(mut picker) = q_picker.get_mut(change.source) {
                picker.color = change.value;
            }
        },
    );

    cmds
}

/// Sincronización bidireccional entre `RuiColorPicker` y `ColorInputValue` (componente inmutable) de Bevy 0.20.
pub fn sync_color_picker(
    mut commands: Commands,
    mut q_pickers: Query<(Entity, &mut RuiColorPicker, Ref<ColorInputValue>)>,
) {
    for (entity, mut rui_picker, input_val) in &mut q_pickers {
        if input_val.is_changed() && !input_val.is_added() && rui_picker.color != input_val.0 {
            rui_picker.color = input_val.0;
        } else if rui_picker.is_changed() && !rui_picker.is_added() && input_val.0 != rui_picker.color {
            commands.entity(entity).insert(ColorInputValue(rui_picker.color));
        }
    }
}
