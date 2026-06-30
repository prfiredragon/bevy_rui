use bevy::prelude::*;
use bevy_rui::{RuiPlugin, widgets::RuiBuilderExt, widgets::button::RuiButtonStateColors, theme::RuiThemeElement};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Cámara UI que dibuja por encima del juego
    let ui_camera = commands.spawn((
        Camera2d,
        Camera {
            order: 1, 
            clear_color: ClearColorConfig::None,
            ..default()
        },
    )).id();
    
    // Cámara de "Juego" 3D/2D (Solo como fondo de demostración)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera { order: 0, ..default() },
    ));
    
    // Color de cielo para simular el fondo de nivel
    commands.insert_resource(ClearColor(Color::srgb(0.4, 0.6, 1.0)));

    // Contenedor UI global
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }).with_children(|root| {
        
        // Creamos la capa Canvas exclusiva para la cámara UI
        root.canvas_layer(ui_camera, |_|{}, |canvas| {
            
            // ==========================================
            // HUD: ARRIBA IZQUIERDA (Puntaje y Monedas)
            // ==========================================
            canvas.hbox(|n| {
                n.position_type = PositionType::Absolute;
                n.left = Val::Px(30.0);
                n.top = Val::Px(30.0);
                n.width = Val::Auto;
                n.height = Val::Auto;
            }, |top_left| {
                
                // Puntaje (Score)
                top_left.vbox(|n| {
                    n.margin = UiRect::right(Val::Px(30.0));
                    n.width = Val::Auto;
                    n.height = Val::Auto;
                }, |score| {
                    score.label("MARIO", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(28.0);
                        color.0 = Color::WHITE;
                    });
                    score.label("000000", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });
                });
                
                // Monedas (Coins)
                top_left.hbox(|n| {
                    n.align_items = AlignItems::Center;
                    n.width = Val::Auto;
                    n.height = Val::Auto;
                }, |coins| {
                    // Círculo simulando la moneda de oro
                    coins.spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(24.0),
                            margin: UiRect::right(Val::Px(10.0)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        ImageNode::solid_color(Color::srgb(1.0, 0.8, 0.1)), // Amarillo/Dorado
                    ));
                    
                    coins.label("x 00", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });
                });
            });
            
            // ==========================================
            // HUD: ARRIBA CENTRO (Mundo / Nivel)
            // ==========================================
            canvas.vbox(|n| {
                n.position_type = PositionType::Absolute;
                // Anclamos al centro y centramos los textos
                n.left = Val::Px(0.0);
                n.right = Val::Px(0.0);
                n.top = Val::Px(30.0);
                n.margin = UiRect::horizontal(Val::Auto);
                n.align_items = AlignItems::Center;
                n.width = Val::Auto;
                n.height = Val::Auto;
            }, |world| {
                world.label("WORLD", |font, color| {
                    font.font_size = bevy::prelude::FontSize::Px(28.0);
                    color.0 = Color::WHITE;
                });
                world.label("1-1", |font, color| {
                    font.font_size = bevy::prelude::FontSize::Px(24.0);
                    color.0 = Color::WHITE;
                });
            });
            
            // ==========================================
            // HUD: ARRIBA DERECHA (Tiempo y Settings)
            // ==========================================
            canvas.hbox(|n| {
                n.position_type = PositionType::Absolute;
                n.right = Val::Px(30.0);
                n.top = Val::Px(30.0);
                n.align_items = AlignItems::Center;
                n.width = Val::Auto;
                n.height = Val::Auto;
            }, |top_right| {
                
                // Tiempo
                top_right.vbox(|n| {
                    n.margin = UiRect::right(Val::Px(30.0));
                    n.align_items = AlignItems::Center;
                    n.width = Val::Auto;
                    n.height = Val::Auto;
                }, |time| {
                    time.label("TIME", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(28.0);
                        color.0 = Color::WHITE;
                    });
                    time.label("400", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });
                });
                
                // Botón de Settings
                top_right.button(|n| {
                    n.width = Val::Px(50.0);
                    n.height = Val::Px(50.0);
                    n.border_radius = BorderRadius::all(Val::Px(25.0)); // Totalmente redondo
                    n.margin = UiRect::top(Val::Px(10.0));
                }, |b| {
                    b.label("⚙", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(30.0);
                        color.0 = Color::WHITE;
                    }).insert(RuiThemeElement::ButtonText);
                }).insert(RuiButtonStateColors {
                    // Colores temáticos llamativos
                    normal: Color::srgb(0.8, 0.1, 0.2),  // Rojo intenso (Mario)
                    hovered: Color::srgb(0.9, 0.2, 0.3),
                    pressed: Color::srgb(0.6, 0.05, 0.1),
                });
            });
            
        });
    });
}
