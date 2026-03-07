pub mod interaction;

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub use crate::interaction::prelude::*;
    pub use univis_ui_engine::internal::{ComputedSize, LayoutDepth};
    pub use univis_ui_engine::layout::geometry::{UCornerRadius, USides, UVal};
    pub use univis_ui_engine::prelude::*;
    pub use univis_ui_engine::schedule::UnivisPostUpdateSet;
}

pub mod prelude {
    pub use crate::interaction::prelude::*;
}
