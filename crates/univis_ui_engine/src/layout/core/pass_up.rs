#![allow(clippy::type_complexity)]

use crate::internal_prelude::*;
use bevy::prelude::*;

/// The Upward Pass (Bottom-Up) of the layout algorithm.
///
/// Iterates from the deepest tree depth up to the root.
/// Calculates the **Intrinsic Size** of containers based on their children.
/// It ignores `Absolute` items as they are out-of-flow.
///
/// Upward Pass with Caching and Reverse Direction Support
pub fn upward_measure_pass_cached(
    tree_depth: Res<LayoutTreeDepth>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>,

    mut params: ParamSet<(
        Query<(&IntrinsicSize, &UNode, Option<&USelf>)>,
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
    let start = std::time::Instant::now();
    let mut calculated_count = 0;

    for depth in (0..=tree_depth.max_depth).rev() {
        let Some(layer_entities) = cache.get_entities_at_depth(depth) else {
            continue;
        };

        // تحضير البيانات وفحص حالة الاتساخ
        let layer_work_items: Vec<_> = {
            let q_parents = params.p1();
            layer_entities
                .iter()
                .filter_map(|&entity| {
                    q_parents
                        .get(entity)
                        .ok()
                        .map(|(e, node, _, children, layout, _)| {
                            let kids: Vec<Entity> =
                                children.map(|c| c.iter().collect()).unwrap_or_default();

                            let self_dirty = cache.is_dirty(entity);
                            let children_dirty = kids.iter().any(|child| cache.is_dirty(*child));
                            let effectively_dirty = self_dirty || children_dirty;

                            (e, node.clone(), kids, layout.cloned(), effectively_dirty)
                        })
                })
                .collect()
        };

        for (entity, node_spec, children, layout_opt, is_dirty) in layer_work_items {
            // 1. محاولة استخدام الكاش
            let mut used_cache = false;

            if !is_dirty
                && let Some(cached) = cache.get_cached_intrinsic(entity)
                && let Ok((_, _, _, _, _, mut intrinsic)) = params.p1().get_mut(entity)
            {
                *intrinsic = cached;
                used_cache = true;
            }

            if used_cache {
                continue;
            }

            // 2. الحساب الفعلي
            calculated_count += 1;

            let mut calculated_min_width = 0.0;
            let mut calculated_max_width = 0.0;
            let mut calculated_min_height = 0.0;
            let mut calculated_max_height = 0.0;
            let has_children = !children.is_empty();

            if has_children {
                let direction = layout_opt
                    .as_ref()
                    .map(|l| l.flex_direction)
                    .unwrap_or(UFlexDirection::Row);
                let legacy_gap = layout_opt.as_ref().map(|l| l.gap).unwrap_or(0.0);
                let gap = if matches!(direction, UFlexDirection::Row | UFlexDirection::RowReverse) {
                    layout_opt
                        .as_ref()
                        .and_then(|l| l.container_ext.box_align.column_gap)
                        .unwrap_or(legacy_gap)
                } else {
                    layout_opt
                        .as_ref()
                        .and_then(|l| l.container_ext.box_align.row_gap)
                        .unwrap_or(legacy_gap)
                };

                let mut accum_main_min: f32 = 0.0;
                let mut accum_main_max: f32 = 0.0;
                let mut max_cross_min: f32 = 0.0;
                let mut max_cross_max: f32 = 0.0;
                let mut visible_count = 0;

                let q_children = params.p0();

                for child_entity in children {
                    if let Ok((child_intrinsic, child_node, child_uself_opt)) =
                        q_children.get(child_entity)
                    {
                        if let Some(uself) = child_uself_opt
                            && uself.position_type == UPositionType::Absolute
                        {
                            continue;
                        }

                        let min_w = child_intrinsic.min_width;
                        let max_w = child_intrinsic.max_width;
                        let min_h = child_intrinsic.min_height;
                        let max_h = child_intrinsic.max_height;
                        let m = child_node.margin;

                        // === التحديث هنا: دمج الاتجاهات المعكوسة ===
                        match direction {
                            // الصفوف (عادي ومعكوس) تحسب العرض تراكمياً
                            UFlexDirection::Row | UFlexDirection::RowReverse => {
                                accum_main_min += min_w + m.left + m.right;
                                accum_main_max += max_w + m.left + m.right;
                                max_cross_min = max_cross_min.max(min_h + m.top + m.bottom);
                                max_cross_max = max_cross_max.max(max_h + m.top + m.bottom);
                            }
                            // الأعمدة (عادي ومعكوس) تحسب الارتفاع تراكمياً
                            UFlexDirection::Column | UFlexDirection::ColumnReverse => {
                                accum_main_min += min_h + m.top + m.bottom;
                                accum_main_max += max_h + m.top + m.bottom;
                                max_cross_min = max_cross_min.max(min_w + m.left + m.right);
                                max_cross_max = max_cross_max.max(max_w + m.left + m.right);
                            }
                        }
                        visible_count += 1;
                    }
                }

                if visible_count > 1 {
                    let gap_total = (visible_count - 1) as f32 * gap;
                    accum_main_min += gap_total;
                    accum_main_max += gap_total;
                }

                // === التحديث هنا أيضاً عند تعيين القيم النهائية ===
                match direction {
                    UFlexDirection::Row | UFlexDirection::RowReverse => {
                        calculated_min_width = accum_main_min;
                        calculated_max_width = accum_main_max;
                        calculated_min_height = max_cross_min;
                        calculated_max_height = max_cross_max;
                    }
                    UFlexDirection::Column | UFlexDirection::ColumnReverse => {
                        calculated_min_width = max_cross_min;
                        calculated_max_width = max_cross_max;
                        calculated_min_height = accum_main_min;
                        calculated_max_height = accum_main_max;
                    }
                }
            }

            let h_pad = node_spec.padding.width_sum();
            let v_pad = node_spec.padding.height_sum();

            let mut q_write = params.p1();
            if let Ok((_, _, _, _, _, mut intrinsic)) = q_write.get_mut(entity) {
                let raw_min_width = match node_spec.width {
                    UVal::Px(v) => v,
                    _ => calculated_min_width + h_pad,
                };
                let raw_max_width = match node_spec.width {
                    UVal::Px(v) => v,
                    _ => calculated_max_width + h_pad,
                };
                let raw_min_height = match node_spec.height {
                    UVal::Px(v) => v,
                    _ => calculated_min_height + v_pad,
                };
                let raw_max_height = match node_spec.height {
                    UVal::Px(v) => v,
                    _ => calculated_max_height + v_pad,
                };

                let min_width = node_spec.clamp_width(raw_min_width);
                let max_width = node_spec.clamp_width(raw_max_width).max(min_width);
                let min_height = node_spec.clamp_height(raw_min_height);
                let max_height = node_spec.clamp_height(raw_max_height).max(min_height);

                let new_width = match node_spec.width {
                    UVal::MinContent => min_width,
                    _ => max_width,
                };
                let new_height = match node_spec.height {
                    UVal::MinContent => min_height,
                    _ => max_height,
                };

                // منع التكرار اللانهائي (Check diff > epsilon)
                if (intrinsic.width - new_width).abs() > 0.001
                    || (intrinsic.height - new_height).abs() > 0.001
                    || (intrinsic.min_width - min_width).abs() > 0.001
                    || (intrinsic.max_width - max_width).abs() > 0.001
                    || (intrinsic.min_height - min_height).abs() > 0.001
                    || (intrinsic.max_height - max_height).abs() > 0.001
                {
                    intrinsic.width = new_width;
                    intrinsic.height = new_height;
                    intrinsic.min_width = min_width;
                    intrinsic.max_width = max_width;
                    intrinsic.min_height = min_height;
                    intrinsic.max_height = max_height;
                }

                cache.cache_intrinsic(
                    entity,
                    IntrinsicSize {
                        width: new_width,
                        height: new_height,
                        min_width,
                        max_width,
                        min_height,
                        max_height,
                    },
                );
            }
        }
    }

    cache.clear_all_dirty();
    cache.increment_frame();

    if let Some(ref mut prof) = profiler {
        prof.upward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
        prof.dirty_nodes = calculated_count;
    }
}
