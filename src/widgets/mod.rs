use bevy::prelude::*;

pub mod accordion;
pub mod button;
pub mod checkbox;
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
pub mod viewport;
pub mod docks;
pub mod slider;
pub mod color_picker;

pub use accordion::*;
pub use button::*;
pub use checkbox::*;
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

#[derive(Resource)]
pub struct RuiDefaultFont(pub Handle<Font>);

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

pub trait RuiBuilderExt {
    fn vbox(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn hbox(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn button(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn label(&mut self, text: &str, modifier: impl FnOnce(&mut TextFont, &mut TextColor)) -> EntityCommands<'_>;
    fn textbox(&mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'_>;
    fn color_picker(&mut self, initial_color: Color, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
    fn multiline_textbox(&mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'_>;
    fn scrollview(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn accordion(&mut self, title: &str, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn window(&mut self, title: &str, closable: bool, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands, Entity)) -> EntityCommands<'_>;
    fn dropdown(&mut self, default_value: &str, options: &[&str], modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
    fn checkbox(&mut self, label_text: &str, checked: bool, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
    fn menu_bar(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn menu_item(&mut self, label: &str, icon: Option<RuiIcon>, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
    fn submenu(&mut self, label: &str, icon: Option<RuiIcon>, depth: u32, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn tabs(&mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut RuiTabsBuilder)) -> EntityCommands<'_>;
    fn resizer(&mut self, dir: RuiResizerDir, min_size: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
    fn viewport(&mut self, camera_entity: Entity, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
    fn dock_split_horizontal(&mut self, left_width: Val, min_size: f32, modifier: impl FnOnce(&mut Node), left_children: impl FnOnce(&mut ChildSpawnerCommands), right_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn dock_split_vertical(&mut self, top_height: Val, min_size: f32, modifier: impl FnOnce(&mut Node), top_children: impl FnOnce(&mut ChildSpawnerCommands), bottom_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
    fn dock_panel(&mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut RuiTabsBuilder)) -> EntityCommands<'_>;
    fn slider(&mut self, min: f32, max: f32, value: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_>;
}

impl RuiBuilderExt for ChildSpawnerCommands<'_> {
    fn vbox(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        layout::spawn_vbox(self, modifier, children)
    }
    fn hbox(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        layout::spawn_hbox(self, modifier, children)
    }
    fn button(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        button::spawn_button(self, modifier, children)
    }
    fn label(&mut self, text: &str, modifier: impl FnOnce(&mut TextFont, &mut TextColor)) -> EntityCommands<'_> {
        label::spawn_label(self, text, modifier)
    }
    fn textbox(&mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'_> {
        crate::widgets::textbox::spawn_textbox(self, placeholder, modifier)
    }
    
    fn color_picker(&mut self, initial_color: Color, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        crate::widgets::color_picker::spawn_color_picker(self, initial_color, modifier)
    }
    fn multiline_textbox(&mut self, placeholder: &str, modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor)) -> EntityCommands<'_> {
        textbox::spawn_multiline_textbox(self, placeholder, modifier)
    }
    fn scrollview(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        scrollview::spawn_scrollview(self, modifier, children)
    }
    fn accordion(&mut self, title: &str, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        accordion::spawn_accordion(self, title, modifier, children)
    }
    fn window(&mut self, title: &str, closable: bool, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands, Entity)) -> EntityCommands<'_> {
        windows::spawn_window(self, title, closable, modifier, children)
    }
    fn dropdown(&mut self, default_value: &str, options: &[&str], modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        dropdown::spawn_dropdown(self, default_value, options, modifier)
    }
    fn checkbox(&mut self, label_text: &str, checked: bool, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        checkbox::spawn_checkbox(self, label_text, checked, modifier)
    }
    fn menu_bar(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        menu::spawn_menu_bar(self, modifier, children)
    }
    fn menu_item(&mut self, label: &str, icon: Option<RuiIcon>, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        menu::spawn_menu_item(self, label, icon, modifier)
    }
    fn submenu(&mut self, label: &str, icon: Option<RuiIcon>, depth: u32, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        menu::spawn_submenu(self, label, icon, depth, modifier, children)
    }
    fn tabs(&mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut RuiTabsBuilder)) -> EntityCommands<'_> {
        tabs::spawn_tabs(self, active_tab, modifier, build_tabs)
    }
    fn resizer(&mut self, dir: RuiResizerDir, min_size: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        resizer::spawn_resizer(self, dir, min_size, modifier)
    }
    fn viewport(&mut self, camera_entity: Entity, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        viewport::spawn_viewport(self, camera_entity, modifier)
    }
    fn dock_split_horizontal(&mut self, left_width: Val, min_size: f32, modifier: impl FnOnce(&mut Node), left_children: impl FnOnce(&mut ChildSpawnerCommands), right_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        docks::spawn_dock_split_horizontal(self, left_width, min_size, modifier, left_children, right_children)
    }
    fn dock_split_vertical(&mut self, top_height: Val, min_size: f32, modifier: impl FnOnce(&mut Node), top_children: impl FnOnce(&mut ChildSpawnerCommands), bottom_children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_> {
        docks::spawn_dock_split_vertical(self, top_height, min_size, modifier, top_children, bottom_children)
    }
    fn dock_panel(&mut self, active_tab: usize, modifier: impl FnOnce(&mut Node), build_tabs: impl FnOnce(&mut RuiTabsBuilder)) -> EntityCommands<'_> {
        docks::spawn_dock_panel(self, active_tab, modifier, build_tabs)
    }
    fn slider(&mut self, min: f32, max: f32, value: f32, modifier: impl FnOnce(&mut Node)) -> EntityCommands<'_> {
        slider::spawn_slider(self, min, max, value, modifier)
    }
}

pub trait RuiRootBuilderExt {
    fn rui_root(&mut self, modifier: impl FnOnce(&mut Node), children: impl FnOnce(&mut ChildSpawnerCommands)) -> EntityCommands<'_>;
}

pub struct RuiWidgets;

impl Plugin for RuiWidgets {
    fn build(&self, app: &mut App) {
        app.init_non_send::<RuiClipboard>();
        app.init_resource::<crate::theme::RuiTheme>();
        app.add_systems(Startup, color_picker::setup_color_picker_images);
        
        app.add_systems(Update, (
            crate::theme::apply_rui_theme,
            handle_button_colors,
            handle_checkbox_clicks,
            handle_accordion_clicks,
            dropdown::update_dropdown_positions,
            handle_dropdown_clicks,
            close_dropdowns_on_outside_click,
            handle_menu_interactions,
            handle_tab_clicks,
            handle_tab_close_clicks,
            handle_resizer_drag,
            handle_resizer_collapse_clicks,
        ));

        app.add_systems(Update, (
            handle_window_drag,
            handle_window_close_clicks,
            handle_window_focus,
            handle_new_windows,
            slider::handle_slider_interaction,
            file_dialog::update_file_list_ui,
            file_dialog::handle_file_clicks,
            file_dialog::handle_dialog_buttons,
            file_dialog::handle_dropdown_change,
            color_picker::handle_color_picker_clicks,
            color_picker::close_color_picker_on_outside_click,
            color_picker::update_color_picker_popups,
            color_picker::sync_color_picker,
            color_picker::apply_color_picker_images,
            color_picker::handle_color_picker_2d_interaction,
        ));

        // Eventos
        app.add_message::<file_dialog::RuiFileSelected>();
        app.add_message::<file_dialog::RuiFileCanceled>();

        app.add_systems(Update, (
            handle_scrollview_scroll,
            handle_scrollview_clicks,
            update_scrollview_visuals,
        ));

        app.add_systems(Update, (
            handle_textbox_input,
            handle_textbox_scroll,
            update_textbox_visuals,
            handle_textbox_clicks.after(crate::focus::sync_mouse_to_focus),
        ));

        app.add_systems(Update, (
            apply_rui_default_font,
        ));

        app.add_systems(PostUpdate, (
            viewport::update_rui_viewports,
        ));
    }
}
