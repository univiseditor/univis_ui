use bevy::prelude::*;
use crate::prelude::*;

/// The Downward Pass (Top-Down) of the layout algorithm.
///
/// Iterates from Root down to leaves.
/// Applies **Box Constraints**, runs the Solver, and updates `ComputedSize` and `Transform`.
pub fn downward_solve_pass(
    tree_depth: Res<LayoutTreeDepth>,
    // Comprehensive Query for nodes (mutable)
    mut nodes: Query<(
        Entity, 
        &UNode, 
        Option<&ULayout>, 
        &LayoutDepth, 
        Option<&Children>, 
        Option<&USelf>,
        &mut ComputedSize, 
        &mut Transform
    )>,
    // Query for IntrinsicSize (Read-only)
    intrinsic_query: Query<&IntrinsicSize>,
    // Root queries
    root_query: Query<&UWorldRoot>,
    window_query: Query<&Window>,
) {
    // 1. Iterate layers from Root (0) downwards
    for depth in 0..=tree_depth.max_depth {
        
        // Collect entities to avoid borrow conflicts
        let current_layer_entities: Vec<Entity> = nodes.iter()
            .filter(|(_, _, _, d, _, _, _, _)| d.0 == depth)
            .map(|(e, ..)| e)
            .collect();

        for entity in current_layer_entities {
            
            // a. Read Parent Data
            let (node_spec, layout_opt, children_vec, current_computed_size, _uself_opt) = {
                if let Ok((_, n, l, _, c, u, s, _)) = nodes.get(entity) {
                    let kids = c.map(|children| children.iter().collect::<Vec<Entity>>());
                    (n.clone(), l.cloned(), kids, *s, u.cloned())
                } else {
                    continue;
                }
            };

            // b. Determine Initial Container Size
            let container_size = if depth == 0 {
                // Root: Use Window size or UWorldRoot size
                if let Ok(world_root) = root_query.get(entity) {
                    world_root.size
                } else if let Ok(window) = window_query.single() {
                    Vec2::new(window.width(), window.height())
                } else {
                    Vec2::new(800.0, 600.0) // Fallback
                }
            } else {
                // Children: Use computed size (from parent's calculation or previous pass)
                Vec2::new(current_computed_size.width, current_computed_size.height)
            };

            // Immediately update Root Computed Size
            if depth == 0 {
                if let Ok((_, _, _, _, _, _, mut s, _)) = nodes.get_mut(entity) {
                    s.width = container_size.x;
                    s.height = container_size.y;
                }
            }

            let children = match children_vec { Some(c) => c, None => continue };
            let layout_config = layout_opt.unwrap_or_default(); 

            // c. Prepare Children for Solver
            let mut solver_items_specs = Vec::new();
            let mut temp_results = Vec::new(); 
            let mut temp_margins = Vec::new();
            let mut solved_children_entities = Vec::new(); 

            for &child_entity in &children {
                // Read child data
                if let Ok((_, child_node, _, _, _, child_uself, _, _)) = nodes.get(child_entity) {
                    let child_intrinsic = intrinsic_query.get(child_entity)
                        .unwrap_or(&IntrinsicSize { width: 0.0, height: 0.0 });
                    
                    // Translate spec
                    let mut spec = translate_spec(child_node, child_uself); 
                    
                    // Inject intrinsic size if Content mode
                    if spec.width_mode == SolverSizeMode::Content {
                        spec.width_val = child_intrinsic.width;
                    }
                    if spec.height_mode == SolverSizeMode::Content {
                        spec.height_val = child_intrinsic.height;
                    }

                    solver_items_specs.push(spec);
                    temp_results.push(SolverResult::default());
                    temp_margins.push(child_node.margin);
                    solved_children_entities.push(child_entity);
                }
            }

            if solved_children_entities.is_empty() {
                continue;
            }

            // Create SolverItems
            let mut final_items: Vec<SolverItem> = solver_items_specs.iter()
                .zip(temp_results.iter_mut())
                .zip(temp_margins.iter())
                .map(|((spec, res), margin)| SolverItem { 
                    spec: *spec, 
                    result: res,
                    margin: *margin 
                })
                .collect();

            // d. Setup Constraints
            let mut constraints = BoxConstraints::tight(container_size);

            // Open constraints for Auto/Content size to allow Placers to dictate size
            if node_spec.width == UVal::Auto || node_spec.width == UVal::Content {
                constraints.min_width = 0.0;
                constraints.max_width = f32::INFINITY; 
            }
            if node_spec.height == UVal::Auto || node_spec.height == UVal::Content {
                constraints.min_height = 0.0;
                constraints.max_height = f32::INFINITY;
            }

            // e. Run Solver
            let solver_conf = translate_config(&layout_config, &node_spec);
            
            // Solver returns actual used size
            let final_size = solve_flex_layout(&solver_conf, constraints, &mut final_items);

            // f. Update Container (Parent) with Final Size
            if let Ok((_, _, _, _, _, _, mut my_computed, _)) = nodes.get_mut(entity) {
                my_computed.width = final_size.x;
                my_computed.height = final_size.y;
            }

            // g. Apply Results to Children
            let parent_w = final_size.x;
            let parent_h = final_size.y;

            for (i, &child_entity) in solved_children_entities.iter().enumerate() {
                let result = &final_items[i].result;
                let spec = &final_items[i].spec;
                
                if let Ok((_, _, _, _, _, _, mut child_computed, mut child_transform)) = nodes.get_mut(child_entity) {
                    // Update Child Computed Size
                    child_computed.width = result.size.x;
                    child_computed.height = result.size.y;

                    // Update Child Position
                    // Transform UI Top-Left origin to Bevy Center origin
                    let child_w = result.size.x; 
                    let child_h = result.size.y;

                    child_transform.translation.x = (-parent_w / 2.0) + result.pos.x + (child_w / 2.0);
                    child_transform.translation.y = (parent_h / 2.0) - result.pos.y - (child_h / 2.0);
                    
                    // Z-Index: Base 0.1 + Order step
                    child_transform.translation.z = 0.1 + (spec.order as f32 * 0.001); 
                }
            }
        }
    }
}

