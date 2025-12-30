/* line_charts.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Linear charts

/********************************************************
 *               WARNING WARNING WARNING                *
 ********************************************************
 * To implement the graphs of the system monitor, I     *
 * used the iced_aksel crate.                           *
 *                                                      *
 * The main part of the source code from here is taken  *
 * from the `dashboard` example:                        *
 * https://github.com/QuistHQ/iced_aksel/tree/main/examples/dashboard *
 ********************************************************
 * NOTE: refactoring!                                   *
 * NOTE: move the graph structures to the `widgets`     *
 * module.                                              *
 ********************************************************
 *            END WARNING WARNING WARNING               *
 ********************************************************/

use iced::{
    Color, Pixels, Theme,
    alignment::{Horizontal, Vertical},
    time::Instant,
};
use iced_aksel::{
    Axis, Chart, Measure, PlotPoint, State, Stroke,
    axis::{self, TickLine, TickResult},
    plot::{Plot, PlotData},
    scale::Linear,
    shape::{Area, Label, Polygon, Polyline, Rectangle},
};
use std::collections::{HashMap, VecDeque};

type AxisID = String;

#[derive(Debug, Clone)]
pub struct LineSeries {
    pub name: String,
    pub current_values: VecDeque<f64>,
    pub target_values: VecDeque<f64>,
    pub y_key: String,
    pub color: Color,
    pub width: f32,
    pub show_markers: bool,
    pub fill_color: Option<Color>,
    pub max_displayed_values: usize,
}

impl LineSeries {
    pub fn new(name: impl Into<String>, color: Color, mx: usize) -> Self {
        Self {
            name: name.into(),
            current_values: VecDeque::new(),
            target_values: VecDeque::new(),
            y_key: "Y".to_string(),
            color,
            width: 1.5,
            show_markers: false,
            fill_color: None,
            max_displayed_values: mx,
        }
    }

    pub fn set_max_values(&mut self, mx: usize) {
        self.max_displayed_values = mx;
    }

    pub fn axis(mut self, y_id: impl Into<String>) -> Self {
        self.y_key = y_id.into();
        self
    }

    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub const fn markers(mut self, show: bool) -> Self {
        self.show_markers = show;
        self
    }

    pub const fn fill(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn push(&mut self, val: f64) {
        let start_val = self.current_values.iter().last().copied().unwrap_or(0.);
        self.current_values.push_back(start_val);
        self.target_values.push_back(val);

        if self.current_values.len() >= self.max_displayed_values + 1 {
            self.current_values.pop_front();
        }
        if self.target_values.len() >= self.max_displayed_values + 1 {
            self.target_values.pop_front();
        }
    }

    pub fn extend(&mut self, vals: impl IntoIterator<Item = f64>) {
        for v in vals {
            self.push(v);
        }
    }

    fn tick(&mut self, alpha: f64) {
        if self.current_values.len() < self.target_values.len() {
            self.current_values.resize(self.target_values.len(), 0.);
        }
        for (cur, tgt) in self
            .current_values
            .iter_mut()
            .zip(self.target_values.iter())
        {
            let diff = *tgt - *cur;
            if diff.abs() > 1e-5 {
                *cur += diff * alpha;
            } else {
                *cur = *tgt;
            }
        }
    }

    fn snap(&mut self) {
        self.current_values = self.target_values.clone();
    }
}

/// Line chart
#[derive(Debug)]
pub struct LineChart {
    state: State<AxisID, f64>,
    series: Vec<LineSeries>,
    labels: Vec<String>,
    defined_axes: Vec<String>,
    show_legend: bool,

    /********************
     *     Animation    *
     ********************/
    animation_speed: Option<f64>,
    last_tick: Option<Instant>,

    /********************
     *  Animated state  *
     ********************/
    current_stack_factor: f64,
    target_stack_factor: f64,

    fill_enabled: bool,
    current_fill_alpha: f32,
    target_fill_alpha: f32,

    current_x_domain: (f64, f64),
    current_y_domains: HashMap<String, (f64, f64)>,

