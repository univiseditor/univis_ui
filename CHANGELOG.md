# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
### Added
- **3D Text Rendering**: Added full support for rendering `UTextLabel` in 3D using standard PBR lighting. Text now accurately reacts to scene lighting and shadows just like standard 3D meshes.
- **3D Text Example**: Added `examples/text_3d.rs` to demonstrate PBR-lit 3D text labels alongside normal widgets and scene lights.
- **2D Picking Example**: Added `examples/picking_2d.rs` to demonstrate 2D UI picking interactions.

### Fixed
- **3D Text Scaling**: Fixed an issue where text meshes ignored the root UI scale (`ui_to_world_scale`) in 3D. 3D text now correctly and automatically scales to match the physical node dimensions (default `0.001` scale) without requiring manual Transform scaling.
- **Shader Pipeline Panics**: Resolved `Validation Error: Pipeline layout mismatch` panics occurring when using the wrong material bind group in 3D UI nodes. PBR widgets and text SDF shaders now correctly bind to `@group(3)`.
- **UI Picking Math**: Fixed UI picking math scaling and interaction detection for 3D elements, specifically ensuring hit tests work accurately under scaled projections and when nodes are nested.
- **Widget Behaviors**: Addressed issues with widgets like `seekbar` not responding interactively unless properly configured within panels.
