//! UI widgets and ergonomic builder extension traits for `bevy_rui`.
//!
//! Includes [`RuiBuilderExt`] (implemented on [`ChildSpawnerCommands`]) and [`RuiRootBuilderExt`]
//! (implemented on [`Commands`]) for building widget hierarchies.

use bevy::prelude::*;

pub mod accordion;
pub mod button;
pub mod checkbox;
#[cfg(not(target_arch = "wasm32"))]
pub mod clipboard;
pub mod dropdown;
pub mod file_dialog;
pub mod label;
pub mod layout;
pub mod menu;
pub mod resizer;
pub mod scrollview;
pub mod textbox;
pub mod tabs;
pub mod tooltip;
pub mod windows;
pub mod debug_colors;
pub mod viewport;
pub mod canvas_layer;
pub mod docks;
pub mod slider;
pub mod color_picker;
pub mod code_editor;
pub mod progress_bar;
pub mod context_menu;

pub use accordion::*;
pub use button::*;
pub use checkbox::*;
#[cfg(not(target_arch = "wasm32"))]
pub use clipboard::*;
pub use dropdown::*;
pub use label::*;
pub use layout::*;
pub use menu::*;
pub use resizer::*;
pub use scrollview::*;
pub use textbox::*;
pub use tabs::*;
pub use tooltip::*;
pub use windows::*;
pub use viewport::*;
pub use docks::*;
pub use color_picker::*;
pub use code_editor::*;
pub use progress_bar::*;
pub use context_menu::*;

/// Optional resource holding a fallback font handle applied to newly spawned [`TextFont`] components.
#[derive(Resource)]
pub struct RuiDefaultFont(pub Handle<Font>);

/// System that applies [`RuiDefaultFont`] to newly added [`TextFont`] components that still use the default handle.
pub fn apply_rui_default_font(
    mut query: Query<&mut TextFont, Added<TextFont>>,
    default_font: Option<Res<RuiDefaultFont>>,
) {
    if let Some(font_res) = default_font {
        for mut text_font in &mut query {
            if text_font.font == bevy::prelude::FontSource::Handle(Handle::default()) {
                text_font.font = bevy::prelude::FontSource::Handle(font_res.0.clone());
            }
        }
    }
}

