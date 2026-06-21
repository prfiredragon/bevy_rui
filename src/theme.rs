use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

/// Componente para identificar el rol de un nodo de UI dentro del tema global.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum RuiThemeElement {
    Panel,
    Button,
    WindowHeader,
    TextboxBg,
    CheckboxIcon,
    DropdownBg,
    SliderTrack,
    SliderHandle,
    Text,
    ButtonText,
    WindowHeaderText,
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

    pub color_textbox_bg: Color,

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

            color_textbox_bg: Color::srgb(0.1, 0.1, 0.1),

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
        }
    }
}

pub fn apply_rui_theme(
    theme: Res<RuiTheme>,
    mut elements: Query<(&mut ImageNode, Option<&mut Node>, Ref<RuiThemeElement>)>,
    mut text_elements: Query<(&mut TextFont, &mut TextColor, Ref<RuiThemeElement>)>,
) {
    let theme_changed = theme.is_changed();

    let apply_to_image = |mut image_node: Mut<ImageNode>, mut opt_node: Option<Mut<Node>>, element: &RuiThemeElement| {
        match element {
            RuiThemeElement::Panel => {
                if let Some(ref img) = theme.image_panel {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::BorderBox;
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
                    image_node.visual_box = bevy::ui::VisualBox::BorderBox;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::WindowHeader => {
                if let Some(ref img) = theme.image_window_header {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::BorderBox;
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
                    image_node.visual_box = bevy::ui::VisualBox::BorderBox;
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
                    image_node.visual_box = bevy::ui::VisualBox::BorderBox;
                    image_node.image_mode = NodeImageMode::Stretch;
                }
            }
            RuiThemeElement::TextboxBg => {
                if let Some(ref img) = theme.image_textbox_bg {
                    image_node.image = img.clone();
                    image_node.color = Color::WHITE;
                    image_node.visual_box = bevy::ui::VisualBox::BorderBox;
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
            _ => {}
        }
    };

    // Apply to images
    for (image_node, opt_node, element_ref) in &mut elements {
        if theme_changed || element_ref.is_added() {
            apply_to_image(image_node, opt_node, &*element_ref);
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
