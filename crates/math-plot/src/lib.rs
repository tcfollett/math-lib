#![forbid(unsafe_code)]
#![doc = "Static plotting for functions, datasets, and histogram-style summaries."]

use math_core::{MathError, MathResult};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const BLUE: Self = Self(31, 119, 180);
    pub const RED: Self = Self(214, 39, 40);
    pub const GREEN: Self = Self(44, 160, 44);
    pub const ORANGE: Self = Self(255, 127, 14);
    pub const PURPLE: Self = Self(148, 103, 189);
    pub const GRAY: Self = Self(120, 120, 120);

    fn css(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }

    #[cfg(feature = "png")]
    fn rgba(self) -> image::Rgba<u8> {
        image::Rgba([self.0, self.1, self.2, 255])
    }
}

#[derive(Debug, Clone)]
pub struct Plot {
    title: Option<String>,
    x_label: String,
    y_label: String,
    width: u32,
    height: u32,
    x_range: Option<(f64, f64)>,
    y_range: Option<(f64, f64)>,
    series: Vec<Series>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlotOutput {
    pub svg: String,
}

#[derive(Debug, Clone)]
enum Series {
    Line {
        label: Option<String>,
        points: Vec<(f64, f64)>,
        color: Color,
    },
    Scatter {
        label: Option<String>,
        points: Vec<(f64, f64)>,
        color: Color,
        radius: f64,
    },
    Histogram {
        label: Option<String>,
        data: Vec<f64>,
        bins: usize,
        color: Color,
    },
}

#[derive(Debug, Clone)]
enum PreparedSeries {
    Line {
        label: Option<String>,
        points: Vec<(f64, f64)>,
        color: Color,
    },
    Scatter {
        label: Option<String>,
        points: Vec<(f64, f64)>,
        color: Color,
        radius: f64,
    },
    Histogram {
        label: Option<String>,
        bars: Vec<HistogramBar>,
        color: Color,
    },
}

#[derive(Debug, Clone)]
struct HistogramBar {
    left: f64,
    right: f64,
    height: f64,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: f64,
    max: f64,
}

impl Plot {
    pub fn new() -> Self {
        Self {
            title: None,
            x_label: "x".to_string(),
            y_label: "y".to_string(),
            width: 960,
            height: 640,
            x_range: None,
            y_range: None,
            series: Vec::new(),
        }
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn x_label(mut self, value: impl Into<String>) -> Self {
        self.x_label = value.into();
        self
    }

    pub fn y_label(mut self, value: impl Into<String>) -> Self {
        self.y_label = value.into();
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width.max(320);
        self.height = height.max(240);
        self
    }

    pub fn x_range(mut self, min: f64, max: f64) -> Self {
        self.x_range = Some((min, max));
        self
    }

    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some((min, max));
        self
    }

    pub fn add_line_series(
        mut self,
        label: impl Into<String>,
        points: Vec<(f64, f64)>,
        color: Color,
    ) -> Self {
        self.series.push(Series::Line {
            label: Some(label.into()),
            points,
            color,
        });
        self
    }

    pub fn add_scatter_series(
        mut self,
        label: impl Into<String>,
        points: Vec<(f64, f64)>,
        color: Color,
    ) -> Self {
        self.series.push(Series::Scatter {
            label: Some(label.into()),
            points,
            color,
            radius: 4.0,
        });
        self
    }

    pub fn add_histogram(
        mut self,
        label: impl Into<String>,
        data: Vec<f64>,
        bins: usize,
        color: Color,
    ) -> Self {
        self.series.push(Series::Histogram {
            label: Some(label.into()),
            data,
            bins: bins.max(1),
            color,
        });
        self
    }

    pub fn add_function<F>(
        mut self,
        label: impl Into<String>,
        f: F,
        lower: f64,
        upper: f64,
        samples: usize,
        color: Color,
    ) -> MathResult<Self>
    where
        F: Fn(f64) -> f64,
    {
        if samples < 2 {
            return Err(MathError::InvalidInput {
                context: "Plot::add_function",
                message: "samples must be at least 2".to_string(),
            });
        }
        if lower >= upper {
            return Err(MathError::InvalidRange {
                context: "Plot::add_function",
                message: "lower bound must be strictly less than upper bound".to_string(),
            });
        }

        let step = (upper - lower) / (samples - 1) as f64;
        let points = (0..samples)
            .map(|index| {
                let x = lower + index as f64 * step;
                (x, f(x))
            })
            .collect();
        self.series.push(Series::Line {
            label: Some(label.into()),
            points,
            color,
        });
        Ok(self)
    }

