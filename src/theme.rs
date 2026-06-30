use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

/// Componente para identificar el rol de un nodo de UI dentro del tema global.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum RuiThemeElement {
    Panel,
    Window,
    Button,
    ListItem,
    WindowHeader,
    TextboxBg,
    CheckboxBg,
    CheckboxIcon,
    DropdownBg,
    SliderTrack,
    SliderHandle,
    Text,
    ButtonText,
    WindowHeaderText,
    ScrollbarTrack,
    ScrollbarThumb,
    Tab,
    TabActive,
    ProgressBarTrack,
    ProgressBarFill,
}

/// Recurso global que define los estilos (fuentes, colores, y texturas/ninepatch).
#[derive(Resource, Clone)]
pub struct RuiTheme {
    // Fonts
    pub font: Handle<Font>,
    pub font_emoji: Option<Handle<Font>>,

    // Colors
    pub color_text: Color,
    pub color_panel: Color,
    pub color_window_header: Color,
    pub color_window_header_text: Color,
    
    pub color_button_normal: Color,
    pub color_button_hover: Color,
    pub color_button_pressed: Color,
    pub color_button_text: Color,

    pub color_list_item_normal: Color,
    pub color_list_item_hover: Color,
    pub color_list_item_pressed: Color,

    pub color_textbox_bg: Color,

    pub border_button: UiRect,
    pub border_color_button: Color,
    pub border_radius_button: BorderRadius,

    pub border_list_item: UiRect,
    pub border_color_list_item: Color,
    pub border_radius_list_item: BorderRadius,

    pub border_window: UiRect,
    pub border_color_window: Color,
    pub border_radius_window: BorderRadius,

    pub border_panel: UiRect,
    pub border_color_panel: Color,
    pub border_radius_panel: BorderRadius,

    pub border_textbox: UiRect,
    pub border_color_textbox: Color,
    pub border_radius_textbox: BorderRadius,

    // Textures (Ninepatch)
    pub image_panel: Option<Handle<Image>>,
    pub panel_margin: f32,

    pub image_window_header: Option<Handle<Image>>,
    pub window_header_margin: f32,
    pub window_header_layout_margin: UiRect,

    pub image_button_normal: Option<Handle<Image>>,
    pub image_button_hover: Option<Handle<Image>>,
    pub image_button_pressed: Option<Handle<Image>>,
    pub button_margin: f32,

    pub image_textbox_bg: Option<Handle<Image>>,
    pub textbox_margin: f32,
    
    pub image_checkbox_checked: Option<Handle<Image>>,
    pub image_checkbox_unchecked: Option<Handle<Image>>,
    
    pub image_slider_track: Option<Handle<Image>>,
    pub slider_track_margin: f32,
    pub image_slider_handle: Option<Handle<Image>>,
    
    pub image_scrollbar_track: Option<Handle<Image>>,
    pub scrollbar_track_margin: f32,
    pub image_scrollbar_thumb: Option<Handle<Image>>,
    pub scrollbar_thumb_margin: f32,
    
    pub image_list_item_normal: Option<Handle<Image>>,
    pub image_list_item_hover: Option<Handle<Image>>,
    pub image_list_item_pressed: Option<Handle<Image>>,
    pub list_item_margin: f32,
    
    pub image_tab_normal: Option<Handle<Image>>,
    pub image_tab_hover: Option<Handle<Image>>,
    pub image_tab_active: Option<Handle<Image>>,
    pub tab_margin: f32,
    
    pub image_progress_bar_track: Option<Handle<Image>>,
    pub progress_bar_track_margin: f32,
    pub image_progress_bar_fill: Option<Handle<Image>>,
    pub progress_bar_fill_margin: f32,
}

impl FromWorld for RuiTheme {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            font: asset_server.load("fonts/afacad.ttf"),
            font_emoji: None,
            
            color_text: Color::WHITE,
            color_panel: Color::srgb(0.15, 0.15, 0.15),
            color_window_header: Color::srgb(0.1, 0.1, 0.25),
            color_window_header_text: Color::WHITE,
            
            color_button_normal: Color::srgb(0.2, 0.2, 0.2),
            color_button_hover: Color::srgb(0.3, 0.3, 0.3),
            color_button_pressed: Color::srgb(0.1, 0.1, 0.1),
            color_button_text: Color::WHITE,

            color_list_item_normal: Color::NONE,
            color_list_item_hover: Color::srgb(0.3, 0.3, 0.3),
            color_list_item_pressed: Color::srgb(0.1, 0.1, 0.1),

            color_textbox_bg: Color::srgb(0.1, 0.1, 0.1),

            border_button: UiRect::all(Val::Px(0.0)),
            border_color_button: Color::NONE,
            border_radius_button: BorderRadius::all(Val::Px(0.0)),

