//! Spatial candidate calculation and tab sequencing for Univis focus navigation.

use bevy::prelude::*;

use super::types::NavDirection;

/// Finds the best spatial navigation candidate in a given direction using interval-based geometry.
pub fn find_best_spatial_candidate(
    origin_pos: Vec2,
    origin_size: Vec2,
    direction: NavDirection,
    candidates: &[(Entity, Vec2, Vec2)],
    wrap_around: bool,
) -> Option<Entity> {
    let o_extents = origin_size * 0.5;
    let o_min = origin_pos - o_extents;
    let o_max = origin_pos + o_extents;

    let mut best_candidate: Option<(Entity, f32)> = None;

    for &(candidate_entity, c_pos, c_size) in candidates {
        let c_extents = c_size * 0.5;
        let c_min = c_pos - c_extents;
        let c_max = c_pos + c_extents;

        let (in_direction, primary_dist, secondary_dist) = match direction {
            NavDirection::Right => {
                let strictly_forward = c_pos.x > origin_pos.x + 0.5;
                let primary = if c_min.x >= o_max.x {
                    c_min.x - o_max.x
                } else {
                    (c_pos.x - origin_pos.x).max(1.0)
                };
                let overlap = o_min.y <= c_max.y && c_min.y <= o_max.y;
                let secondary = if overlap {
                    0.0
                } else {
                    (c_min.y - o_max.y).max(o_min.y - c_max.y)
                };
                (strictly_forward, primary, secondary)
            }
            NavDirection::Left => {
                let strictly_forward = c_pos.x < origin_pos.x - 0.5;
                let primary = if o_min.x >= c_max.x {
                    o_min.x - c_max.x
                } else {
                    (origin_pos.x - c_pos.x).max(1.0)
                };
                let overlap = o_min.y <= c_max.y && c_min.y <= o_max.y;
                let secondary = if overlap {
                    0.0
                } else {
                    (c_min.y - o_max.y).max(o_min.y - c_max.y)
                };
                (strictly_forward, primary, secondary)
            }
            NavDirection::Up => {
                let strictly_forward = c_pos.y > origin_pos.y + 0.5;
                let primary = if c_min.y >= o_max.y {
                    c_min.y - o_max.y
                } else {
                    (c_pos.y - origin_pos.y).max(1.0)
                };
                let overlap = o_min.x <= c_max.x && c_min.x <= o_max.x;
                let secondary = if overlap {
                    0.0
                } else {
                    (c_min.x - o_max.x).max(o_min.x - c_max.x)
                };
                (strictly_forward, primary, secondary)
            }
            NavDirection::Down => {
                let strictly_forward = c_pos.y < origin_pos.y - 0.5;
                let primary = if o_min.y >= c_max.y {
                    o_min.y - c_max.y
                } else {
                    (origin_pos.y - c_pos.y).max(1.0)
                };
                let overlap = o_min.x <= c_max.x && c_min.x <= o_max.x;
                let secondary = if overlap {
                    0.0
                } else {
                    (c_min.x - o_max.x).max(o_min.x - c_max.x)
                };
                (strictly_forward, primary, secondary)
            }
        };

        if in_direction {
            let score = primary_dist + 2.5 * secondary_dist;
            match best_candidate {
                Some((_, best_score)) if score < best_score => {
                    best_candidate = Some((candidate_entity, score));
                }
                None => {
                    best_candidate = Some((candidate_entity, score));
                }
                _ => {}
            }
        }
    }

    if best_candidate.is_some() || !wrap_around || candidates.is_empty() {
        return best_candidate.map(|(e, _)| e);
    }

    // Wrap-around fallback: Find furthest element in the opposite direction
    match direction {
        NavDirection::Right => candidates
            .iter()
            .min_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap())
            .map(|c| c.0),
        NavDirection::Left => candidates
            .iter()
            .max_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap())
            .map(|c| c.0),
        NavDirection::Up => candidates
            .iter()
            .min_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap())
            .map(|c| c.0),
        NavDirection::Down => candidates
            .iter()
            .max_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap())
            .map(|c| c.0),
    }
}

/// Selects the initial focus candidate when nothing is currently focused.
///
/// Favors explicit `tab_index` first, then visual top-left position (-y, +x).
pub fn find_initial_focus_candidate(candidates: &[(Entity, Option<i32>, Vec2)]) -> Option<Entity> {
    if candidates.is_empty() {
        return None;
    }

    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| {
        let a_tab = a.1.unwrap_or(i32::MAX);
        let b_tab = b.1.unwrap_or(i32::MAX);
        a_tab
            .cmp(&b_tab)
            .then_with(|| b.2.y.partial_cmp(&a.2.y).unwrap()) // Topmost first
            .then_with(|| a.2.x.partial_cmp(&b.2.x).unwrap()) // Leftmost first
    });

    sorted.first().map(|c| c.0)
}