    pub fn render_svg(&self) -> MathResult<PlotOutput> {
        if self.series.is_empty() {
            return Err(MathError::EmptyInput {
                context: "plot render",
            });
        }

        let prepared = self.prepare_series()?;
        let x_range = self.resolve_x_range(&prepared)?;
        let y_range = self.resolve_y_range(&prepared)?;
        let legend_count = prepared
            .iter()
            .filter(|series| series.label().is_some())
            .count() as f64;

        let margin_left = 88.0;
        let margin_right = if legend_count > 0.0 { 180.0 } else { 40.0 };
        let margin_top = if self.title.is_some() { 70.0 } else { 42.0 };
        let margin_bottom = 84.0;
        let plot_width = self.width as f64 - margin_left - margin_right;
        let plot_height = self.height as f64 - margin_top - margin_bottom;

        let scale_x =
            |x: f64| margin_left + (x - x_range.min) / (x_range.max - x_range.min) * plot_width;
        let scale_y = |y: f64| {
            margin_top + plot_height - (y - y_range.min) / (y_range.max - y_range.min) * plot_height
        };

        let mut svg = String::new();
        writeln!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            self.width, self.height, self.width, self.height
        )
        .unwrap();
        writeln!(
            svg,
            r##"<rect width="100%" height="100%" fill="#fbfbfd"/>"##
        )
        .unwrap();
        writeln!(
            svg,
            r##"<rect x="{margin_left}" y="{margin_top}" width="{plot_width}" height="{plot_height}" fill="white" stroke="#d0d6df"/>"##
        )
        .unwrap();

        for tick in 0..=5 {
            let ratio = tick as f64 / 5.0;
            let x = margin_left + ratio * plot_width;
            let y = margin_top + ratio * plot_height;
            let x_value = x_range.min + ratio * (x_range.max - x_range.min);
            let y_value = y_range.max - ratio * (y_range.max - y_range.min);

            writeln!(
                svg,
                r##"<line x1="{x:.2}" y1="{margin_top:.2}" x2="{x:.2}" y2="{:.2}" stroke="#eef2f6"/>"##,
                margin_top + plot_height
            )
            .unwrap();
            writeln!(
                svg,
                r##"<line x1="{margin_left:.2}" y1="{y:.2}" x2="{:.2}" y2="{y:.2}" stroke="#eef2f6"/>"##,
                margin_left + plot_width
            )
            .unwrap();
            writeln!(
                svg,
                r##"<text x="{x:.2}" y="{:.2}" text-anchor="middle" font-size="12" fill="#516071">{}</text>"##,
                margin_top + plot_height + 26.0,
                format_number(x_value)
            )
            .unwrap();
            writeln!(
                svg,
                r##"<text x="{:.2}" y="{:.2}" text-anchor="end" dominant-baseline="middle" font-size="12" fill="#516071">{}</text>"##,
                margin_left - 12.0,
                y,
                format_number(y_value)
            )
            .unwrap();
        }

        for series in &prepared {
            match series {
                PreparedSeries::Line { points, color, .. } => {
                    let path = points
                        .iter()
                        .map(|(x, y)| format!("{:.2},{:.2}", scale_x(*x), scale_y(*y)))
                        .collect::<Vec<_>>()
                        .join(" ");
                    writeln!(
                        svg,
                        r#"<polyline fill="none" stroke="{}" stroke-width="2.5" points="{}"/>"#,
                        color.css(),
                        path
                    )
                    .unwrap();
                }
                PreparedSeries::Scatter {
                    points,
                    color,
                    radius,
                    ..
                } => {
                    for (x, y) in points {
                        writeln!(
                            svg,
                            r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{}" fill-opacity="0.85"/>"#,
                            scale_x(*x),
                            scale_y(*y),
                            radius,
                            color.css()
                        )
                        .unwrap();
                    }
                }
                PreparedSeries::Histogram { bars, color, .. } => {
                    for bar in bars {
                        let x = scale_x(bar.left);
                        let right = scale_x(bar.right);
                        let y = scale_y(bar.height);
                        let width = (right - x).max(1.0);
                        let height = (margin_top + plot_height - y).max(0.0);
                        writeln!(
                            svg,
                            r#"<rect x="{x:.2}" y="{y:.2}" width="{width:.2}" height="{height:.2}" fill="{}" fill-opacity="0.6" stroke="{}"/>"#,
                            color.css(),
                            color.css()
                        )
                        .unwrap();
                    }
                }
            }
        }

