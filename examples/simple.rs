use bevy::prelude::*;
use bevy_simple_scroll_view::*;

const CLR_1: Color = Color::srgb(0.168, 0.168, 0.168);
const CLR_2: Color = Color::srgb(0.109, 0.109, 0.109);
const CLR_3: Color = Color::srgb(0.569, 0.592, 0.647);
const CLR_4: Color = Color::srgb(0.902, 0.4, 0.004);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollViewPlugin))
        .add_systems(Startup, prepare)
        .add_systems(Update, reset_scroll)
        .run();
}

fn prepare(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            BackgroundColor(CLR_1),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Percent(20.0),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|p| {
                for btn_action in [ScrollButton::MoveToTop, ScrollButton::MoveToBottom] {
                    p.spawn((
                        Node {
                            margin: UiRect::all(Val::Px(15.0)),
                            padding: UiRect::all(Val::Px(15.0)),
                            max_height: Val::Px(100.0),
                            border: UiRect::all(Val::Px(3.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CLR_2),
                        BorderColor(CLR_4),
                        Button,
                        btn_action,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new(format!("{:#?}", btn_action)),
                            TextFont {
                                font_size: 25.0,
                                ..default()
                            },
                            TextColor(CLR_4),
                        ));
                    });
                }
            });
            p.spawn((
                Node {
                    width: Val::Percent(80.0),
                    margin: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(CLR_2),
                ScrollView::default(),
            ))
            .with_children(|p| {
                p.spawn((
                    Node {
                        flex_direction: bevy::ui::FlexDirection::Column,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ScrollableContent::default(),
                ))
                .with_children(|scroll_area| {
                    for i in 0..21 {
                        scroll_area
                            .spawn((
                                Node {
                                    min_width: Val::Px(200.0),
                                    margin: UiRect::all(Val::Px(15.0)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(30.0)),
                                    ..default()
                                },
                                BorderColor(CLR_3),
                            ))
                            .with_children(|p| {
                                p.spawn((
                                    Text::new(format!("Nr {}", i)),
                                    TextFont {
                                        font_size: 25.0,
                                        ..default()
                                    },
                                    TextColor(CLR_3),
                                    TextLayout::new_with_justify(JustifyText::Center),
                                ));
                            });
                    }
                });
            });
        });
}

#[derive(Component, PartialEq, Debug, Clone, Copy)]
#[require(Button)]
enum ScrollButton {
    MoveToTop,
    MoveToBottom,
}

fn reset_scroll(
    q: Query<(&Interaction, &ScrollButton), Changed<Interaction>>,
    mut scrolls_q: Query<&mut ScrollableContent>,
) {
    let Ok(mut scroll) = scrolls_q.get_single_mut() else {
        return;
    };
    for (interaction, action) in q.iter() {
        if interaction != &Interaction::Pressed {
            continue;
        }
        match action {
            ScrollButton::MoveToTop => scroll.scroll_to_top(),
            ScrollButton::MoveToBottom => scroll.scroll_to_bottom(),
        }
    }
}
