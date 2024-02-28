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

#[derive(Component, Debug, Reflect)]
pub struct ScrollViewport {
    pub scroll_speed: f32,
}

impl Default for ScrollViewport {
    fn default() -> Self {
        Self { scroll_speed: 500.0 }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct ScrollViewContent {
    pub pos_y: f32,
}

pub fn create_scroll_view(mut commands: Commands, q: Query<Entity, Added<ScrollView>>) {
    for e in q.iter() {
        commands.entity(e).with_children(|p| {
            p.spawn((
                NodeBundle {
                    style: Style {
                        overflow: Overflow::clip(),
                        max_height: Val::Percent(100.0),
                        max_width: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    ..Default::default()
                },
                ScrollViewport::default(),
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
                    ScrollViewContent { pos_y: 0.0 },
                ));
            });
        });
    }
}

fn input_mouse_pressed_move(
    mut motion_evr: EventReader<MouseMotion>,
    mut q: Query<(&Children, &Interaction, &Node), With<ScrollViewport>>,
    mut content_q: Query<(&mut Style, &mut ScrollViewContent, &Node)>,
) {
    for evt in motion_evr.read() {
        for (children, &interaction, node) in q.iter_mut() {
            if interaction != Interaction::Pressed {
                continue;
            }
            let container_height = node.size().y;
            for &child in children.iter() {
                if let Ok(item) = content_q.get_mut(child) {
                    let mut style = item.0;
                    let mut scroll = item.1;
                    let max_scroll = (item.2.size().y - container_height).max(0.0);
                    scroll.pos_y += evt.delta.y;
                    scroll.pos_y = scroll.pos_y.clamp(-max_scroll, 0.);
                    style.top = Val::Px(scroll.pos_y);
                }
            }
        }
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&Children, &Interaction, &ScrollViewport, &Node), With<ScrollViewport>>,
    time: Res<Time>,
    mut content_q: Query<(&mut Style, &mut ScrollViewContent, &Node)>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read() {
        for (children, &interaction, scroll_view, node) in q.iter_mut() {
            let y = match ev.unit {
                MouseScrollUnit::Line => {
                    ev.y * time.delta().as_secs_f32() * scroll_view.scroll_speed
                }
                MouseScrollUnit::Pixel => ev.y,
            };
            if interaction != Interaction::Hovered {
                continue;
            }
            let container_height = node.size().y;

            for &child in children.iter() {
                if let Ok(item) = content_q.get_mut(child) {
                    let y = y * time.delta().as_secs_f32() * scroll_view.scroll_speed;
                    let mut style = item.0;
                    let mut scroll = item.1;
                    let max_scroll = (item.2.size().y - container_height).max(0.0);
                    scroll.pos_y += y;
                    scroll.pos_y = scroll.pos_y.clamp(-max_scroll, 0.);
                    style.top = Val::Px(scroll.pos_y);
                }
            }
        }
    }
}
