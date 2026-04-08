use super::*;

pub(super) fn on_panel_resize_handle_press(
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

pub(super) fn handle_panel_window_resize(
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
    parents_query: Query<&ChildOf>,
    root_query: Query<&ResolvedRootUi>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera>>,
    global_query: Query<&GlobalTransform>,
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
            &parents_query,
            &root_query,
            &windows,
            &mut commands,
        ) {
            continue;
        }

        let Some(cursor_parent) = cursor_in_parent_space(
            entity,
            parent,
            &parents_query,
            &root_query,
            &pointers,
            &cameras,
            &global_query,
        ) else {
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
            width: uval_px(node.width)
                .unwrap_or(computed.width.max(panel_window.min_width.max(1.0))),
            height: uval_px(node.height)
                .unwrap_or(computed.height.max(panel_window.min_height.max(1.0))),
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
