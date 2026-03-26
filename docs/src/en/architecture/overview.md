# Architecture

`univis_ui` is organized around four main subsystems:

- `layout/`: layout solving and spatial computation
- `interaction/`: picking and interaction state
- `widget/`: ready-made UI widgets
- `style/`: fonts, icons, and shared theme resources

## Core Idea

Everything in Univis is built from ECS entities and components:

- there is no external retained UI tree
- every UI element is an entity with `UNode` plus optional companion components
- layout, interaction, and rendering all run through Bevy systems

## Standard Frame Flow

1. `PreUpdate`
   - the picking backend resolves hit entities
2. `Update`
   - widget logic, interaction logic, and visual state changes run here
3. `PostUpdate`
   - the layout pipeline runs:
     - `update_layout_hierarchy`
     - `upward_measure_pass_cached`
     - `downward_solve_pass_safe`
   - render and material sync systems apply visual output

## Design Goals

- crisp visuals through SDF rendering
- flexible layout through `Flex`, `Grid`, `Masonry`, `Stack`, and `Radial`
- easy extension through ECS components and plugins
- debuggable performance through profiling and cache-aware systems