    max_values: usize,
    max_y_val: Option<f64>,
}

impl LineChart {
    pub const X: &'static str = "X";
    pub const Y: &'static str = "Y";

    pub fn new() -> Self {
        Self {
            state: State::new(),
            series: vec![],
            labels: vec![],
            defined_axes: vec![],
            show_legend: true,
            animation_speed: None,
            last_tick: None,
            current_stack_factor: 0.,
            target_stack_factor: 0.,
            fill_enabled: false,
            current_fill_alpha: 0.,
            target_fill_alpha: 0.2,
            current_x_domain: (0.0, 1.0),
            current_y_domains: HashMap::new(),
            max_values: 50,
            max_y_val: None,
        }
    }

    pub fn max_values(mut self, mx: usize) -> Self {
        self.max_values = mx;
        for i in &mut self.series {
            i.set_max_values(self.max_values);
        }
        self
    }

    pub fn set_max_values(&mut self, mx: usize) {
        self.max_values = mx;
        for i in &mut self.series {
            i.set_max_values(self.max_values);
        }
    }

    pub fn set_max_y(&mut self, y: f64) {
        self.max_y_val = Some(y);
    }

    pub fn with_default_axes(mut self) -> Self {
        self.with_axis(
            Self::X,
            Axis::new(Linear::new(0., 1.), axis::Position::Bottom),
        );
        self.with_axis(Self::Y, y_axis(0., 1.));
        self
    }

    pub const fn animated(mut self, speed: f64) -> Self {
        self.animation_speed = Some(speed.max(0.0).min(1.0));
        self
    }

    pub const fn legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn fill_alpha(mut self, alpha: f32) -> Self {
        self.target_fill_alpha = alpha.clamp(0., 1.);
        // If already enabled, we might need to update current immediately
        // if not animating
        if self.fill_enabled && self.animation_speed.is_none() {
            self.current_fill_alpha = self.target_fill_alpha;
            self.update_series_fill();
        }
        self
    }

    pub fn toggle_fill(&mut self) {
        self.fill_enabled = !self.fill_enabled;
        let target = if self.fill_enabled {
            self.target_fill_alpha
        } else {
            0.
        };
        if self.animation_speed.is_none() {
            self.current_fill_alpha = target;
            self.update_series_fill();
        }
    }

    fn update_series_fill(&mut self) {
        for s in &mut self.series {
            let mut color = s.color;
            color.a = self.current_fill_alpha;
            s.fill_color = if self.current_fill_alpha > 0. {
                Some(color)
            } else {
                None
            };
        }
    }

    pub fn stacked(mut self, stacked: bool) -> Self {
        self.set_stacked(stacked);
        self
    }

    pub fn set_stacked(&mut self, stacked: bool) {
        self.target_stack_factor = if stacked { 1. } else { 0. };
        // Link Fill Enabled to Stacked State
        self.fill_enabled = stacked;

        if self.animation_speed.is_none() {
            self.current_stack_factor = self.target_stack_factor;
            self.current_fill_alpha = if self.fill_enabled {
                self.target_fill_alpha
            } else {
                0.
            };
            self.update_series_fill();
            self.snap_axes();
        }
    }

    pub fn toggle_stacked(&mut self) {
        let new_stacked_state = self.target_stack_factor <= 0.5;
        self.set_stacked(new_stacked_state);
    }

    pub fn with_axis(&mut self, id: impl Into<String>, axis: Axis<f64>) {
        let key = id.into();
        self.state.set_axis(key.clone(), axis);
        if !self.defined_axes.contains(&key) {
            self.defined_axes.push(key);
        }
    }

    pub fn push_series(&mut self, mut series: LineSeries) {
        if self.current_fill_alpha > 0.0 {
            let mut color = series.color;
            color.a = self.current_fill_alpha;
            series.fill_color = Some(color);
        }
        self.ensure_axes_exist(&series);
        self.series.push(series);
        if self.animation_speed.is_none() {
            self.snap_axes();
        }
    }

    pub fn clear(&mut self) {
        self.series.clear();
        self.labels.clear();
        self.snap_axes();
    }

