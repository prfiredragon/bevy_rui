use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct SingleLineInput;

#[derive(Component)]
struct MultiLineInput;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_textbox".to_string(),
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
                    s.width = Val::Px(440.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(14.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: TextBox (Single & MultiLine)", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .textbox("Escribe tu nombre (una línea)...", |s, font, _| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(42.0);
                            font.font_size = FontSize::Px(18.0);
                        })
                        .insert(SingleLineInput);

                    c.reborrow()
                        .multiline_textbox("Escribe notas (múltiples líneas)...", |s, font, _| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(120.0);
                            font.font_size = FontSize::Px(16.0);
                        })
                        .insert(MultiLineInput);

                    c.reborrow()
                        .button(
                            |s| {
                                s.width = Val::Px(180.0);
                                s.height = Val::Px(40.0);
                                s.align_self = AlignSelf::Center;
                            },
                            |btn| {
                                btn.reborrow()
                                    .label("Limpiar Campos", |f, _| f.font_size = FontSize::Px(16.0))
                                    .insert(RuiThemeElement::ButtonText);
                            },
                        )
                        .observe(
                            |_: On<Activate>,
                             mut q_boxes: Query<
                                &mut RuiTextBox,
                                Or<(With<SingleLineInput>, With<MultiLineInput>)>,
                            >| {
                                for mut tb in &mut q_boxes {
                                    tb.text.clear();
                                    tb.cursor_index = 0;
                                }
                            },
                        );
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}
