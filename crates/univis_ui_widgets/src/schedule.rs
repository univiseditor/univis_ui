use bevy::ecs::schedule::SystemSet;

/// Shared `Update` schedule sets used by stateful widget runtimes.
///
/// These sets make it explicit which widget systems are responsible for
/// structural setup, state transitions, visual refreshes, and event emission.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UnivisWidgetUpdateSet {
    /// Widget systems that create or remove runtime entities/components.
    Build,
    /// Widget systems that mutate runtime state in response to input or messages.
    Logic,
    /// Widget systems that synchronize visuals from the latest widget state.
    Visual,
    /// Widget systems that emit change messages after state and visuals settle.
    Events,
}
