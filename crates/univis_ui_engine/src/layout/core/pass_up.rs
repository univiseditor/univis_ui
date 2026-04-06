#![allow(clippy::type_complexity)]

use crate::internal_prelude::*;
use bevy::prelude::*;

#[derive(Clone, Copy)]
struct MeasuredContainerInput {
    width: UVal,
    height: UVal,
    padding: USides,
    width_bounds: (f32, f32),
    height_bounds: (f32, f32),
    calculated_min_width: f32,
    calculated_max_width: f32,
    calculated_min_height: f32,
    calculated_max_height: f32,
}

#[derive(Default)]
#[doc(hidden)]
pub struct MeasurePassScratch {
    child_entities: Vec<Entity>,
}

/// The Upward Pass (Bottom-Up) of the layout algorithm.
///
/// Iterates from the deepest tree depth up to the root.
/// Calculates the **Intrinsic Size** of containers based on their children.
/// It ignores `Absolute` items as they are out-of-flow.
///
/// Upward Pass with Caching and Reverse Direction Support
pub fn upward_measure_pass_cached(
    _tree_depth: Res<LayoutTreeDepth>,
    rollout: Option<Res<UiRolloutConfig>>,
    mut cache: ResMut<LayoutCache>,
    mut profiler: Option<ResMut<LayoutProfiler>>,
    mut scratch: Local<MeasurePassScratch>,
    mut params: ParamSet<(
        Query<(&IntrinsicSize, &UNode, Option<&USelf>)>,
        Query<(Entity, &UNode, Option<&Children>, Option<&ULayout>)>,
        Query<&mut IntrinsicSize>,
    )>,
) {
    let start = std::time::Instant::now();
    let use_incremental_measure = rollout
        .as_ref()
        .map_or(true, |config| config.use_incremental_measure);
    let frontier = if use_incremental_measure {
        cache.take_measure_frontier()
    } else {
        cache.all_entities_bottom_up()
    };
    let mut calculated_count = 0;

    for entity in frontier {
        scratch.child_entities.clear();
        let Some(measured_input) = (|| {
            let (
                width,
                height,
                padding,
                width_bounds,
                height_bounds,
                direction,
                legacy_gap,
                row_gap,
                column_gap,
                measure_scratch_grew,
            ) = {
                let nodes = params.p1();
                let Ok((_, node_spec, children_opt, layout_opt)) = nodes.get(entity) else {
                    return None;
                };

                let measure_scratch_capacity_before = scratch.child_entities.capacity();
                if let Some(children) = children_opt {
                    scratch.child_entities.reserve(children.len());
                    scratch.child_entities.extend(children.iter());
                }

                (
                    node_spec.width,
                    node_spec.height,
                    node_spec.padding,
                    node_spec.width_bounds(),
                    node_spec.height_bounds(),
                    layout_opt
                        .as_ref()
                        .map(|layout| layout.flex_direction)
                        .unwrap_or(UFlexDirection::Row),
                    layout_opt.as_ref().map(|layout| layout.gap).unwrap_or(0.0),
                    layout_opt
                        .as_ref()
                        .and_then(|layout| layout.container_ext.box_align.row_gap),
                    layout_opt
                        .as_ref()
                        .and_then(|layout| layout.container_ext.box_align.column_gap),
                    scratch.child_entities.capacity() > measure_scratch_capacity_before,
                )
            };
            if let Some(prof) = profiler.as_mut() {
                if measure_scratch_grew {
                    prof.measure_scratch_alloc_grows += 1;
                }
                prof.measure_scratch_peak =
                    prof.measure_scratch_peak.max(scratch.child_entities.len());
            }

            let gap = if matches!(direction, UFlexDirection::Row | UFlexDirection::RowReverse) {
                column_gap.unwrap_or(legacy_gap)
            } else {
                row_gap.unwrap_or(legacy_gap)
            };

            let mut calculated_min_width = 0.0;
            let mut calculated_max_width = 0.0;
            let mut calculated_min_height = 0.0;
            let mut calculated_max_height = 0.0;

            if !scratch.child_entities.is_empty() {
                let mut accum_main_min: f32 = 0.0;
                let mut accum_main_max: f32 = 0.0;
                let mut max_cross_min: f32 = 0.0;
                let mut max_cross_max: f32 = 0.0;
                let mut visible_count = 0;

                let q_children = params.p0();

                for child_entity in scratch.child_entities.iter().copied() {
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
                        let margin = child_node.margin;

                        match direction {
                            UFlexDirection::Row | UFlexDirection::RowReverse => {
                                accum_main_min += min_w + margin.left + margin.right;
                                accum_main_max += max_w + margin.left + margin.right;
                                max_cross_min =
                                    max_cross_min.max(min_h + margin.top + margin.bottom);
                                max_cross_max =
                                    max_cross_max.max(max_h + margin.top + margin.bottom);
                            }
                            UFlexDirection::Column | UFlexDirection::ColumnReverse => {
                                accum_main_min += min_h + margin.top + margin.bottom;
                                accum_main_max += max_h + margin.top + margin.bottom;
                                max_cross_min =
                                    max_cross_min.max(min_w + margin.left + margin.right);
                                max_cross_max =
                                    max_cross_max.max(max_w + margin.left + margin.right);
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

            Some(MeasuredContainerInput {
                width,
                height,
                padding,
                width_bounds,
                height_bounds,
                calculated_min_width,
                calculated_max_width,
                calculated_min_height,
                calculated_max_height,
            })
        })() else {
            cache.clear_dirty(entity);
            continue;
        };

        calculated_count += 1;
        let h_pad = measured_input.padding.width_sum();
        let v_pad = measured_input.padding.height_sum();

        if let Ok(mut intrinsic) = params.p2().get_mut(entity) {
            let raw_min_width = match measured_input.width {
                UVal::Px(v) => v,
                _ => measured_input.calculated_min_width + h_pad,
            };
            let raw_max_width = match measured_input.width {
                UVal::Px(v) => v,
                _ => measured_input.calculated_max_width + h_pad,
            };
            let raw_min_height = match measured_input.height {
                UVal::Px(v) => v,
                _ => measured_input.calculated_min_height + v_pad,
            };
            let raw_max_height = match measured_input.height {
                UVal::Px(v) => v,
                _ => measured_input.calculated_max_height + v_pad,
            };

            let min_width =
                raw_min_width.clamp(measured_input.width_bounds.0, measured_input.width_bounds.1);
            let max_width = raw_max_width
                .clamp(measured_input.width_bounds.0, measured_input.width_bounds.1)
                .max(min_width);
            let min_height = raw_min_height.clamp(
                measured_input.height_bounds.0,
                measured_input.height_bounds.1,
            );
            let max_height = raw_max_height
                .clamp(
                    measured_input.height_bounds.0,
                    measured_input.height_bounds.1,
                )
                .max(min_height);

            let new_width = match measured_input.width {
                UVal::MinContent => min_width,
                _ => max_width,
            };
            let new_height = match measured_input.height {
                UVal::MinContent => min_height,
                _ => max_height,
            };

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
            cache.complete_measure(entity);
            cache.clear_dirty(entity);
        }
    }

    cache.increment_frame();

    if let Some(ref mut prof) = profiler {
        prof.upward_pass_time = start.elapsed().as_secs_f64() * 1000.0;
        prof.dirty_nodes = calculated_count;
    }
}