    pub fn get_last(&self) -> Option<&LineSeries> {
        self.series.last()
    }

    pub fn tick(&mut self, now: Instant) {
        let Some(speed_normalized) = self.animation_speed else {
            return;
        };

        let dt = self
            .last_tick
            .map_or(0.0, |last| (now - last).as_secs_f32() as f64);
        self.last_tick = Some(now);

        let physics_speed = speed_normalized * 10.0;
        let alpha = 1.0 - (-physics_speed * dt).exp();

        // 1. Calculate Targets
        let (target_x, target_ys) = self.calculate_targets();

        // 2. Animate Axes
        let next_x0 =
            (target_x.0 - self.current_x_domain.0).mul_add(alpha, self.current_x_domain.0);
        let next_x1 =
            (target_x.1 - self.current_x_domain.1).mul_add(alpha, self.current_x_domain.1);

        self.current_x_domain = (next_x0, next_x1);

        if let Some(axis) = self.state.axis_mut_opt(&Self::X.to_string()) {
            axis.set_domain(self.current_x_domain.0, self.current_x_domain.1);
        }

        for (id, target) in target_ys {
            let current = self.current_y_domains.entry(id.clone()).or_insert(target);
            current.0 += (target.0 - current.0) * alpha;
            current.1 += (target.1 - current.1) * alpha;

            if let Some(axis) = self.state.axis_mut_opt(&id) {
                axis.set_domain(current.0, current.1);
            }
        }

        // 3. Animate Content
        for s in &mut self.series {
            s.tick(alpha);
        }

        // Animate Stacking Factor
        let diff_stack = self.target_stack_factor - self.current_stack_factor;
        if diff_stack.abs() > 1e-5 {
            self.current_stack_factor += diff_stack * alpha;
        } else {
            self.current_stack_factor = self.target_stack_factor;
        }

        // 4. Animate Fill Alpha
        let target_alpha = if self.fill_enabled {
            self.target_fill_alpha
        } else {
            0.0
        };
        let diff_alpha = target_alpha - self.current_fill_alpha;
        if diff_alpha.abs() > 1e-5 {
            self.current_fill_alpha += diff_alpha * (alpha as f32);
            self.update_series_fill();
        } else if self.current_fill_alpha != target_alpha {
            self.current_fill_alpha = target_alpha;
            self.update_series_fill();
        }
    }

    fn snap_axes(&mut self) {
        let (tx, tys) = self.calculate_targets();
        self.current_x_domain = tx;
        self.current_y_domains = tys;

        if let Some(axis) = self.state.axis_mut_opt(&Self::X.to_string()) {
            axis.set_domain(tx.0, tx.1);
        }
        for (id, d) in &self.current_y_domains {
            if let Some(axis) = self.state.axis_mut_opt(id) {
                axis.set_domain(d.0, d.1);
            }
        }
    }

    fn calculate_targets(&self) -> ((f64, f64), HashMap<String, (f64, f64)>) {
        if self.series.is_empty() {
            return ((0.0, 1.0), HashMap::new());
        }

        let max_len = self
            .series
            .iter()
            .map(|s| s.target_values.len())
            .max()
            .unwrap_or(0);
        let x_max = (max_len as f64 - 1.0).max(0.0);
        let target_x = (0.0, x_max);

        let mut target_ys = HashMap::new();
        let mut stacked_sums: HashMap<String, Vec<f64>> = HashMap::new();
        let factor = self.target_stack_factor;

        for s in &self.series {
            let sums = stacked_sums.entry(s.y_key.clone()).or_default();
            if s.target_values.len() > sums.len() {
                sums.resize(s.target_values.len(), 0.0);
            }

            let entry = target_ys
                .entry(s.y_key.clone())
                .or_insert((f64::MAX, f64::MIN));

            for (i, &val) in s.target_values.iter().enumerate() {
                let baseline = sums[i];
                let effective_val = baseline.mul_add(factor, val);
                entry.0 = entry.0.min(effective_val);
                entry.1 = entry.1.max(effective_val);
                sums[i] += val;
            }
        }

        for (_, bounds) in target_ys.iter_mut() {
            let (min, max) = match self.max_y_val {
                Some(max_y_val) => (bounds.0, max_y_val),
                None => *bounds,
            };
            let padding = if max > min { (max - min) * 0.05 } else { 1.0 };
            let final_min = if factor > 0.1 {
                min.min(0.0)
            } else {
                if min < 0. { min } else { 0.0 }
            };
            *bounds = (final_min, max + padding);
        }

        (target_x, target_ys)
    }

