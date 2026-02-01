/* line_charts.rs
 *
 * Copyright 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>
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

use iced::{
    Color as IColor, Element, Size, Theme,
    widget::{
        canvas::{Cache, Frame, Geometry},
        container,
    },
};
use plotters::prelude::*;
use plotters_iced2::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::collections::VecDeque;

use crate::messages::Message;

#[derive(Debug, Clone)]
pub struct LineChart {
    data: Vec<LineSeries>,
    max_points: usize,
    style: Style,
}

#[derive(Debug, Clone)]
pub struct LineSeries {
    name: String,
    data: VecDeque<f64>,
    color: RGBColor,
    max_points: usize,
}

#[derive(Debug, Clone)]
pub struct Style {
    pub text_size: usize,
    pub text_color: IColor,
    pub y_axis_color: IColor,
    pub legend_background_color: IColor,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            text_size: 11,
            text_color: IColor::WHITE,
            y_axis_color: IColor::WHITE,
            legend_background_color: IColor::BLACK,
        }
    }
}

fn to_rgbcolor(color: IColor) -> RGBColor {
    let oc = color.into_rgba8();
    RGBColor(oc[0], oc[1], oc[2])
}

fn to_rgbacolor(color: IColor) -> RGBAColor {
    let oc = color.into_rgba8();
    RGBAColor(oc[0], oc[1], oc[2], color.a as f64)
}

impl LineSeries {
    pub fn new(name: String, color: IColor, max_len: usize) -> Self {
        Self {
            name,
            max_points: max_len,
            color: to_rgbcolor(color),
            data: VecDeque::with_capacity(max_len),
        }
    }

    pub fn push(&mut self, value: f64) {
        if self.data.len() > self.max_points {
            self.data.pop_front();
        }

        self.data.push_back(value);
    }
}

impl LineChart {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(8),
            max_points: 100,
            style: Style::default(),
        }
    }

    pub fn set_style(&mut self, theme: &Theme) {
        let style = Style {
            text_size: 12,
            text_color: theme.palette().text,
            y_axis_color: theme.palette().text,
            legend_background_color: theme.palette().background,
        };
        self.style = style;
    }

    pub fn set_max_values(&mut self, value: usize) {
        self.max_points = value;
        for s in &mut self.data {
            s.max_points = value;
        }
        self.update_axis();
    }

    pub fn series_count(&self) -> usize {
        self.data.len()
    }

    pub fn push_series(&mut self, value: LineSeries) {
        self.data.push(value);
    }

    pub fn push_to(&mut self, idx: usize, value: f64) {
        if self.data.len() < idx {
            return;
        }
        self.data[idx].push(value);
    }

    pub fn push_value(&mut self, value: f64, idx: usize) {
        if self.data.len() < idx {
            return;
        }
        self.update_axis();
        self.data[idx].data.push_back(value);
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let chart = ChartWidget::new(self);
        container(chart).into()
    }

    fn update_axis(&mut self) {
        'm: loop {
            for s in &mut self.data {
                if s.data.len() > self.max_points {
                    s.data.pop_front();
                } else {
                    break 'm;
                }
            }
        }
    }
}

impl Chart<Message> for LineChart {
    type State = ();

    #[inline]
    fn draw<R: plotters_iced2::Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        size: Size,
        f: F,
    ) -> Geometry {
        renderer.draw_cache(&Cache::new(), size, f)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .x_label_area_size(0)
            .y_label_area_size(0)
            .margin(5)
            .build_cartesian_2d(0..(self.max_points), 0.0..100.0)
            .expect("Failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(to_rgbcolor(self.style.y_axis_color).mix(0.05))
            .disable_x_axis()
            .disable_x_mesh()
            .light_line_style(TRANSPARENT)
            .y_labels(8)
            .x_labels(self.max_points)
            .draw()
            .expect("Failed to draw chart mesh");

        for series in &self.data {
            chart
                .draw_series(
                    AreaSeries::new(
                        series.data.iter().enumerate().map(|x| (x.0, *x.1 as f64)),
                        0.,
                        plotters::style::TRANSPARENT,
                    )
                    .border_style(ShapeStyle::from(series.color).stroke_width(2)),
                )
                .expect("Failed to draw chart data")
                .label(&series.name)
                .legend(|(x, y)| {
                    Rectangle::new([(x - 5, y - 3), (x + 15, y + 8)], series.color.filled())
                });
        }

        if !self.data.is_empty() {
            chart
                .configure_series_labels()
                .label_font(
                    ("sans-serif", self.style.text_size as i32)
                        .into_font()
                        .color(&to_rgbcolor(self.style.text_color)),
                )
                .position(SeriesLabelPosition::UpperRight)
                .background_style(
                    {
                        let mut color = self.style.legend_background_color;
                        color.a = 0.8;
                        to_rgbacolor(color)
                    }
                    .filled(),
                )
                .margin(10)
                .draw()
                .expect("Failed to draw chart");
        }
    }
}
