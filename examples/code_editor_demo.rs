use bevy::prelude::*;
use bevy_rui::*;

fn main() {
    App::new()
        .insert_resource(UiPickingSettings {
            require_markers: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RuiPlugin) // Tu plugin principal de UI
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 1. Cargar y establecer la fuente global para toda la UI
    // NOTA: Para código se recomiendan fuentes monoespaciadas (como JetBrains Mono o Fira Code),
    // pero usaremos tu fuente por defecto establecida para mantener consistencia.
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));

    // 2. Cámara UI estándar de Bevy 0.15
    commands.spawn((
        Camera2d,
        UiPickingCamera,
    ));

    // 3. Código inicial de demostración (Texto crudo multi-línea)
    let codigo_inicial = r#"// ¡Bienvenido al Code Editor de Rui!
fn main() {
    let mensaje = "Hola desde Bevy 0.15 y Syntect";
    println!("{}", mensaje);
    
    for i in 1..=5 {
        println!("Renderizando línea coloreada: {}", i);
    }
}"#;

    // 4. Raíz que cubre el 100% de la ventana
    commands.rui_root(|_| {}, |ui| {
        
        // Contenedor vertical base (Toda la pantalla)
        ui.vbox(|s| {
            s.width = Val::Percent(100.0);
            s.height = Val::Percent(100.0);
        }, |vbox| {
            
            // --- BARRA DE MENÚ SUPERIOR ---
            vbox.menu_bar(|s| { s.flex_shrink = 0.0; }, |menu| {
                menu.submenu("Archivo", None, 0, |_| {}, |file_menu| {
                    file_menu.menu_item("Nuevo Archivo", None, |_| {});
                    file_menu.menu_item("Abrir...", None, |_| {});
                    file_menu.menu_item("Guardar", None, |_| {});
                });
                
                menu.submenu("Editar", None, 0, |_| {}, |edit_menu| {
                    edit_menu.menu_item("Deshacer (Ctrl+Z)", None, |_| {});
                    edit_menu.menu_item("Rehacer (Ctrl+Y)", None, |_| {});
                    edit_menu.menu_item("Cortar (Ctrl+X)", None, |_| {});
                    edit_menu.menu_item("Copiar (Ctrl+C)", None, |_| {});
                    edit_menu.menu_item("Pegar (Ctrl+V)", None, |_| {});
                });

                menu.menu_item("Ayuda", None, |_| {});
            });

            // --- FILA CENTRAL DE TRABAJO (Ocupa todo el espacio restante) ---
            vbox.hbox(|s| { s.flex_grow = 1.0; }, |main_workspace| {
                
                // Panel Lateral Izquierdo: Simulador de Explorador de Proyectos
                main_workspace.vbox(|s| {
                    s.width = Val::Px(220.0);
                    s.height = Val::Percent(100.0);
                    s.padding = UiRect::all(Val::Px(12.0));
                    s.row_gap = Val::Px(6.0);
                }, |sidebar| {
                    sidebar.label("PROYECTO (RUST)", |font, color| {
                        font.font_size = FontSize::Px(14.0);
                        color.0 = Color::srgb(0.5, 0.5, 0.5); // Gris apagado para títulos
                    });
                    
                    sidebar.button(|s| { s.margin = UiRect::top(Val::Px(10.0)); }, |btn| {
                        btn.label("📄 main.rs", |font, _| font.font_size = FontSize::Px(16.0));
                    });
                    
                    sidebar.button(|_| {}, |btn| {
                        btn.label("📄 Cargo.toml", |font, _| font.font_size = FontSize::Px(16.0));
                    });
                    
                    sidebar.button(|_| {}, |btn| {
                        btn.label("📄 README.md", |font, _| font.font_size = FontSize::Px(16.0));
                    });
                }).insert(BackgroundColor(Color::srgb(0.12, 0.12, 0.12))); // Fondo oscuro lateral

                // ¡EL PROTAGONISTA!: El nuevo Widget Code Editor
                // Le ordenamos expandirse horizontalmente para tomar todo el ancho restante y el 100% del alto
                main_workspace.code_editor(codigo_inicial, "rs", |s, font, _color| {
                    s.flex_grow = 1.0;
                    s.height = Val::Percent(100.0);
                    font.font_size = FontSize::Px(18.0); // Tamaño ideal para lectura de código
                });

            });

            // --- BARRA DE ESTADO INFERIOR ---
            vbox.hbox(|s| { 
                s.height = Val::Px(28.0); 
                s.width = Val::Percent(100.0); 
                s.flex_shrink = 0.0;
                s.padding = UiRect::horizontal(Val::Px(12.0));
                s.align_items = AlignItems::Center;
            }, |status_bar| {
                status_bar.label("Línea 1, Col 1  |  Espacios: 4  |  UTF-8  |  Sintaxis: Rust (rs)", |font, color| {
                    font.font_size = FontSize::Px(13.0);
                    color.0 = Color::srgb(0.6, 0.6, 0.6);
                });
            }).insert(BackgroundColor(Color::srgb(0.06, 0.06, 0.06))); // Fondo ultra oscuro inferior

        });
    });
}