    pub fn push(&mut self, label: impl Into<String>, value: f64) {
        let label = label.into();
        if self.series.is_empty() {
            let default_series =
                LineSeries::new("Data", Color::from_rgb(0.2, 0.4, 0.8), self.max_values);
            self.push_series(default_series);
        }

        let needs_label_update = if let Some(last) = self.series.last() {
            last.target_values.len() >= self.labels.len()
        } else {
            false
        };

        if needs_label_update {
            self.labels.push(label);
            self.update_x_axis_labels();
        }

        if let Some(last) = self.series.last_mut() {
            last.push(value);
        }

        if self.animation_speed.is_none() {
            for s in &mut self.series {
                s.current_values = s.target_values.clone();
            }
            self.snap_axes();
        }
    }

    pub fn push_value(&mut self, value: f64) {
        self.push("", value);
    }

    pub fn push_to(&mut self, index: usize, label: impl Into<String>, value: f64) {
        let needs_label_update = if let Some(series) = self.series.get(index) {
            series.target_values.len() >= self.labels.len()
        } else {
            false
        };

        if needs_label_update {
            self.labels.push(label.into());
            self.update_x_axis_labels();
        }

        if let Some(series) = self.series.get_mut(index) {
            series.push(value);
        }

        if self.animation_speed.is_none() {
            if let Some(s) = self.series.get_mut(index) {
                s.snap();
            }
            self.snap_axes();
        }
    }

    pub fn push_value_to(&mut self, index: usize, value: f64) {
        self.push_to(index, "", value);
    }

    pub fn push_value_last_series(&mut self, value: f64) {
        self.push_value(value);
    }

    pub const fn series_count(&self) -> usize {
        self.series.len()
    }

    fn update_x_axis_labels(&mut self) {
        let labels = self.labels.clone();
        let x_axis = self.state.axis_mut(&Self::X.to_string());

        x_axis.set_tick_renderer(move |ctx| {
            let result = TickResult::new();
            let idx = ctx.tick.value.round();
            if (ctx.tick.value - idx).abs() > 0.001 {
                return result;
            }
            let idx = idx as usize;
            if idx < labels.len() {
                return result
                    .label(labels[idx].clone())
                    .tick_line(TickLine::default());
            }

            result
        });
    }

    fn ensure_axes_exist(&mut self, series: &LineSeries) {
        let x_key = Self::X.to_string();
        if !self.defined_axes.contains(&x_key) {
            self.state.set_axis(
                x_key.clone(),
                Axis::new(Linear::new(0.0, 1.0), axis::Position::Bottom),
            );
            self.defined_axes.push(x_key);
            self.update_x_axis_labels();
        }

        if !self.defined_axes.contains(&series.y_key) {
            self.state.set_axis(series.y_key.clone(), y_axis(0., 1.));
            self.defined_axes.push(series.y_key.clone());
        }
    }

    pub fn chart<Message>(&self) -> Chart<'_, AxisID, f64, Message> {
        let mut chart = Chart::new(&self.state);
        let first_y = self
            .series
            .first()
            .map(|s| s.y_key.clone())
            .unwrap_or_else(|| Self::Y.to_string());
        chart = chart.plot_data(self, Self::X.to_string(), first_y);
        chart
    }
}

