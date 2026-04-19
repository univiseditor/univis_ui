use bevy::window::SystemCursorIcon;
use univis_ui_interaction::interaction::feedback::UInteraction;

use super::model::PanelResizeEdge;

pub(super) fn pick_cursor_icon<I>(states: I) -> SystemCursorIcon
where
    I: Iterator<Item = (PanelResizeEdge, UInteraction)>,
{
    let mut pressed_choice: Option<PanelResizeEdge> = None;
    let mut hovered_choice: Option<PanelResizeEdge> = None;

    for (edge, interaction) in states {
        match interaction {
            UInteraction::Pressed if should_replace_edge(pressed_choice, edge) => {
                pressed_choice = Some(edge);
            }
            UInteraction::Hovered if should_replace_edge(hovered_choice, edge) => {
                hovered_choice = Some(edge);
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

pub(super) fn cursor_icon_for_edge(edge: PanelResizeEdge) -> SystemCursorIcon {
    match edge {
        PanelResizeEdge::N | PanelResizeEdge::S => SystemCursorIcon::NsResize,
        PanelResizeEdge::E | PanelResizeEdge::W => SystemCursorIcon::EwResize,
        PanelResizeEdge::NE | PanelResizeEdge::SW => SystemCursorIcon::NeswResize,
        PanelResizeEdge::NW | PanelResizeEdge::SE => SystemCursorIcon::NwseResize,
    }
}
