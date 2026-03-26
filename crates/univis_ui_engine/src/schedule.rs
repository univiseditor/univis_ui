use bevy::ecs::schedule::SystemSet;

/// Shared `PostUpdate` schedule sets used across the Univis UI workspace.
///
/// These sets make ordering between widgets, root resolution, layout, and
/// render synchronization explicit and reusable.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UnivisPostUpdateSet {
    /// Widget-specific systems that prepare layout-affecting state.
    WidgetSync,
    /// Root resolution, camera binding, and root stacking capsule updates.
    RootResolve,
    /// Hierarchy analysis and cached parent/child relationships.
    LayoutHierarchy,
    /// Intrinsic measurement and fit-content evaluation.
    LayoutMeasure,
    /// Final downward layout solve and transform placement.
    LayoutSolve,
    /// Render-side synchronization after layout is final.
    RenderSync,
}
