use bevy::ecs::relationship::Relationship;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon, Window};
use crate::prelude::*;

pub struct UnivisPanelPlugin;

impl Plugin for UnivisPanelPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UPanel>()
            .register_type::<UPanelWindow>()
            .add_systems(
                Update,
                (
                    sync_panel_visuals,
                    init_panel_window_handles,
                    sync_panel_resize_handles,
                    handle_panel_window_resize,
                    cleanup_orphan_panel_resize_handles,
                    update_panel_resize_cursor,
                )
                    .chain(),
            );
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct UPanel {
    pub background: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: UCornerRadius,
    pub padding: USides,
    pub gap: f32,
    pub direction: UFlexDirection,
}

impl Default for UPanel {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.13, 0.15, 0.2),
            border_color: Color::srgba(0.75, 0.8, 0.9, 0.2),
            border_width: 1.0,
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(12.0),
            gap: 10.0,
            direction: UFlexDirection::Column,
        }
    }
}

impl UPanel {
    pub fn card() -> Self {
        Self::default()
    }

    pub fn glass() -> Self {
        Self {
            background: Color::srgba(0.08, 0.1, 0.14, 0.72),
            border_color: Color::srgba(0.9, 0.95, 1.0, 0.25),
            ..default()
        }
    }

    pub fn with_padding(mut self, padding: USides) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    pub fn with_direction(mut self, direction: UFlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct UPanelWindow {
    pub border_hit_thickness: f32,
    pub min_width: f32,
    pub min_height: f32,
}

impl Default for UPanelWindow {
    fn default() -> Self {
        Self {
            border_hit_thickness: 8.0,
            min_width: 180.0,
            min_height: 120.0,
        }
    }
}

impl UPanelWindow {
    pub fn with_min_size(mut self, width: f32, height: f32) -> Self {
        self.min_width = width.max(1.0);
        self.min_height = height.max(1.0);
        self
    }

    pub fn with_border_hit_thickness(mut self, thickness: f32) -> Self {
        self.border_hit_thickness = thickness.max(1.0);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PanelResizeEdge {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl PanelResizeEdge {
    const ALL: [Self; 8] = [
        Self::N,
        Self::S,
        Self::E,
        Self::W,
        Self::NE,
        Self::NW,
        Self::SE,
        Self::SW,
    ];

    fn has_north(self) -> bool {
        matches!(self, Self::N | Self::NE | Self::NW)
    }

    fn has_south(self) -> bool {
        matches!(self, Self::S | Self::SE | Self::SW)
    }

    fn has_east(self) -> bool {
        matches!(self, Self::E | Self::NE | Self::SE)
    }

    fn has_west(self) -> bool {
        matches!(self, Self::W | Self::NW | Self::SW)
    }

    fn is_corner(self) -> bool {
        matches!(self, Self::NE | Self::NW | Self::SE | Self::SW)
    }
}

#[derive(Component, Clone, Copy)]
struct PanelResizeHandle {
    owner: Entity,
    edge: PanelResizeEdge,
}

#[derive(Component)]
struct PanelResizeChrome;

#[derive(Component, Default)]
struct PanelResizeRuntime {
    active_edge: Option<PanelResizeEdge>,
    last_parent_cursor: Option<Vec2>,
}

#[derive(Clone, Copy, Debug)]
struct PanelRect {
    width: f32,
    height: f32,
    left: f32,
    top: f32,
}

fn sync_panel_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UPanel, &UNode), Changed<UPanel>>,
) {
    for (entity, panel, existing_node) in query.iter() {
        commands.entity(entity).insert((
            UNode {
                width: existing_node.width,
                height: existing_node.height,
                padding: panel.padding,
                margin: existing_node.margin,
                background_color: panel.background,
                border_radius: panel.border_radius,
                shape_mode: existing_node.shape_mode,
            },
            UBorder {
                color: panel.border_color,
                width: panel.border_width,
                radius: panel.border_radius,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: panel.direction,
                gap: panel.gap,
                ..default()
            },
        ));
    }
}

fn init_panel_window_handles(
    mut commands: Commands,
    query: Query<Entity, Added<UPanelWindow>>,
) {
    for panel_entity in query.iter() {
        commands
            .entity(panel_entity)
            .insert(PanelResizeRuntime::default())
            .with_children(|parent| {
                for edge in PanelResizeEdge::ALL {
                    parent
                        .spawn((
                            PanelResizeChrome,
                            PanelResizeHandle {
                                owner: panel_entity,
                                edge,
                            },
                            UInteraction::default(),
                            UNode {
                                width: UVal::Px(0.0),
                                height: UVal::Px(0.0),
                                background_color: Color::NONE,
                                ..default()
                            },
                            USelf {
                                position_type: UPositionType::Absolute,
                                ..default()
                            },
                        ))
                        .observe(on_panel_resize_handle_press);
                }
            });
    }
}

fn sync_panel_resize_handles(
    panel_query: Query<
        (Entity, &UPanelWindow, &ComputedSize, &Children),
        Or<(Added<UPanelWindow>, Changed<UPanelWindow>, Changed<ComputedSize>, Changed<Children>)>,
    >,
    mut handle_query: Query<(&PanelResizeHandle, &mut UNode, &mut USelf), With<PanelResizeChrome>>,
) {
    for (panel_entity, panel_window, size, children) in panel_query.iter() {
        let width = size.width.max(1.0);
        let height = size.height.max(1.0);
        let thickness = panel_window
            .border_hit_thickness
            .max(1.0)
            .min(width * 0.5)
            .min(height * 0.5);
        let inner_width = (width - 2.0 * thickness).max(0.0);
        let inner_height = (height - 2.0 * thickness).max(0.0);

        for &child in children {
            let Ok((meta, mut node, mut uself)) = handle_query.get_mut(child) else {
                continue;
            };
            if meta.owner != panel_entity {
                continue;
            }

            let (left, top, w, h) = match meta.edge {
                PanelResizeEdge::N => (thickness, 0.0, inner_width, thickness),
                PanelResizeEdge::S => (thickness, (height - thickness).max(0.0), inner_width, thickness),
                PanelResizeEdge::E => ((width - thickness).max(0.0), thickness, thickness, inner_height),
                PanelResizeEdge::W => (0.0, thickness, thickness, inner_height),
                PanelResizeEdge::NE => ((width - thickness).max(0.0), 0.0, thickness, thickness),
                PanelResizeEdge::NW => (0.0, 0.0, thickness, thickness),
                PanelResizeEdge::SE => (
                    (width - thickness).max(0.0),
                    (height - thickness).max(0.0),
                    thickness,
                    thickness,
                ),
                PanelResizeEdge::SW => (0.0, (height - thickness).max(0.0), thickness, thickness),
            };

            node.width = UVal::Px(w);
            node.height = UVal::Px(h);
            node.background_color = Color::NONE;

            uself.position_type = UPositionType::Absolute;
            uself.left = UVal::Px(left);
            uself.top = UVal::Px(top);
        }
    }
}

fn on_panel_resize_handle_press(
    trigger: On<Pointer<Press>>,
    handle_query: Query<&PanelResizeHandle>,
    mut runtime_query: Query<&mut PanelResizeRuntime, With<UPanelWindow>>,
) {
    if trigger.event.button != PointerButton::Primary {
        return;
    }

    let handle_entity = trigger.entity.entity();
    let Ok(handle) = handle_query.get(handle_entity) else {
        return;
    };
    let Ok(mut runtime) = runtime_query.get_mut(handle.owner) else {
        return;
    };

    runtime.active_edge = Some(handle.edge);
    runtime.last_parent_cursor = None;
}

fn handle_panel_window_resize(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut panel_query: Query<(
        Entity,
        &UPanelWindow,
        &mut PanelResizeRuntime,
        &mut UNode,
        Option<&mut USelf>,
        &ComputedSize,
        &Transform,
        Option<&ChildOf>,
    )>,
    parent_size_query: Query<&ComputedSize>,
    world_root_query: Query<&UWorldRoot>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    parent_global_query: Query<&GlobalTransform>,
) {
    for (entity, panel_window, mut runtime, mut node, mut uself_opt, computed, transform, parent) in
        panel_query.iter_mut()
    {
        let Some(edge) = runtime.active_edge else {
            continue;
        };

        if !mouse_buttons.pressed(MouseButton::Left) {
            runtime.active_edge = None;
            runtime.last_parent_cursor = None;
            continue;
        }

        if !ensure_panel_absolute_geometry(
            entity,
            panel_window,
            &mut node,
            &mut uself_opt,
            computed,
            transform,
            parent,
            &parent_size_query,
            &world_root_query,
            &windows,
            &mut commands,
        ) {
            continue;
        }

        let Some(cursor_parent) =
            cursor_in_parent_space(parent, &windows, &cameras, &parent_global_query)
        else {
            continue;
        };

        let Some(previous_cursor) = runtime.last_parent_cursor else {
            runtime.last_parent_cursor = Some(cursor_parent);
            continue;
        };

        let Some(mut uself) = uself_opt else {
            continue;
        };

        let mut rect = PanelRect {
            width: uval_px(node.width).unwrap_or(computed.width.max(panel_window.min_width.max(1.0))),
            height: uval_px(node.height).unwrap_or(computed.height.max(panel_window.min_height.max(1.0))),
            left: uval_px(uself.left).unwrap_or(0.0),
            top: uval_px(uself.top).unwrap_or(0.0),
        };

        let delta_parent = cursor_parent - previous_cursor;
        apply_resize_delta_to_rect(
            edge,
            delta_parent,
            panel_window.min_width.max(1.0),
            panel_window.min_height.max(1.0),
            &mut rect,
        );

        node.width = UVal::Px(rect.width);
        node.height = UVal::Px(rect.height);
        uself.position_type = UPositionType::Absolute;
        uself.left = UVal::Px(rect.left);
        uself.top = UVal::Px(rect.top);

        runtime.last_parent_cursor = Some(cursor_parent);
    }
}

fn cleanup_orphan_panel_resize_handles(
    mut commands: Commands,
    handles: Query<(Entity, &PanelResizeHandle), With<PanelResizeChrome>>,
    windows: Query<(), With<UPanelWindow>>,
) {
    for (entity, handle) in handles.iter() {
        if windows.get(handle.owner).is_err() {
            commands.entity(entity).despawn();
        }
    }
}

fn update_panel_resize_cursor(
    handles: Query<(&PanelResizeHandle, &UInteraction), With<PanelResizeChrome>>,
    windows: Query<(Entity, Option<&CursorIcon>), With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let Ok((window_entity, current_cursor)) = windows.single() else {
        return;
    };

    let desired = pick_cursor_icon(
        handles
            .iter()
            .map(|(meta, interaction)| (meta.edge, interaction.clone())),
    );
    let desired_cursor = CursorIcon::System(desired);

    if current_cursor.map_or(true, |value| *value != desired_cursor) {
        commands.entity(window_entity).insert(desired_cursor);
    }
}

fn ensure_panel_absolute_geometry(
    panel_entity: Entity,
    panel_window: &UPanelWindow,
    node: &mut UNode,
    uself_opt: &mut Option<Mut<USelf>>,
    computed: &ComputedSize,
    transform: &Transform,
    parent: Option<&ChildOf>,
    parent_size_query: &Query<&ComputedSize>,
    world_root_query: &Query<&UWorldRoot>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    commands: &mut Commands,
) -> bool {
    let needs_absolute = !matches!(node.width, UVal::Px(_))
        || !matches!(node.height, UVal::Px(_))
        || uself_opt.as_ref().map_or(true, |uself| {
            uself.position_type != UPositionType::Absolute
                || !matches!(uself.left, UVal::Px(_))
                || !matches!(uself.top, UVal::Px(_))
        });

    if !needs_absolute {
        return true;
    }

    let width = computed.width.max(panel_window.min_width.max(1.0));
    let height = computed.height.max(panel_window.min_height.max(1.0));
    node.width = UVal::Px(width);
    node.height = UVal::Px(height);

    let parent_size = resolve_parent_size(
        panel_entity,
        parent,
        parent_size_query,
        world_root_query,
        windows,
    )
    .unwrap_or(Vec2::new(width, height));

    let left = transform.translation.x + (parent_size.x * 0.5) - (computed.width.max(1.0) * 0.5);
    let top = (parent_size.y * 0.5) - transform.translation.y - (computed.height.max(1.0) * 0.5);

    if let Some(uself) = uself_opt.as_mut() {
        uself.position_type = UPositionType::Absolute;
        uself.left = UVal::Px(left);
        uself.top = UVal::Px(top);
        true
    } else {
        commands.entity(panel_entity).insert(USelf {
            position_type: UPositionType::Absolute,
            left: UVal::Px(left),
            top: UVal::Px(top),
            ..default()
        });
        false
    }
}

fn resolve_parent_size(
    panel_entity: Entity,
    parent: Option<&ChildOf>,
    parent_size_query: &Query<&ComputedSize>,
    world_root_query: &Query<&UWorldRoot>,
    windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    if let Some(parent) = parent {
        if let Ok(size) = parent_size_query.get(parent.get()) {
            return Some(Vec2::new(size.width.max(1.0), size.height.max(1.0)));
        }
    }

    if let Ok(root) = world_root_query.get(panel_entity) {
        return Some(root.size);
    }

    if let Ok(window) = windows.single() {
        return Some(Vec2::new(window.width(), window.height()));
    }

    None
}

fn cursor_in_parent_space(
    parent: Option<&ChildOf>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    cameras: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    parent_global_query: &Query<&GlobalTransform>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let cursor_screen = window.cursor_position()?;
    let (camera, camera_transform) = cameras.single().ok()?;
    let ray = camera.viewport_to_world(camera_transform, cursor_screen).ok()?;
    let cursor_world = ray.origin.truncate();

    if let Some(parent) = parent {
        let parent_transform = parent_global_query.get(parent.get()).ok()?;
        let inv = parent_transform.to_matrix().inverse();
        return Some(
            inv.transform_point3(cursor_world.extend(0.0))
                .truncate(),
        );
    }

    Some(cursor_world)
}

fn apply_resize_delta_to_rect(
    edge: PanelResizeEdge,
    delta_parent: Vec2,
    min_width: f32,
    min_height: f32,
    rect: &mut PanelRect,
) {
    let min_width = min_width.max(1.0);
    let min_height = min_height.max(1.0);

    if edge.has_east() {
        rect.width = (rect.width + delta_parent.x).max(min_width);
    }

    if edge.has_west() {
        let prev = rect.width;
        rect.width = (rect.width - delta_parent.x).max(min_width);
        rect.left += prev - rect.width;
    }

    let delta_down = -delta_parent.y;
    if edge.has_south() {
        rect.height = (rect.height + delta_down).max(min_height);
    }

    if edge.has_north() {
        let prev = rect.height;
        rect.height = (rect.height - delta_down).max(min_height);
        rect.top += prev - rect.height;
    }
}

fn pick_cursor_icon<I>(states: I) -> SystemCursorIcon
where
    I: Iterator<Item = (PanelResizeEdge, UInteraction)>,
{
    let mut pressed_choice: Option<PanelResizeEdge> = None;
    let mut hovered_choice: Option<PanelResizeEdge> = None;

    for (edge, interaction) in states {
        match interaction {
            UInteraction::Pressed => {
                if should_replace_edge(pressed_choice, edge) {
                    pressed_choice = Some(edge);
                }
            }
            UInteraction::Hovered => {
                if should_replace_edge(hovered_choice, edge) {
                    hovered_choice = Some(edge);
                }
            }
            _ => {}
        }
    }

    pressed_choice
        .or(hovered_choice)
        .map(cursor_icon_for_edge)
        .unwrap_or(SystemCursorIcon::Default)
}

fn should_replace_edge(current: Option<PanelResizeEdge>, candidate: PanelResizeEdge) -> bool {
    match current {
        None => true,
        Some(current) => candidate.is_corner() && !current.is_corner(),
    }
}

fn cursor_icon_for_edge(edge: PanelResizeEdge) -> SystemCursorIcon {
    match edge {
        PanelResizeEdge::N | PanelResizeEdge::S => SystemCursorIcon::NsResize,
        PanelResizeEdge::E | PanelResizeEdge::W => SystemCursorIcon::EwResize,
        PanelResizeEdge::NE | PanelResizeEdge::SW => SystemCursorIcon::NeswResize,
        PanelResizeEdge::NW | PanelResizeEdge::SE => SystemCursorIcon::NwseResize,
    }
}

fn uval_px(value: UVal) -> Option<f32> {
    match value {
        UVal::Px(v) => Some(v),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_resize_edge_to_cursor_icon() {
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::N), SystemCursorIcon::NsResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::S), SystemCursorIcon::NsResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::E), SystemCursorIcon::EwResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::W), SystemCursorIcon::EwResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::NE), SystemCursorIcon::NeswResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::SW), SystemCursorIcon::NeswResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::NW), SystemCursorIcon::NwseResize);
        assert_eq!(cursor_icon_for_edge(PanelResizeEdge::SE), SystemCursorIcon::NwseResize);
    }

    #[test]
    fn panel_resize_west_clamp_updates_left() {
        let mut rect = PanelRect {
            width: 200.0,
            height: 120.0,
            left: 20.0,
            top: 10.0,
        };
        apply_resize_delta_to_rect(
            PanelResizeEdge::W,
            Vec2::new(80.0, 0.0),
            150.0,
            80.0,
            &mut rect,
        );

        assert_eq!(rect.width, 150.0);
        assert_eq!(rect.left, 70.0);
    }

    #[test]
    fn panel_resize_north_clamp_updates_top() {
        let mut rect = PanelRect {
            width: 220.0,
            height: 200.0,
            left: 0.0,
            top: 30.0,
        };
        apply_resize_delta_to_rect(
            PanelResizeEdge::N,
            Vec2::new(0.0, -80.0),
            100.0,
            150.0,
            &mut rect,
        );

        assert_eq!(rect.height, 150.0);
        assert_eq!(rect.top, 80.0);
    }

    #[test]
    fn panel_resize_corner_updates_both_axes() {
        let mut rect = PanelRect {
            width: 220.0,
            height: 180.0,
            left: 40.0,
            top: 20.0,
        };
        apply_resize_delta_to_rect(
            PanelResizeEdge::NW,
            Vec2::new(-30.0, 20.0),
            120.0,
            100.0,
            &mut rect,
        );

        assert_eq!(rect.width, 250.0);
        assert_eq!(rect.left, 10.0);
        assert_eq!(rect.height, 200.0);
        assert_eq!(rect.top, 0.0);
    }

    #[test]
    fn panel_cursor_priority_prefers_pressed() {
        let icon = pick_cursor_icon(
            [
                (PanelResizeEdge::NW, UInteraction::Hovered),
                (PanelResizeEdge::E, UInteraction::Pressed),
            ]
            .into_iter(),
        );
        assert_eq!(icon, SystemCursorIcon::EwResize);

        let corner_icon = pick_cursor_icon(
            [
                (PanelResizeEdge::E, UInteraction::Hovered),
                (PanelResizeEdge::NW, UInteraction::Hovered),
            ]
            .into_iter(),
        );
        assert_eq!(corner_icon, SystemCursorIcon::NwseResize);
    }
}
