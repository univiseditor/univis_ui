use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UnivisPostUpdateSet {
    WidgetSync,
    LayoutHierarchy,
    LayoutMeasure,
    LayoutSolve,
    RenderSync,
}
