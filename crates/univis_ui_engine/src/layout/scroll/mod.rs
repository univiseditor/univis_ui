//! Scroll container, viewport, visual scrollbars, and kinetic drag management for Univis UI.
//!
//! Provides:
//! - [`crate::layout::scroll::UScrollContainer`] and [`crate::layout::scroll::UScrollContent`] for mouse-wheel driven scrolling,
//!   target offset glide, and hardware SDF clipping via [`crate::layout::univis_node::UClip`].
//! - [`crate::layout::scroll::UScrollbarTrack`], [`crate::layout::scroll::UScrollbarThumb`], and [`crate::layout::scroll::UScrollbarFade`] for visual,
//!   proportional, customizable scrollbars.
//! - [`crate::layout::scroll::UScrollKineticDrag`] for touch and pointer drag-to-scroll with momentum and friction.

pub mod container;
pub mod kinetic;
pub mod scrollbar;

pub use container::{
    UScrollContainer, UScrollContent, apply_scroll_transitions, auto_init_scroll_content,
    handle_mouse_wheel_scroll, init_scroll_containers, sync_scroll_extents,
};
pub use kinetic::{UScrollKineticDrag, handle_kinetic_drag};
pub use scrollbar::{
    UScrollbarAxis, UScrollbarFade, UScrollbarThumb, UScrollbarTrack, calculate_thumb_size,
    handle_scrollbar_drag, handle_scrollbar_fade, sync_scrollbar_thumbs,
};
