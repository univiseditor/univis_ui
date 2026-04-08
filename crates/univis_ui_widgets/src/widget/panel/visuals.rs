use super::*;

pub(super) fn sync_panel_visuals(
    mut commands: Commands,
    query: Query<(Entity, &UPanel, &UNode), Changed<UPanel>>,
) {
    for (entity, panel, existing_node) in query.iter() {
        commands.entity(entity).insert((
            UNode {
                padding: panel.padding,
                background_color: panel.background,
                border_radius: panel.border_radius,
                ..existing_node.clone()
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

pub(super) fn init_panel_window_handles(
    mut commands: Commands,
    query: Query<Entity, Added<UPanelWindow>>,
) {
    for panel_entity in query.iter() {
        commands
            .entity(panel_entity)
            .insert(PanelResizeRuntime::default())
            .with_children(|parent| {
                for edge in model::PanelResizeEdge::ALL {
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

pub(super) fn sync_panel_resize_handles(
    panel_query: Query<
        (Entity, &UPanelWindow, &ComputedSize, &Children),
        Or<(
            Added<UPanelWindow>,
            Changed<UPanelWindow>,
            Changed<ComputedSize>,
            Changed<Children>,
        )>,
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
                model::PanelResizeEdge::N => (thickness, 0.0, inner_width, thickness),
                model::PanelResizeEdge::S => (
                    thickness,
                    (height - thickness).max(0.0),
                    inner_width,
                    thickness,
                ),
                model::PanelResizeEdge::E => (
                    (width - thickness).max(0.0),
                    thickness,
                    thickness,
                    inner_height,
                ),
                model::PanelResizeEdge::W => (0.0, thickness, thickness, inner_height),
                model::PanelResizeEdge::NE => {
                    ((width - thickness).max(0.0), 0.0, thickness, thickness)
                }
                model::PanelResizeEdge::NW => (0.0, 0.0, thickness, thickness),
                model::PanelResizeEdge::SE => (
                    (width - thickness).max(0.0),
                    (height - thickness).max(0.0),
                    thickness,
                    thickness,
                ),
                model::PanelResizeEdge::SW => {
                    (0.0, (height - thickness).max(0.0), thickness, thickness)
                }
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

pub(super) fn cleanup_orphan_panel_resize_handles(
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

pub(super) fn update_panel_resize_cursor(
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