        writeln!(
            svg,
            r##"<line x1="{margin_left:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#5a6470" stroke-width="1.5"/>"##,
            margin_top + plot_height,
            margin_left + plot_width,
            margin_top + plot_height
        )
        .unwrap();
        writeln!(
            svg,
            r##"<line x1="{margin_left:.2}" y1="{margin_top:.2}" x2="{margin_left:.2}" y2="{:.2}" stroke="#5a6470" stroke-width="1.5"/>"##,
            margin_top + plot_height
        )
        .unwrap();

        if let Some(title) = &self.title {
            writeln!(
                svg,
                r##"<text x="{}" y="36" text-anchor="middle" font-size="26" font-weight="600" fill="#12263a">{}</text>"##,
                self.width as f64 / 2.0,
                escape_xml(title)
            )
            .unwrap();
        }
        writeln!(
            svg,
            r##"<text x="{}" y="{}" text-anchor="middle" font-size="16" fill="#334556">{}</text>"##,
            margin_left + plot_width / 2.0,
            self.height as f64 - 24.0,
            escape_xml(&self.x_label)
        )
        .unwrap();
        writeln!(
            svg,
            r##"<text x="24" y="{}" transform="rotate(-90 24,{})" text-anchor="middle" font-size="16" fill="#334556">{}</text>"##,
            margin_top + plot_height / 2.0,
            margin_top + plot_height / 2.0,
            escape_xml(&self.y_label)
        )
        .unwrap();

        if legend_count > 0.0 {
            let legend_x = margin_left + plot_width + 24.0;
            let mut legend_y = margin_top + 12.0;
            for series in &prepared {
                if let Some(label) = series.label() {
                    writeln!(
                        svg,
                        r#"<rect x="{legend_x:.2}" y="{legend_y:.2}" width="18" height="10" rx="3" fill="{}"/>"#,
                        series.color().css()
                    )
                    .unwrap();
                    writeln!(
                        svg,
                        r##"<text x="{:.2}" y="{:.2}" font-size="13" fill="#334556">{}</text>"##,
                        legend_x + 28.0,
                        legend_y + 9.0,
                        escape_xml(label)
                    )
                    .unwrap();
                    legend_y += 22.0;
                }
            }
        }

        writeln!(svg, "</svg>").unwrap();
        Ok(PlotOutput { svg })
    }

    pub fn save_svg(&self, path: impl AsRef<Path>) -> MathResult<()> {
        let output = self.render_svg()?;
        fs::write(path, output.svg)?;
        Ok(())
    }

