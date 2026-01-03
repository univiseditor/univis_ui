//! # Univis UI
//!
//! **Univis** is a high-performance, SDF-based world-space UI library designed for the [Bevy Game Engine](https://bevyengine.org/).
//!
//! Unlike standard raster-based UI frameworks, Univis renders interface elements using **Signed Distance Fields (SDF)**
//! directly on meshes. This approach ensures **infinite resolution** (crisp edges without pixelation) at any zoom level
//! or camera angle, making it the ideal choice for:
//!
//! - **Diegetic UI:** Interfaces that exist within the game world (e.g., computer screens, holograms).
//! - **Sci-Fi HUDs:** Complex, glowing, and animated heads-up displays.
//! - **VR/AR:** Interfaces requiring high clarity and depth interaction.
//!
//! ## Key Features
//!
//! - **Infinite Resolution:** SDF rendering provides perfect anti-aliasing and rounded corners.
//! - **Advanced Layout Engine:** A powerful, single-pass layout solver supporting:
//!   - **Flexbox:** Standard row/column layouts.
//!   - **Grid:** 2D grid arrangements.
//!   - **Masonry:** Pinterest-style packing.
//!   - **Radial:** Circular layouts for sci-fi menus.
//! - **ECS-Native:** Fully integrated with Bevy's ECS. All UI elements are standard Entities with Components.
//! - **Physics Ready:** Elements interact with 3D lighting, depth, and physics (if configured).
//!
//! ## Quick Start
//!
//! Add the [`UnivisUiPlugin`] to your App to initialize the library.
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use univis::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(UnivisUiPlugin) // Initialize Univis
//!         .add_systems(Startup, setup_ui)
//!         .run();
//! }
//!
//! fn setup_ui(mut commands: Commands) {
//!     // Create a World Space Root
//!     commands.spawn((
//!         UWorldRoot {
//!             size: Vec2::new(800.0, 600.0),
//!             resolution_scale: 1.0,
//!         },
//!         // Main Container
//!         UNode {
//!             width: UVal::Percent(1.0),
//!             height: UVal::Percent(1.0),
//!             background_color: Color::srgb(0.1, 0.1, 0.1),
//!             padding: USides::all(20.0),
//!             ..default()
//!         },
//!         ULayout {
//!             display: UDisplay::Flex,
//!             align_items: UAlignItems::Center,
//!             justify_content: UJustifyContent::Center,
//!             ..default()
//!         }
//!     )).with_children(|parent| {
//!         // Add a Label
//!         parent.spawn(UTextLabel {
//!             text: "Hello Univis!".into(),
//!             font_size: 32.0,
//!             color: Color::WHITE,
//!             ..default()
//!         });
//!     });
//! }
//! ```

pub mod widget;
pub mod layout;
pub mod interaction;

/// A convenient module that exports the most commonly used types and traits.
///
/// It is recommended to import this module to get started:
/// ```rust
/// use univis::prelude::*;
/// ```
pub mod prelude {
    // Layout System
    pub use crate::layout::prelude::*;

    // Widgets (Buttons, Text, Checkboxes, etc.)
    pub use crate::widget::prelude::*;
    
    // Interaction System (Picking, Hover, Click)
    pub use crate::interaction::prelude::*;
    
    // The Main Plugin
    pub use crate::UnivisUiPlugin;
}

use bevy::prelude::*;
use crate::{prelude::*, widget::UnivisWidgetPlugin};

/// The main plugin for the Univis UI library.
///
/// This plugin initializes all necessary subsystems, including:
/// - **Interaction:** Picking events (Hover, Click, Drag).
/// - **Rendering:** Mesh generation and SDF shader pipelines.
/// - **Layout:** The flex/grid solver engine.
/// - **Widgets:** Systems for Sliders, Checkboxes, Text, etc.
///
/// # Example
///
/// ```rust
/// # use bevy::prelude::*;
/// # use univis::UnivisUiPlugin;
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(UnivisUiPlugin)
///     .run();
/// ```
pub struct UnivisUiPlugin;

impl Plugin for UnivisUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core Systems
            .add_plugins(UnivisInteractionPlugin)
            .add_plugins(MeshPickingPlugin) // Custom Backend for Rounded SDF Picking
            .add_plugins(UnivisNodePlugin)  // Core Node & Material Management
            .add_plugins(UnivisLayoutPlugin) // Layout Solver
            
            // Built-in Widgets
            .add_plugins(UnivisWidgetPlugin);
    }
}