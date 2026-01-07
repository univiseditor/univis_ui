use bevy::prelude::*;
use crate::prelude::*;

// =========================================================
// نسخة Owned آمنة من SolverItem
// =========================================================

/// نسخة آمنة من SolverItem - تملك بياناتها
pub struct SolverItemOwned {
    pub spec: SolverSpec,
    pub result: Box<SolverResult>,
    pub margin: USides,
}

impl SolverItemOwned {
    /// تحويل إلى SolverItem مؤقت (للاستخدام مع الـ Solver)
    pub fn as_solver_item(&mut self) -> SolverItem<'_> {
        SolverItem {
            spec: self.spec,
            result: &mut *self.result,
            margin: self.margin,
        }
    }
}

// =========================================================
// Data Structures
// =========================================================

#[derive(Clone)]
struct NodeData {
    spec: UNode,
    layout: ULayout,
    children: Vec<Entity>,
    computed_size: ComputedSize,
}

struct ChildLayoutData {
    entity: Entity,
    spec: SolverSpec,
    margin: USides,
}

struct SolvedChild {
    entity: Entity,
    result: SolverResult,
}

// =========================================================
// النظام الرئيسي - 100% آمن
// =========================================================

pub fn downward_solve_pass_safe(
    tree_depth: Res<LayoutTreeDepth>,
    cache: Res<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>, // إضافة Profiler اختياري
    
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
    
    intrinsic_query: Query<&IntrinsicSize>,
    root_query: Query<&UWorldRoot>,
    window_query: Query<&Window>,
) {
    let start = std::time::Instant::now();
    
    for depth in 0..=tree_depth.max_depth {
        
        // استخدام Cache
        let Some(layer_entities) = cache.get_entities_at_depth(depth) else {
            continue;
        };

        for &entity in layer_entities {
            
            // 1. استخراج البيانات
            let Some(node_data) = extract_node_data(entity, &nodes) else {
                continue;
            };

            // 2. حساب حجم الحاوية
            let container_size = if depth == 0 {
                calculate_root_size(entity, &root_query, &window_query)
            } else {
                Vec2::new(node_data.computed_size.width, node_data.computed_size.height)
            };

            // تحديث الجذر
            if depth == 0 {
                if let Ok((_, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
                    computed.width = container_size.x;
                    computed.height = container_size.y;
                }
            }

            // 3. جمع بيانات الأطفال
            let children_layout_data = collect_children_layout_data(
                &node_data.children,
                &nodes,
                &intrinsic_query,
            );

            if children_layout_data.is_empty() {
                continue;
            }

            // 4. تحويل إلى Solver (الطريقة الآمنة)
            let (mut solver_items_owned, entities_map) = 
                prepare_solver_data_safe(children_layout_data);

            // 5. تحويل مؤقت إلى SolverItem
            let mut solver_items_refs: Vec<SolverItem> = solver_items_owned
                .iter_mut()
                .map(|item| item.as_solver_item())
                .collect();

            // 6. إعداد القيود
            let constraints = build_constraints(container_size, &node_data.spec);

            // 7. تشغيل Solver
            let solver_config = translate_config(&node_data.layout, &node_data.spec);
            let final_size = solve_flex_layout(
                &solver_config,
                constraints,
                &mut solver_items_refs,
            );

            // 8. تحديث حجم الحاوية
            if let Ok((_, _, _, _, _, _, mut computed, _)) = nodes.get_mut(entity) {
                computed.width = final_size.x;
                computed.height = final_size.y;
            }

            // 9. ترجمة النتائج
            let solved_children: Vec<SolvedChild> = solver_items_owned
                .iter()
                .zip(entities_map.iter())
                .map(|(item, &entity)| SolvedChild {
                    entity,
                    result: *item.result,
                })
                .collect();

            // 10. تطبيق النتائج
            apply_results_to_children(&solved_children, final_size, &mut nodes);
        }
    }
    
    // تحديث Profiler
    if let Some(ref mut prof) = profiler {
        prof.downward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
    }
}

// =========================================================
// Helper Functions
// =========================================================

fn extract_node_data(
    entity: Entity,
    query: &Query<(
        Entity, &UNode, Option<&ULayout>, &LayoutDepth,
        Option<&Children>, Option<&USelf>,
        &mut ComputedSize, &mut Transform
    )>,
) -> Option<NodeData> {
    let (_, node, layout_opt, _, children_opt, _, computed, _) = 
        query.get(entity).ok()?;

    Some(NodeData {
        
        spec: node.clone(),
        layout: layout_opt.copied().unwrap_or_default(),
        children: children_opt
            .map(|c| c.iter().collect())
            .unwrap_or_default(),
        computed_size: *computed,
    })
}

fn calculate_root_size(
    entity: Entity,
    root_query: &Query<&UWorldRoot>,
    window_query: &Query<&Window>,
) -> Vec2 {
    if let Ok(world_root) = root_query.get(entity) {
        world_root.size
    } else if let Ok(window) = window_query.single() {
        Vec2::new(window.width(), window.height())
    } else {
        Vec2::new(800.0, 600.0)
    }
}

fn collect_children_layout_data(
    children: &[Entity],
    nodes_query: &Query<(
        Entity, &UNode, Option<&ULayout>, &LayoutDepth,
        Option<&Children>, Option<&USelf>,
        &mut ComputedSize, &mut Transform
    )>,
    intrinsic_query: &Query<&IntrinsicSize>,
) -> Vec<ChildLayoutData> {
    children.iter()
        .filter_map(|&child_entity| {
            let (_, node, _, _, _, uself_opt, _, _) = nodes_query.get(child_entity).ok()?;
            let intrinsic = intrinsic_query.get(child_entity).ok()?;

            let mut spec = translate_spec(node, uself_opt.as_deref());

            if spec.width_mode == SolverSizeMode::Content {
                spec.width_val = intrinsic.width;
            }
            if spec.height_mode == SolverSizeMode::Content {
                spec.height_val = intrinsic.height;
            }

            Some(ChildLayoutData {
                entity: child_entity,
                spec,
                margin: node.margin,
            })
        })
        .collect()
}

/// ✅ الطريقة الآمنة لتحضير بيانات Solver
fn prepare_solver_data_safe(
    children_data: Vec<ChildLayoutData>,
) -> (Vec<SolverItemOwned>, Vec<Entity>) {
    
    let mut entities_map = Vec::new();
    let mut solver_items = Vec::new();
    
    for child_data in children_data {
        entities_map.push(child_data.entity);
        
        solver_items.push(SolverItemOwned {
            spec: child_data.spec,
            result: Box::new(SolverResult::default()),
            margin: child_data.margin,
        });
    }
    
    (solver_items, entities_map)
}

fn build_constraints(
    container_size: Vec2,
    node_spec: &UNode,
) -> BoxConstraints {
    let mut constraints = BoxConstraints::tight(container_size);

    if matches!(node_spec.width, UVal::Auto | UVal::Content) {
        constraints.min_width = 0.0;
        constraints.max_width = f32::INFINITY;
    }

    if matches!(node_spec.height, UVal::Auto | UVal::Content) {
        constraints.min_height = 0.0;
        constraints.max_height = f32::INFINITY;
    }

    constraints
}

fn apply_results_to_children(
    solved_children: &[SolvedChild],
    parent_size: Vec2,
    nodes_query: &mut Query<(
        Entity, &UNode, Option<&ULayout>, &LayoutDepth,
        Option<&Children>, Option<&USelf>,
        &mut ComputedSize, &mut Transform
    )>,
) {
    for solved in solved_children.iter() {
        if let Ok((_, _, _, _, _, _, mut computed, mut transform)) = 
            nodes_query.get_mut(solved.entity) 
        {
            computed.width = solved.result.size.x;
            computed.height = solved.result.size.y;

            let child_w = solved.result.size.x;
            let child_h = solved.result.size.y;

            transform.translation.x = (-parent_size.x / 2.0) 
                + solved.result.pos.x 
                + (child_w / 2.0);
                
            transform.translation.y = (parent_size.y / 2.0) 
                - solved.result.pos.y 
                - (child_h / 2.0);
            
            transform.translation.z = 0.1;
        }
    }
}

// =========================================================
// Translation Functions
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
        (
            u.position_type, u.left, u.right, u.top, u.bottom,
            if u.align_self == UAlignSelf::Auto { None } else { Some(u.align_self) },
            u.order,
        )
    } else {
        (
            UPositionType::Relative,
            UVal::Auto, UVal::Auto, UVal::Auto, UVal::Auto,
            None, 0,
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