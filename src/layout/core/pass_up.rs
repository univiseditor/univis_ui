use crate::prelude::*;
use bevy::prelude::*;

/// The Upward Pass (Bottom-Up) of the layout algorithm.
///
/// Iterates from the deepest tree depth up to the root.
/// Calculates the **Intrinsic Size** of containers based on their children.
/// It ignores `Absolute` items as they are out-of-flow.
pub fn upward_measure_pass(
    tree_depth: Res<LayoutTreeDepth>,
    mut params: ParamSet<(
        // P0: Read children data (IntrinsicSize, UNode, USelf)
        Query<(&IntrinsicSize, &UNode, Option<&USelf>)>,
        // P1: Read/Write parent data (IntrinsicSize)
        Query<(
            Entity,
            &UNode,
            &LayoutDepth,
            Option<&Children>,
            Option<&ULayout>,
            &mut IntrinsicSize,
        )>,
    )>,
) {
    // Start from the bottom (max depth) and go up
    for depth in (0..=tree_depth.max_depth).rev() {
        // 1. Collect "Work Items" for this layer to avoid borrow checker issues with P0/P1
        let layer_work_items: Vec<(Entity, UNode, Option<Vec<Entity>>, Option<ULayout>)> = {
            let q_parents = params.p1();
            q_parents
                .iter()
                .filter(|(_, _, d, _, _, _)| d.0 == depth)
                .map(|(e, node, _, children, layout, _)| {
                    let kids = children.map(|c| c.iter().collect());
                    (e, node.clone(), kids, layout.cloned())
                })
                .collect()
        };

        // 2. Process each parent in this layer
        for (entity, node_spec, children_vec, layout_opt) in layer_work_items {
            let mut calculated_width = 0.0;
            let mut calculated_height = 0.0;

            // a. Calculate content size from children (Using P0)
            if let Some(children) = children_vec {
                let direction = layout_opt
                    .as_ref()
                    .map(|l| l.flex_direction)
                    .unwrap_or(UFlexDirection::Row);
                let gap = layout_opt.as_ref().map(|l| l.gap).unwrap_or(0.0);

                let mut accum_main: f32 = 0.0;
                let mut max_cross: f32 = 0.0;
                let mut visible_count = 0;

                // Open P0 for reading
                let q_children = params.p0();

                for child_entity in children {
                    if let Ok((child_intrinsic, child_node, child_uself_opt)) =
                        q_children.get(child_entity)
                    {
                        // Fix 1: Ignore Absolute items
                        if let Some(uself) = child_uself_opt {
                            if uself.position_type == UPositionType::Absolute {
                                continue;
                            }
                        }

                        let w = child_intrinsic.width;
                        let h = child_intrinsic.height;

                        // Fix 2: Add child margins
                        let m = child_node.margin;

                        match direction {
                            UFlexDirection::Row => {
                                // Main Axis (Width): Child Width + Horizontal Margins
                                accum_main += w + m.left + m.right;

                                // Cross Axis (Height): Max Height + Vertical Margins
                                max_cross = max_cross.max(h + m.top + m.bottom);
                            }
                            UFlexDirection::Column => {
                                // Main Axis (Height): Child Height + Vertical Margins
                                accum_main += h + m.top + m.bottom;

                                // Cross Axis (Width): Max Width + Horizontal Margins
                                max_cross = max_cross.max(w + m.left + m.right);
                            }
                        }
                        visible_count += 1;
                    }
                }

                // Add Gaps
                if visible_count > 1 {
                    accum_main += (visible_count - 1) as f32 * gap;
                }

                match direction {
                    UFlexDirection::Row => {
                        calculated_width = accum_main;
                        calculated_height = max_cross;
                    }
                    UFlexDirection::Column => {
                        calculated_width = max_cross;
                        calculated_height = accum_main;
                    }
                }
            }

            // b. Add Parent Padding
            let h_pad = node_spec.padding.width_sum(); // left + right
            let v_pad = node_spec.padding.height_sum(); // top + bottom

            // c. Write result to Parent (Open P1 for write)
            let mut q_write = params.p1();
            if let Ok((_, _, _, _, _, mut intrinsic)) = q_write.get_mut(entity) {
                // If explicit pixel size is set, use it.
                // Otherwise (Auto/Content/Flex/Percent), use calculated size.
                intrinsic.width = match node_spec.width {
                    UVal::Px(v) => v,
                    _ => calculated_width + h_pad,
                };

                intrinsic.height = match node_spec.height {
                    UVal::Px(v) => v,
                    _ => calculated_height + v_pad,
                };
            }
        }
    }
}