// =========================================================
// Helpers
// =========================================================

fn map_uval_to_mode(val: UVal) -> SolverSizeMode {
    match val {
        UVal::Px(_) => SolverSizeMode::Fixed,
        UVal::Percent(_) => SolverSizeMode::Percent,
        UVal::Flex(_) => SolverSizeMode::Flex,
        UVal::Content | UVal::Auto => SolverSizeMode::Content,
    }
}

fn translate_config(layout: &ULayout, node: &UNode) -> SolverConfig {
    SolverConfig {
        layout: *layout,
        gap: layout.gap,
        padding: node.padding,
        grid_columns: layout.grid_columns,
        
        width_mode: map_uval_to_mode(node.width),
        height_mode: map_uval_to_mode(node.height),
    }
}

// Re-implementation of translate_spec for local usage (if needed) or reused
fn translate_spec(node: &UNode, uself: Option<&USelf>) -> SolverSpec {
    let map_dim = |dim: UVal| -> (SolverSizeMode, f32, f32) {
        match dim {
            UVal::Px(v) => (SolverSizeMode::Fixed, v, 0.0),
            UVal::Percent(p) => (SolverSizeMode::Percent, p, 0.0),
            UVal::Flex(f) => (SolverSizeMode::Flex, 0.0, f),
            UVal::Content | UVal::Auto => (SolverSizeMode::Content, 0.0, 0.0),
        }
    };

    let (w_mode, w_val, w_flex) = map_dim(node.width);
    let (h_mode, h_val, h_flex) = map_dim(node.height);

    let (pos_type, l, r, t, b, align, order) = if let Some(u) = uself {
        (u.position_type, u.left, u.right, u.top, u.bottom, 
         if u.align_self == UAlignSelf::Auto { None } else { Some(u.align_self) }, 
         u.order)
    } else {
        (
            UPositionType::Relative, 
            UVal::Auto, UVal::Auto, UVal::Auto, UVal::Auto, 
            None, 0
        )
    };

    SolverSpec {
        width_mode: w_mode, width_val: w_val, width_flex: w_flex,
        height_mode: h_mode, height_val: h_val, height_flex: h_flex,
        
        position_type: pos_type,
        left: l, right: r, top: t, bottom: b,
        align_self: align,
        order,
    }
}