            border_list_item: UiRect::all(Val::Px(0.0)),
            border_color_list_item: Color::NONE,
            border_radius_list_item: BorderRadius::all(Val::Px(0.0)),

            border_window: UiRect::all(Val::Px(0.0)),
            border_color_window: Color::NONE,
            border_radius_window: BorderRadius::all(Val::Px(0.0)),

            border_panel: UiRect::all(Val::Px(0.0)),
            border_color_panel: Color::NONE,
            border_radius_panel: BorderRadius::all(Val::Px(0.0)),

            border_textbox: UiRect::all(Val::Px(0.0)),
            border_color_textbox: Color::NONE,
            border_radius_textbox: BorderRadius::all(Val::Px(0.0)),

            image_panel: None,
            panel_margin: 0.0,

            image_window_header: None,
            window_header_margin: 0.0,
            window_header_layout_margin: UiRect::all(Val::Px(0.0)),

            image_button_normal: None,
            image_button_hover: None,
            image_button_pressed: None,
            button_margin: 0.0,

            image_textbox_bg: None,
            textbox_margin: 0.0,
            
            image_checkbox_checked: None,
            image_checkbox_unchecked: None,
            
            image_slider_track: None,
            slider_track_margin: 0.0,
            image_slider_handle: None,
            
            image_scrollbar_track: None,
            scrollbar_track_margin: 0.0,
            image_scrollbar_thumb: None,
            scrollbar_thumb_margin: 0.0,
            
            image_list_item_normal: None,
            image_list_item_hover: None,
            image_list_item_pressed: None,
            list_item_margin: 0.0,
            
            image_tab_normal: None,
            image_tab_hover: None,
            image_tab_active: None,
            tab_margin: 0.0,
            
            image_progress_bar_track: None,
            progress_bar_track_margin: 0.0,
            image_progress_bar_fill: None,
            progress_bar_fill_margin: 0.0,
        }
    }
}

