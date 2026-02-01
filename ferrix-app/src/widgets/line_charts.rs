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
        column, container, grid, row, text,
    },
};
use plotters::prelude::*;
use plotters_iced2::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::collections::VecDeque;

use crate::{messages::Message, settings::ChartLineThickness};

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
    pub y_axis_color: IColor,
    pub line_thickness: u32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            y_axis_color: IColor::WHITE,
            line_thickness: ChartLineThickness::default().to_u32(),
        }
    }
}

fn to_rgbcolor(color: IColor) -> RGBColor {
    let oc = color.into_rgba8();
    RGBColor(oc[0], oc[1], oc[2])
}

fn to_icolor(color: RGBColor) -> IColor {
    let (r, g, b) = (color.0, color.1, color.2);
    IColor::from_rgb8(r, g, b)
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
            y_axis_color: theme.palette().text,
            line_thickness: self.style.line_thickness,
        };
        self.style = style;
    }

    pub fn set_line_thickness(&mut self, thickness: ChartLineThickness) {
        self.style.line_thickness = thickness.to_u32();
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

    pub fn legend_parameters<'a>(&'a self) -> Element<'a, Message> {
        let mut items = Vec::with_capacity(self.data.len());
        let bold_font = {
            let mut font = iced::Font::default();
            font.weight = iced::font::Weight::Bold;
            font
        };

        for line in &self.data {
            let last = line.data.len() - 1;
            items.push(
                row![
                    text(format!("{}:", &line.name))
                        .color(to_icolor(line.color))
                        .font(bold_font),
                    text(format!("{:.2}%", line.data[last])),
                ]
                .spacing(3),
            );
        }

        let mut gr = grid([]).spacing(5).columns(8).fluid(125.).height(35.);
        for item in items {
            gr = gr.push(item);
        }
        container(gr).into()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let chart = ChartWidget::new(self);
        container(column![chart, self.legend_parameters()]).into()
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
                    .border_style(
                        ShapeStyle::from(series.color).stroke_width(self.style.line_thickness),
                    ),
                )
                .expect("Failed to draw chart data")
                .label(&series.name)
                .legend(|(x, y)| {
                    Rectangle::new([(x - 5, y - 3), (x + 15, y + 8)], series.color.filled())
                });
        }
    }
}
