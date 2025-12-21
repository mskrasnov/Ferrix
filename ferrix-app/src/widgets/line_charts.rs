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
    Alignment::Center,
    Color, Theme,
    alignment::{Horizontal, Vertical},
    time::Instant,
    widget::{button, column, container, row, slider, text},
};
use iced_aksel::{
    Axis, Chart, Measure, PlotPoint, State, Stroke,
    axis::{self, TickLine, TickResult},
    plot::{Plot, PlotData},
    scale::Linear,
    shape::{Label, Polygon, Polyline, Rectangle},
};
use std::collections::{HashMap, VecDeque};

type AxisID = String;
