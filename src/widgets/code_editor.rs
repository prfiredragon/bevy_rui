use bevy::prelude::*;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

#[derive(Resource)]
pub struct RuiSyntaxAssets {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub default_theme: String,
}

impl Default for RuiSyntaxAssets {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            // Puedes usar "base16-ocean.dark", "base16-eighties.dark", "InspiredGitHub", etc.
            default_theme: "base16-ocean.dark".to_string(), 
        }
    }
}


use ropey::Rope;

#[derive(Clone, Default)]
pub struct EditorState {
    pub text: Rope,
    pub cursor_index: usize,
    pub selection: Option<(usize, usize)>,
}

#[derive(Component)]
pub struct RuiCodeEditor {
    pub text: Rope,
    pub language: String, // ej: "rs" para Rust, "py" para Python
    pub placeholder: String,
    pub cursor_timer: Timer,
    pub show_cursor: bool,
    pub cursor_index: usize,
    pub scroll_offset: Vec2,
    pub readonly: bool,
    pub selection: Option<(usize, usize)>,
    pub selection_anchor: Option<usize>,
    pub cursor_moved: bool,
    pub dragging_v_scroll: bool,
    pub dragging_h_scroll: bool,
    pub previous_cursor_index: usize,
    pub previous_text_len: usize,
    pub previous_focused: bool,
    pub undo_stack: Vec<EditorState>,
    pub redo_stack: Vec<EditorState>,
}

impl Default for RuiCodeEditor {
    fn default() -> Self {
        Self {
            text: Rope::new(),
            language: "rs".to_string(), // Por defecto Rust
            placeholder: String::new(),
            cursor_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            show_cursor: false,
            cursor_index: 0,
            scroll_offset: Vec2::ZERO,
            readonly: false,
            selection: None,
            selection_anchor: None,
            cursor_moved: false,
            dragging_v_scroll: false,
            dragging_h_scroll: false,
            previous_cursor_index: 0,
            previous_text_len: 0,
            previous_focused: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
}


use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

// Componentes marcadores específicos para el Code Editor
#[derive(Component)]
pub struct RuiCodeEditorGutter;

#[derive(Component)]
pub struct RuiCodeEditorText;

#[derive(Component)]
pub struct RuiCodeEditorSelection;

#[derive(Component)]
pub struct RuiCodeEditorCursor;



pub fn spawn_code_editor<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    placeholder: &str,
    language: &str,
    modifier: impl FnOnce(&mut Node, &mut TextFont, &mut TextColor),
) -> EntityCommands<'a> {
    // 1. Configuración del Contenedor Raíz (Row)
    let mut root_node = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        min_height: Val::Px(200.0), // Un code editor suele ser más grande
        min_width: Val::Px(300.0),
        border: UiRect::all(Val::Px(2.0)),
        overflow: Overflow::clip(), // Esencial para esconder lo que desborda
        ..default()
    };
    
    let mut font = TextFont::default();
    let mut color = TextColor(Color::WHITE);
    modifier(&mut root_node, &mut font, &mut color);

    let mut cmds = parent_cmd.spawn((
        root_node,
        Button,
        crate::focus::Focusable,
        bevy::ui::FocusPolicy::Block,
        Pickable::default(),
        ImageNode { 
            visual_box: bevy::ui::VisualBox::BorderBox, 
            image_mode: bevy::ui::widget::NodeImageMode::Stretch, 
            ..ImageNode::default() 
        },
        crate::theme::RuiThemeElement::TextboxBg, // Puedes crear un ThemeElement::CodeEditorBg luego
        BorderColor::all(Color::srgb(0.051, 0.051, 0.051)),
        RelativeCursorPosition::default(),
        RuiCodeEditor { 
            placeholder: placeholder.to_string(), 
            language: language.to_string(),
            ..default() 
        },
    ));

    cmds.with_children(|parent| {
        
        // --- PANEL IZQUIERDO: EL GUTTER ---
        parent.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Px(45.0), // Ancho inicial para los números
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd, // Alinea los números a la derecha
                padding: UiRect::right(Val::Px(8.0)), // Separación con el código
                overflow: Overflow::clip(),
                ..default()
            },
            // Color de fondo ligeramente más claro/oscuro para diferenciar el gutter
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)), 
        )).with_children(|gutter| {
            gutter.spawn((
                Text::new("1"), // Iniciamos con la línea 1
                font.clone(),
                TextColor(Color::srgb(0.4, 0.4, 0.4)), // Gris atenuado
                RuiCodeEditorGutter,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.0), // Mismo padding top que el área de código
                    ..default()
                },
                Pickable::IGNORE,
            ));
        });

        // --- PANEL DERECHO: ÁREA DE CÓDIGO ---
        parent.spawn((
            Node {
                display: Display::Flex,
                flex_grow: 1.0, // Toma todo el espacio restante
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                overflow: Overflow::clip(),
                ..default()
            },
        )).with_children(|editor_area| {
            // El Texto Principal (Donde inyectaremos los TextSpans)
            editor_area.spawn((
                Text::new(placeholder),
                font,
                color,
                RuiCodeEditorText,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    ..default()
                },
                Pickable::IGNORE,
            ));

            // Cursor
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(2.0),
                    height: Val::Px(20.0),
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    ..default()
                },
                ImageNode::solid_color(Color::WHITE),
                Visibility::Hidden,
                RuiCodeEditorCursor,
                Pickable::IGNORE,
            ));

            // Highlight de Selección
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    width: Val::Px(0.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                ImageNode::solid_color(Color::srgba(0.2, 0.4, 0.8, 0.5)),
                Visibility::Hidden,
                RuiCodeEditorSelection,
                Pickable::IGNORE,
            ));

            // Scrollbars (Reutilizando tus componentes)
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    width: Val::Px(6.0),
                    height: Val::Percent(0.0),
                    ..default()
                },
                ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                Visibility::Hidden,
                crate::widgets::scrollview::RuiVerticalScrollbar,
                Pickable::IGNORE,
            ));
            
            editor_area.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                    width: Val::Percent(0.0),
                    height: Val::Px(6.0),
                    ..default()
                },
                ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                Visibility::Hidden,
                crate::widgets::scrollview::RuiHorizontalScrollbar,
                Pickable::IGNORE,
            ));
        });
    });

    cmds
}