/// Selects the next candidate in sequential tab navigation order.
pub fn find_next_tab_candidate(
    current: Option<Entity>,
    backward: bool,
    candidates: &[(Entity, Option<i32>, Vec2)],
) -> Option<Entity> {
    if candidates.is_empty() {
        return None;
    }

    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| {
        let a_tab = a.1.unwrap_or(i32::MAX);
        let b_tab = b.1.unwrap_or(i32::MAX);
        a_tab
            .cmp(&b_tab)
            .then_with(|| b.2.y.partial_cmp(&a.2.y).unwrap())
            .then_with(|| a.2.x.partial_cmp(&b.2.x).unwrap())
    });

    let current_idx = current.and_then(|curr| sorted.iter().position(|c| c.0 == curr));

    match current_idx {
        Some(idx) => {
            let next_idx = if backward {
                (idx + sorted.len() - 1) % sorted.len()
            } else {
                (idx + 1) % sorted.len()
            };
            Some(sorted[next_idx].0)
        }
        None => {
            if backward {
                sorted.last().map(|c| c.0)
            } else {
                sorted.first().map(|c| c.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_navigation_grid() {
        let e0 = Entity::from_bits(1);
        let e1 = Entity::from_bits(2);
        let e2 = Entity::from_bits(3);
        let e3 = Entity::from_bits(4);
        let e4 = Entity::from_bits(5);
        let e5 = Entity::from_bits(6);
        let e6 = Entity::from_bits(7);
        let e7 = Entity::from_bits(8);
        let e8 = Entity::from_bits(9);

        let size = Vec2::new(80.0, 40.0);
        let candidates = vec![
            (e0, Vec2::new(0.0, 100.0), size),
            (e1, Vec2::new(100.0, 100.0), size),
            (e2, Vec2::new(200.0, 100.0), size),
            (e3, Vec2::new(0.0, 0.0), size),
            (e4, Vec2::new(100.0, 0.0), size),
            (e5, Vec2::new(200.0, 0.0), size),
            (e6, Vec2::new(0.0, -100.0), size),
            (e7, Vec2::new(100.0, -100.0), size),
            (e8, Vec2::new(200.0, -100.0), size),
        ];

        let right = find_best_spatial_candidate(
            Vec2::new(100.0, 0.0),
            size,
            NavDirection::Right,
            &candidates,
            false,
        );
        assert_eq!(right, Some(e5));

        let left = find_best_spatial_candidate(
            Vec2::new(100.0, 0.0),
            size,
            NavDirection::Left,
            &candidates,
            false,
        );
        assert_eq!(left, Some(e3));

        let up = find_best_spatial_candidate(
            Vec2::new(100.0, 0.0),
            size,
            NavDirection::Up,
            &candidates,
            false,
        );
        assert_eq!(up, Some(e1));

        let down = find_best_spatial_candidate(
            Vec2::new(100.0, 0.0),
            size,
            NavDirection::Down,
            &candidates,
            false,
        );
        assert_eq!(down, Some(e7));
    }

    #[test]
    fn test_tab_navigation_ordering() {
        let e0 = Entity::from_bits(1);
        let e1 = Entity::from_bits(2);
        let e2 = Entity::from_bits(3);

        let candidates = vec![
            (e0, None, Vec2::new(0.0, 0.0)),
            (e1, Some(1), Vec2::new(0.0, 50.0)),
            (e2, Some(2), Vec2::new(0.0, 25.0)),
        ];

        assert_eq!(find_next_tab_candidate(None, false, &candidates), Some(e1));
        assert_eq!(
            find_next_tab_candidate(Some(e1), false, &candidates),
            Some(e2)
        );
        assert_eq!(
            find_next_tab_candidate(Some(e2), false, &candidates),
            Some(e0)
        );
        assert_eq!(
            find_next_tab_candidate(Some(e0), false, &candidates),
            Some(e1)
        );
        assert_eq!(
            find_next_tab_candidate(Some(e1), true, &candidates),
            Some(e0)
        );
    }

    #[test]
    fn test_wrap_around_spatial() {
        let e0 = Entity::from_bits(1);
        let e1 = Entity::from_bits(2);
        let size = Vec2::new(80.0, 40.0);

        let candidates = vec![
            (e0, Vec2::new(0.0, 0.0), size),
            (e1, Vec2::new(100.0, 0.0), size),
        ];

        let no_wrap = find_best_spatial_candidate(
            Vec2::new(100.0, 0.0),
            size,
            NavDirection::Right,
            &candidates,
            false,
        );
        assert_eq!(no_wrap, None);

        let wrap = find_best_spatial_candidate(
            Vec2::new(100.0, 0.0),
            size,
            NavDirection::Right,
            &candidates,
            true,
        );
        assert_eq!(wrap, Some(e0));
    }
}
