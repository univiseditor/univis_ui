use bevy::prelude::*;

use crate::prelude::*;

pub mod math;
pub mod picking;
pub mod feedback;

pub mod prelude {
    pub use crate::interaction::{
        feedback::*,
        picking::*,
        UnivisInteractionPlugin,
    };
}

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