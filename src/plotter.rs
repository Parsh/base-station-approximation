use plotters::prelude::*;
use std::{collections::HashMap, error::Error};

use crate::SmartphoneData;

pub fn plot_smartphone_data(
    smartphone_data: &[SmartphoneData], 
    approximated_positions: &HashMap<i32, Option<(f64, f64)>>,
    path: &str
) -> Result<(), Box<dyn Error>> {
    // Create a drawing area
    let root = BitMapBackend::new(path, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..3000f64, 0f64..3000f64)?;

    // Draw grid
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_label_formatter(&|v| format!("{:.0}", v))
        .y_label_formatter(&|v| format!("{:.0}", v))
        .draw()?;

    // Plot smartphone points and their signal circles
    for data in smartphone_data {
        // Draw signal circle
        chart.draw_series(std::iter::once(Circle::new(
            (data.x, data.y), 
            data.distance, 
            ShapeStyle {
                color: RGBColor(0, 0, 255).mix(0.2), // Transparent blue
                stroke_width: 1,
                filled: false,
            }
        )))?;

        // Draw smartphone point
        chart.draw_series(std::iter::once(Circle::new(
            (data.x, data.y), 
            5.0, 
            ShapeStyle {
                color: RED.mix(0.7),
                stroke_width: 2,
                filled: true,
            }
        )))?;
    }

    // Plot approximated base station positions
    for (ci, pos) in approximated_positions {
        if let Some((x, y)) = pos {
            chart.draw_series(std::iter::once(Cross::new(
                (*x, *y), 
                10, 
                ShapeStyle {
                    color: GREEN.mix(0.7),
                    stroke_width: 2,
                    filled: true,
                }
            )))?;
        }
    }

    // Ensure the plot is drawn
    root.present()?;

    Ok(())
}