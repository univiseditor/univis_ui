pub mod widget;

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub use crate::widget::prelude::*;
    pub use univis_ui_engine::internal::ComputedSize;
    pub use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
    pub use univis_ui_engine::prelude::*;
    pub use univis_ui_interaction::prelude::*;
    pub use univis_ui_engine::schedule::UnivisPostUpdateSet;
    pub use univis_ui_style::prelude::*;
}

pub mod prelude {
    pub use crate::widget::prelude::*;
}