impl PlotData<f64> for LineChart {
    fn draw(&self, plot: &mut Plot<f64>, theme: &Theme) {
        let chart_floor = self
            .state
            .axis_opt(&Self::Y.to_string())
            .map_or(0.0, |axis| *axis.domain().0);

        let mut baseline: Vec<f64> = Vec::new();

        for s in &self.series {
            if s.current_values.len() < 2 {
                continue;
            }

            if baseline.len() < s.current_values.len() {
                baseline.resize(s.current_values.len(), 0.0);
            }

            let points: Vec<PlotPoint<f64>> = s
                .current_values
                .iter()
                .enumerate()
                .map(|(i, &v)| {
                    let effective_base = baseline[i] * self.current_stack_factor;
                    let total = effective_base + v;
                    PlotPoint::new(i as f64, total)
                })
                .collect();

            if self.current_fill_alpha > 0.0 {
                let mut fill_poly = points.clone();
                for (i, _) in s
                    .current_values
                    .iter()
                    .enumerate()
                    .take(s.current_values.len())
                    .rev()
                {
                    let base_val = baseline[i] * self.current_stack_factor;
                    let floor = chart_floor * (1.0 - self.current_stack_factor)
                        + base_val * self.current_stack_factor;
                    fill_poly.push(PlotPoint::new(i as f64, floor));
                }
                let mut color = s.color;
                color.a = self.current_fill_alpha;
                plot.add_shape(Area::new(fill_poly).fill(color));
            }

            plot.add_shape(Polyline {
                points: points.clone(),
                stroke: Some(Stroke::new(s.color, Measure::Screen(s.width))),
                extend_start: false,
                extend_end: false,
                arrow_start: false,
                arrow_end: false,
                arrow_size: 10.0,
            });

            if s.show_markers {
                for point in &points {
                    let marker_size = Measure::Screen(s.width.mul_add(2.0, 2.0));
                    plot.add_shape(Polygon::new(*point, marker_size, 4).fill(s.color));
                }
            }

            for (i, &v) in s.current_values.iter().enumerate() {
                baseline[i] += v;
            }
        }

        if self.show_legend {
            let palette = theme.palette();
            if let (Some(x_axis), Some(y_axis)) = (
                self.state.axis_opt(&Self::X.to_string()),
                self.state.axis_opt(&Self::Y.to_string()),
            ) {
                let (x_min, x_max) = x_axis.domain();
                let (y_min, y_max) = y_axis.domain();

                let max_cols = 6;
                let item_cnt = self.series.len();
                let cols_per_row = if item_cnt <= 2 { item_cnt } else { max_cols };

                let start_x = (x_max - x_min).mul_add(0.02, *x_min);
                let start_y = (y_max - y_min).mul_add(-0.05, *y_max);
                let step_y = (y_max - y_min) * 0.1;

                let avail_width = (x_max - x_min) * 0.8;
                let col_width = if cols_per_row > 0 {
                    avail_width / cols_per_row as f64
                } else {
                    avail_width
                };

                for (i, series) in self.series.iter().enumerate() {
                    let row = i / cols_per_row;
                    let col = i % cols_per_row;

                    let x_pos = start_x + (col as f64) * col_width;
                    let y_pos = start_y - (row as f64) * step_y;

                    // let y_pos = (i as f64).mul_add(-step_y, start_y);
                    plot.add_shape(
                        Rectangle::centered(
                            PlotPoint::new(x_pos, y_pos),
                            Measure::Screen(10.0),
                            Measure::Screen(10.0),
                        )
                        .fill(series.color),
                    );
                    let text_offset = (x_max - x_min) * 0.01;
                    plot.add_shape(
                        Label::new(&series.name, PlotPoint::new(x_pos + text_offset, y_pos))
                            .fill(palette.text)
                            .size(12.0)
                            .align(Horizontal::Left, Vertical::Center),
                    );
                }
            }
        }
    }
}

fn y_axis(min_y: f64, max_y: f64) -> Axis<f64> {
    Axis::new(Linear::new(min_y, max_y), axis::Position::Right).with_tick_renderer(|ctx| match ctx
        .tick
        .level
    {
        // 0 => TickResult::default().label({
        //     let mut label = iced_aksel::axis::Label::from(format!("{:.2}", ctx.tick.value));
        //     label.size = Pixels::from(10.);
        //     label
        // }),
        0 => TickResult::default().label(format!("{:.2}", ctx.tick.value)),
        _ => TickResult::new(),
    })
}
