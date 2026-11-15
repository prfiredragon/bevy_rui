use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct ClickCounterText(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_button".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.spawn(Camera2d);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.vbox(
                |s| {
                    s.width = Val::Px(360.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(16.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Button", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Px(220.0);
                                s.height = Val::Px(44.0);
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Sumar Clic (+1)", |f, _| f.font_size = FontSize::Px(18.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .observe(
                            |_: On<Activate>,
                             mut q_counter: Query<(&mut ClickCounterText, &mut Text)>| {
                                if let Ok((mut counter, mut text)) = q_counter.single_mut() {
                                    counter.0 += 1;
                                    text.0 = format!("Clics realizados: {}", counter.0);
                                }
                            },
                        );

                    c.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Px(220.0);
                                s.height = Val::Px(44.0);
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Reiniciar Contador", |f, _| f.font_size = FontSize::Px(18.0));
                            },
                        )
                        .insert(RuiButtonStateColors {
                            normal: Color::srgb(0.55, 0.2, 0.2),
                            hovered: Color::srgb(0.7, 0.25, 0.25),
                            pressed: Color::srgb(0.4, 0.12, 0.12),
                        })
                        .observe(
                            |_: On<Activate>,
                             mut q_counter: Query<(&mut ClickCounterText, &mut Text)>| {
                                if let Ok((mut counter, mut text)) = q_counter.single_mut() {
                                    counter.0 = 0;
                                    text.0 = "Clics realizados: 0".to_string();
                                }
                            },
                        );

                    c.reborrow()
                        .label("Clics realizados: 0", |font, color| {
                            font.font_size = FontSize::Px(18.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(ClickCounterText(0));
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
