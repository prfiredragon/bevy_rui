use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

#[derive(Component)]
struct SelectedDropdownLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_dropdown".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, sync_dropdown_label)
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
                    s.width = Val::Px(380.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(18.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                    s.overflow = Overflow::visible();
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Dropdown", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().dropdown(
                        "Guerrero",
                        &["Guerrero", "Mago", "Arquero", "Paladín", "Explorador"],
                        |s| {
                            s.width = Val::Px(240.0);
                            s.height = Val::Px(42.0);
                            s.padding = UiRect::horizontal(Val::Px(12.0));
                            s.border = UiRect::all(Val::Px(1.0));
                        },
                    );

                    c.reborrow()
                        .label("Clase seleccionada: Guerrero", |font, color| {
                            font.font_size = FontSize::Px(18.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(SelectedDropdownLabel);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}

fn sync_dropdown_label(
    q_dropdown: Query<&RuiDropdown>,
    q_texts: Query<&Text, (Changed<Text>, Without<SelectedDropdownLabel>)>,
    mut q_label: Query<&mut Text, With<SelectedDropdownLabel>>,
) {
    for dropdown in &q_dropdown {
        if let Ok(selected_txt) = q_texts.get(dropdown.text_entity) {
            if let Ok(mut label) = q_label.single_mut() {
                label.0 = format!("Clase seleccionada: {}", selected_txt.0);
            }
        }
    }
}
