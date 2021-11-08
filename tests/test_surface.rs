use plotpy::{Plot, StrError, Surface};
use russell_lab::generate3d;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_surface() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface
        .set_row_stride(1)
        .set_col_stride(1)
        .set_with_wireframe(true)
        .set_colormap_name("Pastel1")
        .set_with_colorbar(true)
        .set_colorbar_label("temperature")
        .set_number_format_cb("%.1f")
        .set_line_color("#1862ab")
        .set_line_style(":")
        .set_line_width(0.75);

    // draw surface
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface.draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1650);
    Ok(())
}

#[test]
fn test_wireframe() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_surface(false).set_with_wireframe(true);

    // draw wireframe
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
    surface.draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_wireframe.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 890);
    Ok(())
}

#[test]
fn test_cylinder() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_colormap(false);
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[5.0, 0.0, 0.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[0.0, 5.0, 0.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[0.0, 0.0, 5.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[5.0, 5.0, 5.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[5.0, 5.0, 0.0], &[5.0, 5.0, 5.0], 0.5, 1, 20)?;

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_cylinder.svg");
    plot.set_range_3d(-1.0, 6.0, -1.0, 6.0, -1.0, 6.0);
    plot.save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1340);
    Ok(())
}