    #[cfg(feature = "png")]
    pub fn save_png(&self, path: impl AsRef<Path>) -> MathResult<()> {
        use image::{Rgba, RgbaImage};

        let prepared = self.prepare_series()?;
        let x_range = self.resolve_x_range(&prepared)?;
        let y_range = self.resolve_y_range(&prepared)?;

        let margin_left = 88.0;
        let margin_right = 40.0;
        let margin_top = if self.title.is_some() { 70.0 } else { 42.0 };
        let margin_bottom = 84.0;
        let plot_width = self.width as f64 - margin_left - margin_right;
        let plot_height = self.height as f64 - margin_top - margin_bottom;

        let scale_x =
            |x: f64| margin_left + (x - x_range.min) / (x_range.max - x_range.min) * plot_width;
        let scale_y = |y: f64| {
            margin_top + plot_height - (y - y_range.min) / (y_range.max - y_range.min) * plot_height
        };

        let mut image = RgbaImage::from_pixel(self.width, self.height, Rgba([251, 251, 253, 255]));
        draw_rect(
            &mut image,
            margin_left as i32,
            margin_top as i32,
            plot_width as i32,
            plot_height as i32,
            Rgba([255, 255, 255, 255]),
        );

        for series in prepared {
            match series {
                PreparedSeries::Line { points, color, .. } => {
                    for window in points.windows(2) {
                        let (x0, y0) = window[0];
                        let (x1, y1) = window[1];
                        draw_line(
                            &mut image,
                            scale_x(x0) as i32,
                            scale_y(y0) as i32,
                            scale_x(x1) as i32,
                            scale_y(y1) as i32,
                            color.rgba(),
                        );
                    }
                }
                PreparedSeries::Scatter {
                    points,
                    color,
                    radius,
                    ..
                } => {
                    for (x, y) in points {
                        draw_circle(
                            &mut image,
                            scale_x(x) as i32,
                            scale_y(y) as i32,
                            radius as i32,
                            color.rgba(),
                        );
                    }
                }
                PreparedSeries::Histogram { bars, color, .. } => {
                    for bar in bars {
                        let x = scale_x(bar.left) as i32;
                        let right = scale_x(bar.right) as i32;
                        let y = scale_y(bar.height) as i32;
                        draw_rect(
                            &mut image,
                            x,
                            y,
                            (right - x).max(1),
                            ((margin_top + plot_height) as i32 - y).max(0),
                            color.rgba(),
                        );
                    }
                }
            }
        }

        draw_line(
            &mut image,
            margin_left as i32,
            (margin_top + plot_height) as i32,
            (margin_left + plot_width) as i32,
            (margin_top + plot_height) as i32,
            Color::GRAY.rgba(),
        );
        draw_line(
            &mut image,
            margin_left as i32,
            margin_top as i32,
            margin_left as i32,
            (margin_top + plot_height) as i32,
            Color::GRAY.rgba(),
        );

        image
            .save(path)
            .map_err(|error| MathError::Io(error.to_string()))
    }

    fn prepare_series(&self) -> MathResult<Vec<PreparedSeries>> {
        let mut prepared = Vec::with_capacity(self.series.len());
        for series in &self.series {
            match series {
                Series::Line {
                    label,
                    points,
                    color,
                } => {
                    validate_points(points, "line series")?;
                    prepared.push(PreparedSeries::Line {
                        label: label.clone(),
                        points: points.clone(),
                        color: *color,
                    });
                }
                Series::Scatter {
                    label,
                    points,
                    color,
                    radius,
                } => {
                    validate_points(points, "scatter series")?;
                    prepared.push(PreparedSeries::Scatter {
                        label: label.clone(),
                        points: points.clone(),
                        color: *color,
                        radius: *radius,
                    });
                }
                Series::Histogram {
                    label,
                    data,
                    bins,
                    color,
                } => {
                    if data.is_empty() {
                        return Err(MathError::EmptyInput {
                            context: "histogram series",
                        });
                    }
                    if data.iter().any(|value| !value.is_finite()) {
                        return Err(MathError::InvalidInput {
                            context: "histogram series",
                            message: "all histogram values must be finite".to_string(),
                        });
                    }
                    prepared.push(PreparedSeries::Histogram {
                        label: label.clone(),
                        bars: histogram_bins(data, *bins),
                        color: *color,
                    });
                }
            }
        }
        Ok(prepared)
    }

