use bevy_color::Color;
use bevy_gizmos::gizmos::Gizmos;
use bevy_math::{
    bounding::{Aabb2d, RayCast2d},
    prelude::*,
};

/// Draws points as a continuous line positioned and scaled by bounds.
///
/// Doesn't filter or clamp.
/// Line can draw outside the bounds.
///
/// Uses [`Gizmos::linestrip_2d`] to draw the line.
///
/// Computationally cheap since it doesn't perform bounds testing, raycasts, or partial line computation.
pub fn draw_gizmo_line_plot_2d_raw<'a>(
    points: impl IntoIterator<Item = &'a Vec2>,
    gizmos: &mut Gizmos,
    bounds: Rect,
    min_xy: Vec2,
    max_xy: Vec2,
    color: Color,
) {
    gizmos.linestrip_2d(
        points
            .into_iter()
            .map(|point| remap_vec2(min_xy, max_xy, bounds.min, bounds.max, *point)),
        color,
    );
}

/// Draws points as a continuous line positioned and scaled by bounds.
///
/// Clamps line segments to within bounds.
/// Skips line segments that lie out of bounds.
///
/// Uses [`Gizmos::line_2d`] to draw the line.
pub fn draw_gizmo_line_plot_2d_clamped<'a>(
    points: impl IntoIterator<Item = &'a Vec2>,
    gizmos: &mut Gizmos,
    bounds: Rect,
    min: Vec2,
    max: Vec2,
    color: Color,
) {
    let mut points_iter = points.into_iter().copied();

    let Some(last_point) = points_iter.next() else {
        return;
    };

    let mut last_point = remap_vec2(min, max, bounds.min, bounds.max, last_point);

    points_iter.for_each(|point| {
        let current_point = remap_vec2(min, max, bounds.min, bounds.max, point);

        let last_point_in_bounds = bounds.contains(last_point);
        let current_point_in_bounds = bounds.contains(current_point);

        if last_point_in_bounds && current_point_in_bounds {
            gizmos.line_2d(last_point, current_point, color);
        } else if last_point_in_bounds && !current_point_in_bounds {
            let line_delta = last_point - current_point;
            let line_dist = line_delta.length();
            let dir = Dir2::new(line_delta).unwrap();
            let t = RayCast2d::new(current_point, dir, line_dist)
                .aabb_intersection_at(&Aabb2d {
                    min: bounds.min,
                    max: bounds.max,
                })
                .unwrap();

            gizmos.line_2d(
                last_point,
                last_point.lerp(current_point, 1.0 - t / line_dist),
                color,
            );
        } else if !last_point_in_bounds && current_point_in_bounds {
            let line_delta = current_point - last_point;
            let line_dist = line_delta.length();
            let dir = Dir2::new(line_delta).unwrap();
            let t = RayCast2d::new(last_point, dir, line_dist)
                .aabb_intersection_at(&Aabb2d {
                    min: bounds.min,
                    max: bounds.max,
                })
                .unwrap();

            gizmos.line_2d(
                current_point,
                current_point.lerp(last_point, 1.0 - t / line_dist),
                color,
            );
        }

        last_point = current_point;
    });
}

/// Computes the minimum and maximum of a series of points.
#[inline]
pub fn points_min_max<'a>(points: impl IntoIterator<Item = &'a Vec2>) -> Option<(Vec2, Vec2)> {
    let mut points = points.into_iter().copied();

    let Some(first) = points.next() else {
        return None;
    };

    Some(points.fold((first, first), |(min, max), point| {
        (min.min(point), max.max(point))
    }))
}

/// Computes the minimum and maximum of multiple series of points.
#[inline]
pub fn lines_min_max_by_points<'a, 'b, L, LI: 'a, P, F>(lines: L, points: F) -> Option<(Vec2, Vec2)>
where
    L: IntoIterator<Item = &'a LI>,
    P: IntoIterator<Item = &'b Vec2>,
    F: Fn(&'a LI) -> P,
{
    lines
        .into_iter()
        .filter_map(|l| points_min_max(points(l)))
        .reduce(|(min, max), (line_min, line_max)| (min.min(line_min), max.max(line_max)))
}

#[inline]
fn remap_vec2(i_min: Vec2, i_max: Vec2, o_min: Vec2, o_max: Vec2, v: Vec2) -> Vec2 {
    Vec2 {
        x: v.x.remap(i_min.x, i_max.x, o_min.x, o_max.x),
        y: v.y.remap(i_min.y, i_max.y, o_min.y, o_max.y),
    }
}
