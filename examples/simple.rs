use bevy::prelude::*;
use bevy_simple_scroll_view::*;

const CLR_1: Color = Color::rgb(0.168, 0.168, 0.168);
const CLR_2: Color = Color::rgb(0.109, 0.109, 0.109);
const CLR_3: Color = Color::rgb(0.569, 0.592, 0.647);
const CLR_4: Color = Color::rgb(0.902, 0.4, 0.004);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollViewPlugin))
        .add_systems(Startup, prepare)
        .add_systems(Update, reset_scroll)
        .run();
}

fn prepare(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: CLR_1.into(),
            ..default()
        })
        .with_children(|p| {
            p.spawn(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(15.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    max_height: Val::Px(100.0),
                    border: UiRect::all(Val::Px(3.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: CLR_2.into(),
                border_color: CLR_4.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Reset scroll",
                    TextStyle {
                        font_size: 25.0,
                        color: CLR_4,
                        ..default()
                    },
                ));
            });
            p.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: CLR_2.into(),
                    ..default()
                },
                ScrollView::default(),
            ))
            .with_children(|p| {
                p.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: bevy::ui::FlexDirection::Column,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    ScrollableContent::default(),
                ))
                .with_children(|scroll_area| {
                    for i in 0..21 {
                        scroll_area
                            .spawn(NodeBundle {
                                style: Style {
                                    min_width: Val::Px(200.0),
                                    margin: UiRect::all(Val::Px(15.0)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(30.0)),
                                    ..default()
                                },
                                border_color: CLR_3.into(),
                                ..default()
                            })
                            .with_children(|p| {
                                p.spawn(
                                    TextBundle::from_section(
                                        format!("Nr {}", i),
                                        TextStyle {
                                            font_size: 25.0,
                                            color: CLR_3,
                                            ..default()
                                        },
                                    )
                                    .with_text_justify(JustifyText::Center),
                                );
                            });
                    }
                });
            });
        });
}

fn reset_scroll(
    q: Query<(&Button, &Interaction), Changed<Interaction>>,
    mut scrolls_q: Query<&mut ScrollableContent>,
) {
    for (_, interaction) in q.iter() {
        if interaction == &Interaction::Pressed {
            for mut scroll in scrolls_q.iter_mut() {
                scroll.pos_y = 0.0;
            }
        }
    }
}