    fn resolve_x_range(&self, series: &[PreparedSeries]) -> MathResult<Range> {
        if let Some((min, max)) = self.x_range {
            return normalize_range(min, max, "plot x-range");
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for series in series {
            match series {
                PreparedSeries::Line { points, .. } | PreparedSeries::Scatter { points, .. } => {
                    for (x, _) in points {
                        min = min.min(*x);
                        max = max.max(*x);
                    }
                }
                PreparedSeries::Histogram { bars, .. } => {
                    for bar in bars {
                        min = min.min(bar.left);
                        max = max.max(bar.right);
                    }
                }
            }
        }
        normalize_range(min, max, "plot x-range")
    }

    fn resolve_y_range(&self, series: &[PreparedSeries]) -> MathResult<Range> {
        if let Some((min, max)) = self.y_range {
            return normalize_range(min, max, "plot y-range");
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for series in series {
            match series {
                PreparedSeries::Line { points, .. } | PreparedSeries::Scatter { points, .. } => {
                    for (_, y) in points {
                        min = min.min(*y);
                        max = max.max(*y);
                    }
                }
                PreparedSeries::Histogram { bars, .. } => {
                    min = min.min(0.0);
                    for bar in bars {
                        max = max.max(bar.height);
                    }
                }
            }
        }
        normalize_range(min, max, "plot y-range")
    }
}

impl Default for Plot {
    fn default() -> Self {
        Self::new()
    }
}

impl PlotOutput {
    pub fn save_svg(&self, path: impl AsRef<Path>) -> MathResult<()> {
        fs::write(path, &self.svg)?;
        Ok(())
    }
}

impl PreparedSeries {
    fn label(&self) -> Option<&str> {
        match self {
            Self::Line { label, .. }
            | Self::Scatter { label, .. }
            | Self::Histogram { label, .. } => label.as_deref(),
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Line { color, .. }
            | Self::Scatter { color, .. }
            | Self::Histogram { color, .. } => *color,
        }
    }
}

fn validate_points(points: &[(f64, f64)], context: &'static str) -> MathResult<()> {
    if points.is_empty() {
        return Err(MathError::EmptyInput { context });
    }
    if points.iter().any(|(x, y)| !x.is_finite() || !y.is_finite()) {
        return Err(MathError::InvalidInput {
            context,
            message: "all coordinates must be finite".to_string(),
        });
    }
    Ok(())
}

fn histogram_bins(data: &[f64], bins: usize) -> Vec<HistogramBar> {
    let min = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let width = if (max - min).abs() <= f64::EPSILON {
        1.0
    } else {
        (max - min) / bins as f64
    };
    let mut counts = vec![0usize; bins];

    for value in data {
        let mut index = ((value - min) / width).floor() as usize;
        if index >= bins {
            index = bins - 1;
        }
        counts[index] += 1;
    }

    (0..bins)
        .map(|index| HistogramBar {
            left: min + index as f64 * width,
            right: min + (index + 1) as f64 * width,
            height: counts[index] as f64,
        })
        .collect()
}

fn normalize_range(min: f64, max: f64, context: &'static str) -> MathResult<Range> {
    if !min.is_finite() || !max.is_finite() {
        return Err(MathError::InvalidInput {
            context,
            message: "range endpoints must be finite".to_string(),
        });
    }
    if min > max {
        return Err(MathError::InvalidRange {
            context,
            message: "minimum must be less than or equal to maximum".to_string(),
        });
    }
    if (max - min).abs() <= f64::EPSILON {
        let padding = if min.abs() < 1.0 {
            1.0
        } else {
            min.abs() * 0.1
        };
        return Ok(Range {
            min: min - padding,
            max: max + padding,
        });
    }

    let padding = (max - min) * 0.05;
    Ok(Range {
        min: min - padding,
        max: max + padding,
    })
}

fn format_number(value: f64) -> String {
    if value.abs() >= 1_000.0 || (value.abs() > 0.0 && value.abs() < 0.01) {
        format!("{value:.2e}")
    } else {
        format!("{value:.2}")
    }
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(feature = "png")]
fn draw_line(
    image: &mut image::RgbaImage,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: image::Rgba<u8>,
) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        put_pixel(image, x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            error += dy;
            x0 += sx;
        }
        if e2 <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

#[cfg(feature = "png")]
fn draw_circle(
    image: &mut image::RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: image::Rgba<u8>,
) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                put_pixel(image, center_x + x, center_y + y, color);
            }
        }
    }
}

#[cfg(feature = "png")]
fn draw_rect(
    image: &mut image::RgbaImage,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: image::Rgba<u8>,
) {
    for py in y..(y + height) {
        for px in x..(x + width) {
            put_pixel(image, px, py, color);
        }
    }
}

#[cfg(feature = "png")]
fn put_pixel(image: &mut image::RgbaImage, x: i32, y: i32, color: image::Rgba<u8>) {
    if x >= 0 && y >= 0 && (x as u32) < image.width() && (y as u32) < image.height() {
        image.put_pixel(x as u32, y as u32, color);
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Plot};

    #[test]
    fn svg_output_contains_expected_shapes() {
        let plot = Plot::new()
            .title("Example")
            .x_label("Time")
            .y_label("Value")
            .add_line_series("line", vec![(0.0, 0.0), (1.0, 1.0)], Color::BLUE)
            .add_scatter_series("points", vec![(0.5, 0.5)], Color::RED)
            .add_histogram("hist", vec![0.1, 0.2, 0.4, 0.5], 2, Color::GREEN);

        let svg = plot.render_svg().unwrap().svg;
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<polyline"));
        assert!(svg.contains("<circle"));
        assert!(svg.contains("<rect"));
        assert!(svg.contains("Example"));
    }
}