/// Extension trait on [`ChildSpawnerCommands`] providing ergonomic methods to spawn `bevy_rui` widgets.
pub trait RuiBuilderExt<'w> {
    /// Spawns a vertical flex container (`FlexDirection::Column`). See [`layout::spawn_vbox`].
    fn vbox(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a horizontal flex container (`FlexDirection::Row`). See [`layout::spawn_hbox`].
    fn hbox(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns an interactive themed button. Attach `observe(|_: On<Activate>| ...)` to handle clicks. See [`button::spawn_button`].
    fn button(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a themed text label (`Pickable::IGNORE`). See [`label::spawn_label`].
    fn label(&'w mut self, text: &str, modifier: impl FnOnce(&mut TextFont, &mut TextColor)) -> EntityCommands<'w>;
    /// Spawns a single-line editable text box with placeholder text. See [`textbox::spawn_textbox`].
    fn textbox(&'w mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'w>;
    /// Spawns a color picker widget backed by `FeathersColorInput` and [`color_picker::RuiColorPicker`]. See [`color_picker::spawn_color_picker`].
    fn color_picker(&'w mut self, initial_color: Color, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a multi-line editable text box with placeholder text. See [`textbox::spawn_multiline_textbox`].
    fn multiline_textbox(&'w mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'w>;
    /// Spawns a scrollable container with automatic vertical and horizontal scrollbars. See [`scrollview::spawn_scrollview`].
    fn scrollview(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a collapsible accordion section with a clickable header and hidden-by-default content area. See [`accordion::spawn_accordion`].
    fn accordion(&'w mut self, title: &str, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a draggable floating window with a title bar and optional close button. See [`windows::spawn_window`].
    fn window(&'w mut self, title: &str, closable: bool, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands, Entity)) -> EntityCommands<'w>;
    /// Spawns a dropdown select menu with a list of string options. See [`dropdown::spawn_dropdown`].
    fn dropdown(&'w mut self, default_value: &str, options: &[&str], modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a toggleable checkbox with a text label. See [`checkbox::spawn_checkbox`].
    fn checkbox(&'w mut self, label_text: &str, checked: bool, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a horizontal top menu bar container. See [`menu::spawn_menu_bar`].
    fn menu_bar(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a clickable menu item with an optional [`menu::RuiIcon`]. See [`menu::spawn_menu_item`].
    fn menu_item(&'w mut self, label: &str, icon: Option<menu::RuiIcon>, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a nested submenu trigger and its floating popup container. See [`menu::spawn_submenu`].
    fn submenu(&'w mut self, label: &str, icon: Option<menu::RuiIcon>, depth: u32, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a tabbed container using [`tabs::RuiTabsBuilder`] to register individual tabs. See [`tabs::spawn_tabs`].
    fn tabs(&'w mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut tabs::RuiTabsBuilder)) -> EntityCommands<'w>;
    /// Spawns a draggable splitter bar between two sibling flex panes. See [`resizer::spawn_resizer`].
    fn resizer(&'w mut self, dir: resizer::RuiResizerDir, min_size: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a UI viewport node that synchronizes a camera's physical render rect with the node's layout bounds. See [`viewport::spawn_viewport`].
    fn viewport(&'w mut self, camera_entity: Entity, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a horizontally split dock area (left and right panes separated by a [`resizer::RuiResizer`]). See [`docks::spawn_dock_split_horizontal`].
    fn dock_split_horizontal(&'w mut self, left_width: Val, min_size: f32, modifier: impl FnOnce(&mut Node), left_children: impl FnOnce(&mut ChildSpawnerCommands), right_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a vertically split dock area (top and bottom panes separated by a [`resizer::RuiResizer`]). See [`docks::spawn_dock_split_vertical`].
    fn dock_split_vertical(&'w mut self, top_height: Val, min_size: f32, modifier: impl FnOnce(&mut Node), top_children: impl FnOnce(&mut ChildSpawnerCommands), bottom_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Spawns a tabbed dock panel (wrapper around [`tabs::spawn_tabs`] that fills 100% of its dock pane). See [`docks::spawn_dock_panel`].
    fn dock_panel(&'w mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut tabs::RuiTabsBuilder)) -> EntityCommands<'w>;
    /// Spawns an interactive horizontal numeric slider in `[min, max]`. See [`slider::spawn_slider`].
    fn slider(&'w mut self, min: f32, max: f32, value: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a horizontal progress bar indicator in `[min, max]`. See [`progress_bar::spawn_progress_bar`].
    fn progress_bar(&'w mut self, min: f32, max: f32, value: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
    /// Spawns a syntax-highlighted code editor widget powered by `ropey` and `syntect`. See [`code_editor::spawn_code_editor`].
    fn code_editor(&'w mut self, placeholder: &str, language: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'w>; 
    /// Spawns a full-screen HUD overlay targeted at a specific camera entity. See [`canvas_layer::spawn_canvas_layer`].
    fn canvas_layer(&'w mut self, camera_entity: Entity, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w>;
    /// Attaches a right-click context menu popup to the current parent entity. See [`context_menu::spawn_context_menu`].
    fn context_menu(&'w mut self, build_items: impl FnOnce(&mut ChildSpawnerCommands));
    /// Spawns a clickable item inside a context menu popup. See [`context_menu::spawn_context_menu_item`].
    fn context_menu_item(&'w mut self, label: &str, icon: Option<menu::RuiIcon>, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w>;
}

impl<'w> RuiBuilderExt<'w> for ChildSpawnerCommands<'w> {
    fn vbox(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { layout::spawn_vbox(self, modifier, children) }
    fn hbox(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { layout::spawn_hbox(self, modifier, children) }
    fn button(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { button::spawn_button(self, modifier, children) }
    fn label(&'w mut self, text: &str, modifier: impl FnOnce(&mut TextFont, &mut TextColor)) -> EntityCommands<'w> { label::spawn_label(self, text, modifier) }
    fn textbox(&'w mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'w> { crate::widgets::textbox::spawn_textbox(self, placeholder, modifier) }
    fn color_picker(&'w mut self, initial_color: Color, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { crate::widgets::color_picker::spawn_color_picker(self, initial_color, modifier) }
    fn multiline_textbox(&'w mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'w> { textbox::spawn_multiline_textbox(self, placeholder, modifier) }
    fn scrollview(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { scrollview::spawn_scrollview(self, modifier, children) }
    fn accordion(&'w mut self, title: &str, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { accordion::spawn_accordion(self, title, modifier, children) }
    fn window(&'w mut self, title: &str, closable: bool, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands, Entity)) -> EntityCommands<'w> { windows::spawn_window(self, title, closable, modifier, children) }
    fn dropdown(&'w mut self, default_value: &str, options: &[&str], modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { dropdown::spawn_dropdown(self, default_value, options, modifier) }
    fn checkbox(&'w mut self, label_text: &str, checked: bool, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { checkbox::spawn_checkbox(self, label_text, checked, modifier) }
    fn menu_bar(&'w mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { menu::spawn_menu_bar(self, modifier, children) }
    fn menu_item(&'w mut self, label: &str, icon: Option<menu::RuiIcon>, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { menu::spawn_menu_item(self, label, icon, modifier) }
    fn submenu(&'w mut self, label: &str, icon: Option<menu::RuiIcon>, depth: u32, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { menu::spawn_submenu(self, label, icon, depth, modifier, children) }
    fn tabs(&'w mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut tabs::RuiTabsBuilder)) -> EntityCommands<'w> { tabs::spawn_tabs(self, active_tab, modifier, build_tabs) }
    fn resizer(&'w mut self, dir: resizer::RuiResizerDir, min_size: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { resizer::spawn_resizer(self, dir, min_size, modifier) }
    fn viewport(&'w mut self, camera_entity: Entity, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { viewport::spawn_viewport(self, camera_entity, modifier) }
    fn canvas_layer(&'w mut self, camera_entity: Entity, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { canvas_layer::spawn_canvas_layer(self, camera_entity, modifier, children) }
    fn dock_split_horizontal(&'w mut self, left_width: Val, min_size: f32, modifier: impl FnOnce(&mut Node), left_children: impl FnOnce(&mut ChildSpawnerCommands), right_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { docks::spawn_dock_split_horizontal(self, left_width, min_size, modifier, left_children, right_children) }
    fn dock_split_vertical(&'w mut self, top_height: Val, min_size: f32, modifier: impl FnOnce(&mut Node), top_children: impl FnOnce(&mut ChildSpawnerCommands), bottom_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'w> { docks::spawn_dock_split_vertical(self, top_height, min_size, modifier, top_children, bottom_children) }
    fn dock_panel(&'w mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut tabs::RuiTabsBuilder)) -> EntityCommands<'w> { docks::spawn_dock_panel(self, active_tab, modifier, build_tabs) }
    fn slider(&'w mut self, min: f32, max: f32, value: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { slider::spawn_slider(self, min, max, value, modifier) }
    fn progress_bar(&'w mut self, min: f32, max: f32, value: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { progress_bar::spawn_progress_bar(self, min, max, value, modifier) }
    fn code_editor(&'w mut self, placeholder: &str, language: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'w> { code_editor::spawn_code_editor(self, placeholder, language, modifier) }
    fn context_menu(&'w mut self, build_items: impl FnOnce(&mut ChildSpawnerCommands)) { context_menu::spawn_context_menu(self, build_items); }
    fn context_menu_item(&'w mut self, label: &str, icon: Option<menu::RuiIcon>, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'w> { context_menu::spawn_context_menu_item(self, label, icon, modifier) }
}

/// Extension trait on [`Commands`] for spawning the top-level full-screen UI root container.
pub trait RuiRootBuilderExt {
    /// Spawns an absolute full-screen (`100% x 100%`) flex container as the root of a `bevy_rui` layout.
    fn rui_root(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
}

/// Plugin that initializes all `bevy_rui` widget resources, messages, and update systems.
pub struct RuiWidgets;

impl Plugin for RuiWidgets {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        app.init_non_send::<clipboard::RuiClipboard>();
        app.init_resource::<crate::theme::RuiTheme>();
        app.init_resource::<crate::widgets::code_editor::RuiSyntaxAssets>(); 
        app.add_systems(Startup, color_picker::setup_color_picker_images);
        
        app.add_systems(Update, (
            button::handle_button_colors,
            scrollview::handle_scrollview_clicks,
            dropdown::update_dropdown_positions,
            dropdown::close_dropdowns_on_outside_click,
            resizer::handle_resizer_drag,
            resizer::handle_resizer_collapse_clicks,
            code_editor::handle_code_editor_input,
            code_editor::update_code_editor_visuals,
            code_editor::handle_code_editor_clicks.after(crate::focus::sync_mouse_to_focus),
            context_menu::handle_context_menu_clicks,
            context_menu::close_context_menus_on_outside_click,
        ));

        app.add_systems(Update, (
            windows::handle_window_focus,
            windows::handle_new_windows,
            slider::handle_slider_navigation,
            slider::update_slider_visuals, 
            textbox::sync_native_textbox_buffers,
            file_dialog::update_file_list_ui,
            file_dialog::handle_file_clicks,
            file_dialog::handle_dialog_buttons,
            file_dialog::handle_dropdown_change,
            color_picker::close_color_picker_on_outside_click,
            color_picker::update_color_picker_popups,
            color_picker::sync_color_picker,
            color_picker::apply_color_picker_images,
            color_picker::handle_color_picker_2d_interaction,
        ));

        app.add_message::<file_dialog::RuiFileSelected>();
        app.add_message::<file_dialog::RuiFileCanceled>();

        app.add_systems(Update, (
            scrollview::handle_scrollview_scroll,
            scrollview::handle_scrollview_clicks,
            scrollview::update_scrollview_visuals,
        ));

        app.add_systems(Update, (
            textbox::handle_textbox_input,
            textbox::handle_textbox_scroll,
        ));

        app.add_systems(PostUpdate, (
            (crate::theme::apply_rui_theme, apply_rui_default_font).before(bevy::ui::UiSystems::Layout),
            viewport::update_rui_viewports,
        ));
    }
}