pub fn upward_measure_pass_cached(
    tree_depth: Res<LayoutTreeDepth>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    
    mut params: ParamSet<(
        Query<(&IntrinsicSize, &UNode, Option<&USelf>)>,
        Query<(Entity, &UNode, &LayoutDepth, Option<&Children>, Option<&ULayout>, &mut IntrinsicSize)>,
    )>,
) {
    let start = std::time::Instant::now();
    let mut calculated_count = 0;

    for depth in (0..=tree_depth.max_depth).rev() {
        
        let Some(layer_entities) = cache.get_entities_at_depth(depth) else {
            continue;
        };
        
        // تحضير البيانات وفحص حالة الاتساخ
        let layer_work_items: Vec<_> = {
            let q_parents = params.p1();
            layer_entities.iter()
                .filter_map(|&entity| {
                    q_parents.get(entity).ok()
                        .map(|(e, node, _, children, layout, _)| {
                            let kids: Vec<Entity> = children.map(|c| c.iter().collect()).unwrap_or_default();
                            
                            let self_dirty = cache.is_dirty(entity);
                            let children_dirty = kids.iter().any(|child| cache.is_dirty(*child));
                            let effectively_dirty = self_dirty || children_dirty;

                            (e, node.clone(), kids, layout.cloned(), effectively_dirty)
                        })
                })
                .collect()
        };

        for (entity, node_spec, children, layout_opt, is_dirty) in layer_work_items {
            
            // === التصحيح هنا ===
            // التحقق مما إذا كان يمكننا استخدام الكاش
            let mut used_cache = false;

            if !is_dirty {
                // نحاول القراءة من الكاش
                if let Some(cached) = cache.get_cached_intrinsic(entity) {
                    if let Ok((_, _, _, _, _, mut intrinsic)) = params.p1().get_mut(entity) {
                        *intrinsic = cached;
                        used_cache = true; // نجحنا في استخدام الكاش
                    }
                }
            }

            // إذا استخدمنا الكاش بنجاح، نتخطى الحساب
            if used_cache {
                continue;
            }
            
            // إذا وصلنا هنا، فهذا يعني إما أن العنصر متسخ (Dirty)
            // أو أنه نظيف ولكن غير موجود في الكاش (Cache Miss)
            // في الحالتين: يجب الحساب!
            
            calculated_count += 1;
            
            // ... (باقي كود الحساب كما هو تماماً دون تغيير) ...
            let mut calculated_width = 0.0;
            let mut calculated_height = 0.0;
            let has_children = !children.is_empty();

            if has_children {
                let direction = layout_opt.as_ref()
                    .map(|l| l.flex_direction)
                    .unwrap_or(UFlexDirection::Row);
                let gap = layout_opt.as_ref().map(|l| l.gap).unwrap_or(0.0);
                
                let mut accum_main: f32 = 0.0;
                let mut max_cross: f32 = 0.0;
                let mut visible_count = 0;

                let q_children = params.p0();

                for child_entity in children {
                    if let Ok((child_intrinsic, child_node, child_uself_opt)) = q_children.get(child_entity) {
                        
                        if let Some(uself) = child_uself_opt {
                            if uself.position_type == UPositionType::Absolute {
                                continue;
                            }
                        }

                        let w = child_intrinsic.width;
                        let h = child_intrinsic.height;
                        let m = child_node.margin;

                        match direction {
                            UFlexDirection::Row => {
                                accum_main += w + m.left + m.right;
                                max_cross = max_cross.max(h + m.top + m.bottom);
                            },
                            UFlexDirection::Column => {
                                accum_main += h + m.top + m.bottom;
                                max_cross = max_cross.max(w + m.left + m.right);
                            },
                        }
                        visible_count += 1;
                    }
                }

                if visible_count > 1 {
                    accum_main += (visible_count - 1) as f32 * gap;
                }

                match direction {
                    UFlexDirection::Row => {
                        calculated_width = accum_main;
                        calculated_height = max_cross;
                    },
                    UFlexDirection::Column => {
                        calculated_width = max_cross;
                        calculated_height = accum_main;
                    },
                }
            }

            let h_pad = node_spec.padding.width_sum();
            let v_pad = node_spec.padding.height_sum();

            let mut q_write = params.p1();
            if let Ok((_, _, _, _, _, mut intrinsic)) = q_write.get_mut(entity) {
                
                let new_width = match node_spec.width {
                    UVal::Px(v) => v,
                    _ => calculated_width + h_pad,
                };

                let new_height = match node_spec.height {
                    UVal::Px(v) => v,
                    _ => calculated_height + v_pad,
                };
                
                // === التعديل الهام لمنع التكرار ===
                // نكتب فقط إذا تغيرت القيمة فعلياً
                // هذا يمنع triggering Changed<IntrinsicSize> إذا كانت القيمة نفسها
                if (intrinsic.width - new_width).abs() > 0.001 || (intrinsic.height - new_height).abs() > 0.001 {
                    intrinsic.width = new_width;
                    intrinsic.height = new_height;
                }
                
                // تحديث الكاش بالقيمة الجديدة (دائماً نحدث الكاش لضمان التزامن)
                cache.cache_intrinsic(entity, IntrinsicSize { width: new_width, height: new_height });
            }
        }
    }
    
    // تنظيف العلامات في النهاية
    cache.clear_all_dirty();
    cache.increment_frame();
    
    if let Some(ref mut prof) = profiler {
        prof.upward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
        prof.dirty_nodes = calculated_count;
    }
}