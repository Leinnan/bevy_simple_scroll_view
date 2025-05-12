#![doc = include_str!("../README.md")]

use bevy::{input::mouse::MouseWheel, prelude::*};

/// A `Plugin` providing the systems and components required to make a ScrollView work.
///
/// # Example
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_simple_scroll_view::*;
///
/// App::new()
///     .add_plugins((DefaultPlugins,ScrollViewPlugin))
///     .run();
/// ```
pub struct ScrollViewPlugin;

impl Plugin for ScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ScrollView>()
            .register_type::<ScrollableContent>()
            .add_systems(
                Update,
                (
                    create_scroll_view,
                    update_size,
                    scroll_events,
                    fling_update,
                    scroll_update,
                )
                    .chain(),
            )
            .add_observer(on_drag)
            .add_observer(on_drag_end);
    }
}

/// Root component of scroll, it should have clipped style.
#[derive(Component, Debug, Reflect)]
#[require(Interaction, Node = scroll_view_node())]
pub struct ScrollView {
    /// Field which control speed of the scrolling.
    /// Could be negative number to implement invert scroll
    pub scroll_speed: f32,
    /// Amount of friction to apply to slow down the fling
    pub friction: f32,
    /// Current vertical velocity
    velocity: f32,
    /// Drag delta for fling
    drag_delta: Option<DragDelta>,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            scroll_speed: 1200.0,
            friction: 4.2,
            velocity: 0.0,
            drag_delta: None,
        }
    }
}

/// Component containing offset value of the scroll container to the parent.
/// It is possible to update the field `pos_y` manually to move scrollview to desired location.
#[derive(Component, Debug, Reflect, Default)]
#[require(Node = scroll_content_node())]
pub struct ScrollableContent {
    /// Scroll container offset to the `ScrollView`.
    pub pos_y: f32,
    /// Maximum value for the scroll. It is updated automatically based on the size of the children nodes.
    pub max_scroll: f32,
}

impl ScrollableContent {
    /// Scrolls to the top of the scroll view.
    pub fn scroll_to_top(&mut self) {
        self.pos_y = 0.0;
    }
    /// Scrolls to the bottom of the scroll view.
    pub fn scroll_to_bottom(&mut self) {
        self.pos_y = -self.max_scroll;
    }

    /// Scrolls by a specified amount.
    ///
    /// # Parameters
    /// - `value`: The amount to scroll vertically. Positive values scroll down,
    ///   and negative values scroll up.
    ///
    /// Ensures the new position is clamped between the valid scroll range.
    pub fn scroll_by(&mut self, value: f32) {
        self.pos_y += value;
        self.pos_y = self.pos_y.clamp(-self.max_scroll, 0.);
    }
}

