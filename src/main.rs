extern crate plotters;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a bitmap backend for rendering the plot
    let backend = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();

    // Configure the chart settings
    let mut chart = ChartBuilder::on(&backend)
        .caption("Example Chart", ("Arial", 30).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(0..10, 0f32..100f32)?;

    // Generate some sample data
    let data = vec![(0, 0), (1, 1), (2, 4), (3, 9), (4, 16), (5, 25)];

    // Draw the line series
    chart.draw_series(LineSeries::new(data, &RED))?;

    // Save the plot to a file
    backend.present()?;
    Ok(())
}
