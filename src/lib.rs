use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

/// A `Plugin` providing the systems and components required to make a ScrollView work.
pub struct ScrollViewPlugin;

impl Plugin for ScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ScrollView>()
            .register_type::<ScrollViewport>()
            .register_type::<ScrollViewContent>()
            .add_systems(
                Update,
                (create_scroll_view, input_mouse_pressed_move, scroll_events),
            );
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct ScrollView;

#[derive(Component, Default, Debug, Reflect)]
pub struct ScrollViewport;

#[derive(Component, Debug, Reflect)]
pub struct ScrollViewContent(pub Entity);

pub fn create_scroll_view(mut commands: Commands, q: Query<Entity, Added<ScrollView>>) {
    for e in q.iter() {
        commands.entity(e).with_children(|p| {
            p.spawn((
                NodeBundle {
                    style: Style {
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    ..Default::default()
                },
                ScrollViewport,
                Interaction::None,
            ))
            .with_children(|v| {
                v.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: bevy::ui::FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    ScrollViewContent(e),
                ));
            });
        });
    }
}

fn input_mouse_pressed_move(
    mut motion_evr: EventReader<MouseMotion>,
    mut q: Query<(Entity, &mut Style, &Interaction), With<ScrollViewport>>,
) {
    for evt in motion_evr.read() {
        info!("{:?}", evt);
        for (_e, mut style, &interaction) in q.iter_mut() {
            if interaction == Interaction::Pressed {
                style.top = match style.top {
                    Val::Px(px) => Val::Px(px + evt.delta.y),
                    _ => Val::Px(0.0),
                }
            }
        }
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(Entity, &mut Style, &Interaction), With<ScrollViewport>>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read() {
        let y = match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                ev.y
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                ev.y
            }
        };
        for (_e, mut style, &interaction) in q.iter_mut() {
            if interaction == Interaction::Hovered {
                style.top = match style.top {
                    Val::Px(px) => Val::Px(px + y),
                    _ => Val::Px(0.0),
                }
            }
        }
    }
}
