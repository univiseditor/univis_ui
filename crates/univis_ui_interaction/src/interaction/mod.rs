//! Picking and interaction state for Univis UI.
//!
//! The interaction plugin resolves hits against Univis roots and updates
//! high-level state such as [`feedback::UInteraction`].

use bevy::prelude::*;

use crate::internal_prelude::*;

pub mod feedback;
pub mod math;
pub mod picking;

/// Common imports for interaction-related integrations.
pub mod prelude {
    pub use crate::interaction::{UnivisInteractionPlugin, feedback::*, picking::*};
}

/// Registers Univis pointer picking and the default interaction observers.
pub struct UnivisInteractionPlugin;

impl Plugin for UnivisInteractionPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(UnivisInputFieldPlugin);
        // 1. إضافة Backend الالتقاط (حساب من أين يمر الماوس)
        app.add_systems(PreUpdate, univis_picking_backend);

        // 2. تسجيل المراقبين (Observers) - الطريقة الجديدة للتفاعل
        // هذه المراقبون سيعملون تلقائياً لأي كيان يرسل له Backend حدثاً
        app.add_observer(feedback::on_pointer_over);
        app.add_observer(feedback::on_pointer_out);
        app.add_observer(feedback::on_pointer_press);
        app.add_observer(feedback::on_pointer_release);
        app.add_observer(feedback::on_pointer_click);
    }
}