pub fn apply_rui_theme(
    mut commands: Commands,
    theme: Res<RuiTheme>,
    mut elements: Query<(
        Entity,
        &mut ImageNode,
        Option<&mut Node>,
        Option<&mut BorderColor>,
        Ref<RuiThemeElement>,
    )>,
    mut text_elements: Query<(&mut TextFont, &mut TextColor, Ref<RuiThemeElement>)>,
) {
    let theme_changed = theme.is_changed();

    let mut apply_to_image = |entity: Entity, mut image_node: Mut<ImageNode>, mut opt_node: Option<Mut<Node>>, mut opt_border: Option<Mut<BorderColor>>, element: &RuiThemeElement| {
        let (border_rect, border_color, border_radius) = match element {
            RuiThemeElement::Button => (theme.border_button, theme.border_color_button, theme.border_radius_button),
            RuiThemeElement::ListItem => (theme.border_list_item, theme.border_color_list_item, theme.border_radius_list_item),
            RuiThemeElement::Window => (theme.border_window, theme.border_color_window, theme.border_radius_window),
            RuiThemeElement::Panel => (theme.border_panel, theme.border_color_panel, theme.border_radius_panel),
            RuiThemeElement::TextboxBg => (theme.border_textbox, theme.border_color_textbox, theme.border_radius_textbox),
            _ => (UiRect::all(Val::Px(0.0)), Color::NONE, BorderRadius::all(Val::Px(0.0))),
        };

        if let Some(ref mut node) = opt_node {
            node.border = border_rect;
            node.border_radius = border_radius;
        }

        let has_border = border_rect.top != Val::Px(0.0) || border_rect.bottom != Val::Px(0.0) || border_rect.left != Val::Px(0.0) || border_rect.right != Val::Px(0.0);

        if has_border || border_color != Color::NONE {
            if let Some(ref mut b) = opt_border {
                *b.as_mut() = BorderColor::all(border_color);
            } else {
                commands.entity(entity).insert(BorderColor::all(border_color));
            }
        }

        match element {
            RuiThemeElement::Panel | RuiThemeElement::Window => {
                if let Some(ref img) = theme.image_panel {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                    if theme.panel_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::all(theme.panel_margin),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        });
                    } else {
                        image_node.image_mode = NodeImageMode::Stretch;
                    }
                } else {
                    image_node.image = Handle::default();
                    image_node.color = theme.color_panel;
                    image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::WindowHeader => {
                if let Some(ref img) = theme.image_window_header {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                    if theme.window_header_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::all(theme.window_header_margin),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        });
                    } else {
                        image_node.image_mode = NodeImageMode::Stretch;
                    }
                } else {
                    image_node.image = Handle::default();
                    image_node.color = theme.color_window_header;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
                if let Some(ref mut node) = opt_node {
                    node.margin = theme.window_header_layout_margin;
                }
            }
            RuiThemeElement::Button => {
                if let Some(ref img) = theme.image_button_normal {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                    if theme.button_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::all(theme.button_margin),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        });
                    } else {
                        image_node.image_mode = NodeImageMode::Stretch;
                    }
                } else {
                    image_node.image = Handle::default();
                    image_node.color = theme.color_button_normal;
                    image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::ListItem => {
                image_node.image = Handle::default();
                image_node.color = theme.color_list_item_normal;
                image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                image_node.image_mode = NodeImageMode::Stretch;
            }
            RuiThemeElement::TextboxBg => {
                if let Some(ref img) = theme.image_textbox_bg {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::PaddingBox;
                    if theme.textbox_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::all(theme.textbox_margin),
                            center_scale_mode: SliceScaleMode::Stretch,
                            sides_scale_mode: SliceScaleMode::Stretch,
                            max_corner_scale: 1.0,
                        });
                    } else {
                        image_node.image_mode = NodeImageMode::Stretch;
                    }
                } else {
                    image_node.image = Handle::default();
                    image_node.color = theme.color_textbox_bg;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::SliderTrack => {
                if let Some(ref img) = theme.image_slider_track {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    if theme.slider_track_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer { border: BorderRect::all(theme.slider_track_margin), center_scale_mode: SliceScaleMode::Stretch, sides_scale_mode: SliceScaleMode::Stretch, max_corner_scale: 1.0 });
                    } else { image_node.image_mode = NodeImageMode::Stretch; }
                }
            }
            RuiThemeElement::SliderHandle => {
                if let Some(ref img) = theme.image_slider_handle {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::ScrollbarTrack => {
                if let Some(ref img) = theme.image_scrollbar_track {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    if theme.scrollbar_track_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer { border: BorderRect::all(theme.scrollbar_track_margin), center_scale_mode: SliceScaleMode::Stretch, sides_scale_mode: SliceScaleMode::Stretch, max_corner_scale: 1.0 });
                    } else { image_node.image_mode = NodeImageMode::Stretch; }
                }
            }
            RuiThemeElement::ScrollbarThumb => {
                if let Some(ref img) = theme.image_scrollbar_thumb {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    if theme.scrollbar_thumb_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer { border: BorderRect::all(theme.scrollbar_thumb_margin), center_scale_mode: SliceScaleMode::Stretch, sides_scale_mode: SliceScaleMode::Stretch, max_corner_scale: 1.0 });
                    } else { image_node.image_mode = NodeImageMode::Stretch; }
                }
            }
            RuiThemeElement::ProgressBarTrack => {
                if let Some(ref img) = theme.image_progress_bar_track {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    if theme.progress_bar_track_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer { border: BorderRect::all(theme.progress_bar_track_margin), center_scale_mode: SliceScaleMode::Stretch, sides_scale_mode: SliceScaleMode::Stretch, max_corner_scale: 1.0 });
                    } else { image_node.image_mode = NodeImageMode::Stretch; }
                }
            }
            RuiThemeElement::ProgressBarFill => {
                if let Some(ref img) = theme.image_progress_bar_fill {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    if theme.progress_bar_fill_margin > 0.0 {
                        image_node.image_mode = NodeImageMode::Sliced(TextureSlicer { border: BorderRect::all(theme.progress_bar_fill_margin), center_scale_mode: SliceScaleMode::Stretch, sides_scale_mode: SliceScaleMode::Stretch, max_corner_scale: 1.0 });
                    } else { image_node.image_mode = NodeImageMode::Stretch; }
                }
            }
            RuiThemeElement::CheckboxBg => {
                if let Some(ref img) = theme.image_checkbox_unchecked {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::CheckboxIcon => {
                if let Some(ref img) = theme.image_checkbox_checked {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            _ => {}
        }
    };

    // Apply to images
    for (entity, image_node, opt_node, opt_border, element_ref) in &mut elements {
        if theme_changed || element_ref.is_added() {
            apply_to_image(entity, image_node, opt_node, opt_border, &*element_ref);
        }
    }

    // Apply to text fonts and colors
    let apply_to_text = |mut font: Mut<TextFont>, mut color: Mut<TextColor>, element: &RuiThemeElement| {
        if theme.font != Handle::default() {
            font.font = bevy::prelude::FontSource::Handle(theme.font.clone());
        }
        match element {
            RuiThemeElement::Text => color.0 = theme.color_text,
            RuiThemeElement::ButtonText => color.0 = theme.color_button_text,
            RuiThemeElement::WindowHeaderText => color.0 = theme.color_window_header_text,
            _ => {}
        }
    };

    for (text_font, text_color, element_ref) in &mut text_elements {
        if theme_changed || element_ref.is_added() {
            apply_to_text(text_font, text_color, &*element_ref);
        }
    }
}
