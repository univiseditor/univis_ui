mod cursor;
mod math;
mod model;
mod resize_runtime;
#[cfg(test)]
mod tests;
mod visuals;

use bevy::picking::backend::prelude::PointerLocation;
use bevy::picking::pointer::{PointerButton, PointerId};
use bevy::prelude::*;
#[cfg(test)]
use bevy::window::SystemCursorIcon;
use bevy::window::{CursorIcon, PrimaryWindow, Window};

#[cfg(test)]
use self::cursor::cursor_icon_for_edge;
use self::cursor::pick_cursor_icon;
#[cfg(test)]
use self::math::resolve_parent_size;
use self::math::{
    PanelRect, apply_resize_delta_to_rect, cursor_in_parent_space, ensure_panel_absolute_geometry,
    uval_px,
};
#[cfg(test)]
use self::model::PanelResizeEdge;
use self::model::{PanelResizeChrome, PanelResizeHandle, PanelResizeRuntime};
use self::resize_runtime::{handle_panel_window_resize, on_panel_resize_handle_press};
use self::visuals::{
    cleanup_orphan_panel_resize_handles, init_panel_window_handles, sync_panel_resize_handles,
    sync_panel_visuals, update_panel_resize_cursor,
};
use crate::schedule::UnivisWidgetUpdateSet;
use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
use univis_ui_engine::layout::query::{ComputedSize, ResolvedRootUi};
use univis_ui_engine::layout::univis_node::{
    UBorder, UDisplay, UFlexDirection, ULayout, UNode, UPositionType, USelf,
};
use univis_ui_interaction::interaction::feedback::UInteraction;

pub use self::model::{UPanel, UPanelWindow};

/// Registers panel and resizable panel-window behavior.
pub struct UnivisPanelPlugin;

impl Plugin for UnivisPanelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UPanel>()
            .register_type::<UPanelWindow>()
            .add_systems(
                Update,
                init_panel_window_handles.in_set(UnivisWidgetUpdateSet::Build),
            )
            .add_systems(
                Update,
                handle_panel_window_resize.in_set(UnivisWidgetUpdateSet::Logic),
            )
            .add_systems(
                Update,
                (
                    sync_panel_visuals,
                    cleanup_orphan_panel_resize_handles,
                    sync_panel_resize_handles,
                    update_panel_resize_cursor,
                )
                    .chain()
                    .in_set(UnivisWidgetUpdateSet::Visual),
            );
    }
}
