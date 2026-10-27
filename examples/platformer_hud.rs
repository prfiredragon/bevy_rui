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
        let mut root_ui = root.reborrow();
        
        // Creamos la capa Canvas exclusiva para la cámara UI
        root_ui.canvas_layer(ui_camera, |_|{}, |canvas| {
            let mut c = canvas.reborrow();
            
            // ==========================================
            // HUD: ARRIBA IZQUIERDA (Puntaje y Monedas)
            // ==========================================
            c.reborrow().hbox(|n| {
                n.position_type = PositionType::Absolute;
                n.left = Val::Px(30.0);
                n.top = Val::Px(30.0);
                n.width = Val::Auto;
                n.height = Val::Auto;
            }, |top_left| {
                let mut tl = top_left.reborrow();
                
                // Puntaje (Score)
                tl.reborrow().vbox(|n| {
                    n.margin = UiRect::right(Val::Px(30.0));
                    n.width = Val::Auto;
                    n.height = Val::Auto;
                }, |score| {
                    let mut s1 = score.reborrow();
                    s1.reborrow().label("MANGO - Player", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(28.0);
                        color.0 = Color::WHITE;
                    });
                    let mut s2 = score.reborrow();
                    s2.reborrow().label("000000", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });
                });
                
                // Monedas (Coins)
                tl.reborrow().hbox(|n| {
                    n.align_items = AlignItems::Center;
                    n.width = Val::Auto;
                    n.height = Val::Auto;
                }, |coins| {
                    let mut co = coins.reborrow();
                    // Círculo simulando la moneda de oro
                    co.reborrow().spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(24.0),
                            margin: UiRect::right(Val::Px(10.0)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        ImageNode::solid_color(Color::srgb(1.0, 0.8, 0.1)),
                    ));
                    
                    let mut co2 = coins.reborrow();
                    co2.reborrow().label("x 00", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });
                });
            });
            
            // ==========================================
            // HUD: ARRIBA CENTRO (Mundo / Nivel)
            // ==========================================
            c.reborrow().vbox(|n| {
                n.position_type = PositionType::Absolute;
                n.left = Val::Px(0.0);
                n.right = Val::Px(0.0);
                n.top = Val::Px(30.0);
                n.margin = UiRect::horizontal(Val::Auto);
                n.align_items = AlignItems::Center;
                n.width = Val::Auto;
                n.height = Val::Auto;
            }, |world| {
                let mut w = world.reborrow();
                w.reborrow().label("WORLD", |font, color| {
                    font.font_size = bevy::prelude::FontSize::Px(28.0);
                    color.0 = Color::WHITE;
                });
                let mut w2 = world.reborrow();
                w2.reborrow().label("1-1", |font, color| {
                    font.font_size = bevy::prelude::FontSize::Px(24.0);
                    color.0 = Color::WHITE;
                });
                
                // Progress Bar
                let mut w3 = world.reborrow();
                w3.reborrow().progress_bar(0.0, 100.0, 35.0, |n| {
                    n.width = Val::Px(150.0);
                    n.height = Val::Px(12.0);
                    n.margin = UiRect::top(Val::Px(10.0));
                });
            });
            
            // ==========================================
            // HUD: ARRIBA DERECHA (Tiempo y Settings)
            // ==========================================
            c.reborrow().hbox(|n| {
                n.position_type = PositionType::Absolute;
                n.right = Val::Px(30.0);
                n.top = Val::Px(30.0);
                n.align_items = AlignItems::Center;
                n.width = Val::Auto;
                n.height = Val::Auto;
            }, |top_right| {
                let mut tr = top_right.reborrow();
                
                // Tiempo
                tr.reborrow().vbox(|n| {
                    n.margin = UiRect::right(Val::Px(30.0));
                    n.align_items = AlignItems::Center;
                    n.width = Val::Auto;
                    n.height = Val::Auto;
                }, |time| {
                    let mut t1 = time.reborrow();
                    t1.reborrow().label("TIME", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(28.0);
                        color.0 = Color::WHITE;
                    });
                    let mut t2 = time.reborrow();
                    t2.reborrow().label("400", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });
                });
                
                // Botón de Settings
                let mut tr2 = top_right.reborrow();
                tr2.reborrow().button(|n| {
                    n.width = Val::Px(50.0);
                    n.height = Val::Px(50.0);
                    n.border_radius = BorderRadius::all(Val::Px(25.0));
                    n.margin = UiRect::top(Val::Px(10.0));
                }, |b| {
                    let mut btn = b.reborrow();
                    btn.reborrow().label("⚙", |font, color| {
                        font.font_size = bevy::prelude::FontSize::Px(30.0);
                        color.0 = Color::WHITE;
                    }).insert(RuiThemeElement::ButtonText);
                }).insert(RuiButtonStateColors {
                    normal: Color::srgb(0.8, 0.1, 0.2),
                    hovered: Color::srgb(0.9, 0.2, 0.3),
                    pressed: Color::srgb(0.6, 0.05, 0.1),
                });
            });
            
        });
    });
}
