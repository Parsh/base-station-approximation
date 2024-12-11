use geo::{Coord, Point};
use nalgebra::{Vector2, Matrix2, Matrix2x1};

use crate::SmartphoneData;

#[derive(Debug, Clone)]
struct Circle {
    center: Point<f64>,
    radius: f64,
}

pub fn multilaterate(ci_data: &[SmartphoneData]) -> Option<(f64, f64)> {
    // Require at least two circles for intersection
    if ci_data.len() < 2 {
        return None;
    }

    // Convert smartphone data to circles
    let circles: Vec<Circle> = ci_data
        .iter()
        .map(|data| Circle {
            center: Point::new(data.x, data.y),
            radius: data.distance,
        })
        .collect();

    // Try intersection of first two circles
    match find_circle_intersection(&circles[0], &circles[1]) {
        Some(point) => Some((point.x(), point.y())),
        None => None
    }
}

fn find_circle_intersection(circle1: &Circle, circle2: &Circle) -> Option<Point<f64>> {
    // Distance between circle centers
    let d = ((circle1.center.x() - circle2.center.x()).powi(2) + 
             (circle1.center.y() - circle2.center.y()).powi(2)).sqrt();

    // Check if circles are too far apart or don't intersect
    if d > circle1.radius + circle2.radius || 
       d < (circle1.radius - circle2.radius).abs() {
        return None;
    }

    // Compute intersection using analytic geometry
    let a = (circle1.radius.powi(2) - circle2.radius.powi(2) + d.powi(2)) / (2.0 * d);
    let h = (circle1.radius.powi(2) - a.powi(2)).sqrt();

    // Compute intersection coordinates
    let x0 = circle1.center.x() + a * (circle2.center.x() - circle1.center.x()) / d;
    let y0 = circle1.center.y() + a * (circle2.center.y() - circle1.center.y()) / d;

    let x1 = x0 + h * (circle2.center.y() - circle1.center.y()) / d;
    let y1 = y0 - h * (circle2.center.x() - circle1.center.x()) / d;

    Some(Point::new(x1, y1))
}