/// Creates a default scroll view node.
///
/// This function defines the visual and layout properties of a scrollable container.
pub fn scroll_view_node() -> Node {
    Node {
        overflow: Overflow::clip(),
        align_items: AlignItems::Start,
        align_self: AlignSelf::Stretch,
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

/// Creates a default scroll content node.
pub fn scroll_content_node() -> Node {
    Node {
        flex_direction: bevy::ui::FlexDirection::Column,
        width: Val::Percent(100.0),
        ..default()
    }
}

/// Applies the default scroll view style to newly added `ScrollView` components.
///
/// This function updates the style of all new `ScrollView` nodes with the default
/// properties defined in `scroll_view_node`.
pub fn create_scroll_view(mut q: Query<&mut Node, Added<ScrollView>>) {
    let Node {
        overflow,
        align_items,
        align_self,
        flex_direction,
        ..
    } = scroll_view_node();
    for mut style in q.iter_mut() {
        style.overflow = overflow;
        style.align_items = align_items;
        style.align_self = align_self;
        style.flex_direction = flex_direction;
    }
}

fn update_size(
    mut q: Query<(&Children, &ComputedNode), With<ScrollView>>,
    mut content_q: Query<(&mut ScrollableContent, &ComputedNode), Changed<ComputedNode>>,
) {
    for (children, scroll_view_node) in q.iter_mut() {
        let container_height = scroll_view_node.size().y * scroll_view_node.inverse_scale_factor();
        for child in children.iter() {
            let Ok((mut scroll, node)) = content_q.get_mut(child) else {
                continue;
            };

            scroll.max_scroll =
                (node.size().y * node.inverse_scale_factor() - container_height).max(0.0);
            #[cfg(feature = "extra_logs")]
            info!(
                "CONTAINER {}, max_scroll: {}",
                container_height, scroll.max_scroll
            );
        }
    }
}

#[derive(Debug, Reflect)]
struct DragDelta {
    /// Sum of drag deltas since last reset
    pub diff: Vec2,
    /// Last time when drag_delta was added to velocity and reset
    pub time: f32,
}

fn on_drag(
    mut drag: Trigger<Pointer<Drag>>,
    mut q: Query<(&mut ScrollView, &mut Children)>,
    mut content_q: Query<&mut ScrollableContent>,
    time: Res<Time>,
) {
    if let Ok((mut view, children)) = q.get_mut(drag.target()) {
        if let Some(mut delta) = view.drag_delta.take() {
            let elapsed = time.elapsed_secs() - delta.time;
            if elapsed <= 0. {
                delta.diff += drag.delta;
                view.drag_delta = Some(delta);
            } else {
                view.velocity = (view.velocity + delta.diff.y / elapsed) / 2.0;
                view.drag_delta = Some(DragDelta {
                    diff: Vec2::new(0., 0.),
                    time: time.elapsed_secs(),
                });
            }
        } else {
            view.drag_delta = Some(DragDelta {
                diff: drag.delta,
                time: time.elapsed_secs(),
            });
        }
        for child in children.iter() {
            let Ok(mut scroll) = content_q.get_mut(child) else {
                continue;
            };
            scroll.scroll_by(drag.delta.y);
        }
        drag.propagate(false);
    }
}

fn on_drag_end(
    mut drag: Trigger<Pointer<DragEnd>>,
    mut q_view: Query<&mut ScrollView>,
) {
    if let Ok(mut view) = q_view.get_mut(drag.target()) {
        view.drag_delta = None;
        drag.propagate(false);
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&Children, &Interaction, &ScrollView), With<ScrollView>>,
    time: Res<Time>,
    mut content_q: Query<&mut ScrollableContent>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read() {
        for (children, &interaction, scroll_view) in q.iter_mut() {
            if interaction != Interaction::Hovered {
                continue;
            }
            let y = match ev.unit {
                MouseScrollUnit::Line => {
                    ev.y * time.delta().as_secs_f32() * scroll_view.scroll_speed
                }
                MouseScrollUnit::Pixel => ev.y,
            };
            #[cfg(feature = "extra_logs")]
            info!("Scroolling by {:#?}: {} movement", ev.unit, y);

            for child in children.iter() {
                let Ok(mut scroll) = content_q.get_mut(child) else {
                    continue;
                };
                scroll.scroll_by(y);
            }
        }
    }
}

fn scroll_update(mut q: Query<(&ScrollableContent, &mut Node), Changed<ScrollableContent>>) {
    for (scroll, mut style) in q.iter_mut() {
        style.top = Val::Px(scroll.pos_y);
    }
}

fn fling_update(
    mut q: Query<(&Children, &mut ScrollView)>,
    mut content_q: Query<&mut ScrollableContent>,
    time: Res<Time>,
) {
    for (children, mut view) in q.iter_mut() {
        if view.drag_delta.is_some() {
            continue;
        }
        if view.velocity.abs() > 1. {
            for child in children.iter() {
                let Ok(mut scroll) = content_q.get_mut(child) else {
                    continue;
                };
                let (position, _) = calc_position_and_velocity(
                    scroll.pos_y,
                    view.velocity,
                    -view.friction,
                    time.delta_secs(),
                );
                scroll.pos_y = position.clamp(-scroll.max_scroll, 0.);
            }
            view.velocity = calc_position_and_velocity(
                0.,
                view.velocity,
                -view.friction,
                time.delta_secs(),
            ).1;
        } else {
            view.velocity = 0.0;
        }
    }
}

fn calc_position_and_velocity(position: f32, velocity: f32, friction: f32, delta_t: f32) -> (f32, f32) {
    (
        position - velocity / friction + velocity / friction * (friction * delta_t).exp(),
        velocity * (delta_t * friction).exp(),
    )
}
