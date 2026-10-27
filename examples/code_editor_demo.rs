use bevy::prelude::*;
use bevy_rui::*;

fn main() {
    App::new()
        .insert_resource(UiPickingSettings {
            require_markers: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 1. Cargar y establecer la fuente global para toda la UI
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));

    // 2. Cámara UI estándar
    commands.spawn((
        Camera2d,
        UiPickingCamera,
    ));

    // 3. Código inicial de demostración (Texto crudo multi-línea)
    let codigo_inicial = r#"// ¡Bienvenido al Code Editor de Rui!
fn main() {
    let mensaje = "Hola desde Bevy 0.20 y Syntect";
    println!("{}", mensaje);
    
    for i in 1..=5 {
        println!("Renderizando línea coloreada: {}", i);
    }
}"#;

    // 4. Raíz que cubre el 100% de la ventana
    commands.rui_root(|_| {}, |ui| {
        let mut root_ui = ui.reborrow();
        
        // Contenedor vertical base (Toda la pantalla)
        root_ui.vbox(|s| {
            s.width = Val::Percent(100.0);
            s.height = Val::Percent(100.0);
        }, |vbox| {
            let mut vb = vbox.reborrow();
            
            // --- BARRA DE MENÚ SUPERIOR ---
            let mut bar_bind = vb.reborrow();
            bar_bind.menu_bar(|s| { s.flex_shrink = 0.0; }, |menu| {
                let mut m = menu.reborrow();
                
                let mut file_bind = m.reborrow();
                file_bind.submenu("Archivo", None, 0, |_| {}, |file_menu| {
                    let mut fm = file_menu.reborrow();
                    fm.reborrow().menu_item("Nuevo Archivo", None, |_| {});
                    fm.reborrow().menu_item("Abrir...", None, |_| {});
                    fm.reborrow().menu_item("Guardar", None, |_| {});
                });
                
                let mut edit_bind = m.reborrow();
                edit_bind.submenu("Editar", None, 0, |_| {}, |edit_menu| {
                    let mut em = edit_menu.reborrow();
                    em.reborrow().menu_item("Deshacer (Ctrl+Z)", None, |_| {});
                    em.reborrow().menu_item("Rehacer (Ctrl+Y)", None, |_| {});
                    em.reborrow().menu_item("Cortar (Ctrl+X)", None, |_| {});
                    em.reborrow().menu_item("Copiar (Ctrl+C)", None, |_| {});
                    em.reborrow().menu_item("Pegar (Ctrl+V)", None, |_| {});
                });

                let mut help_bind = m.reborrow();
                help_bind.menu_item("Ayuda", None, |_| {});
            });

            // --- FILA CENTRAL DE TRABAJO (Ocupa todo el espacio restante) ---
            let mut workspace_bind = vb.reborrow();
            workspace_bind.hbox(|s| { s.flex_grow = 1.0; }, |main_workspace| {
                let mut mw = main_workspace.reborrow();
                
                // Panel Lateral Izquierdo: Simulador de Explorador de Proyectos
                let mut sidebar_bind = mw.reborrow();
                sidebar_bind.vbox(|s| {
                    s.width = Val::Px(220.0);
                    s.height = Val::Percent(100.0);
                    s.padding = UiRect::all(Val::Px(12.0));
                    s.row_gap = Val::Px(6.0);
                }, |sidebar| {
                    let mut sb = sidebar.reborrow();
                    sb.reborrow().label("PROYECTO (RUST)", |font, color| {
                        font.font_size = FontSize::Px(14.0);
                        color.0 = Color::srgb(0.5, 0.5, 0.5);
                    });
                    
                    let mut sb_btn1 = sb.reborrow();
                    sb_btn1.reborrow().button(|s| { s.margin = UiRect::top(Val::Px(10.0)); }, |btn| {
                        btn.reborrow().label("📄 main.rs", |font, _| font.font_size = FontSize::Px(16.0));
                    });
                    
                    let mut sb_btn2 = sb.reborrow();
                    sb_btn2.reborrow().button(|_| {}, |btn| {
                        btn.reborrow().label("📄 Cargo.toml", |font, _| font.font_size = FontSize::Px(16.0));
                    });
                    
                    let mut sb_btn3 = sb.reborrow();
                    sb_btn3.reborrow().button(|_| {}, |btn| {
                        btn.reborrow().label("📄 README.md", |font, _| font.font_size = FontSize::Px(16.0));
                    });
                }).insert(BackgroundColor(Color::srgb(0.12, 0.12, 0.12)));

                // Widget Code Editor
                let mut editor_bind = mw.reborrow();
                editor_bind.code_editor(codigo_inicial, "rs", |s, font, _color| {
                    s.flex_grow = 1.0;
                    s.height = Val::Percent(100.0);
                    font.font = bevy::prelude::FontSource::Handle(asset_server.load("fonts/FiraMono-Medium.ttf"));
                    font.font_size = FontSize::Px(18.0);
                });

            });

            // --- BARRA DE ESTADO INFERIOR ---
            let mut status_bind = vb.reborrow();
            status_bind.hbox(|s| { 
                s.height = Val::Px(28.0); 
                s.width = Val::Percent(100.0); 
                s.flex_shrink = 0.0;
                s.padding = UiRect::horizontal(Val::Px(12.0));
                s.align_items = AlignItems::Center;
            }, |status_bar| {
                let mut stb = status_bar.reborrow();
                stb.reborrow().label("Línea 1, Col 1  |  Espacios: 4  |  UTF-8  |  Sintaxis: Rust (rs)", |font, color| {
                    font.font_size = FontSize::Px(13.0);
                    color.0 = Color::srgb(0.6, 0.6, 0.6);
                });
            }).insert(BackgroundColor(Color::srgb(0.06, 0.06, 0.06)));

        });
    });
}
