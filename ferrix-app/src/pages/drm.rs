//! DRM Page

use crate::{
    DataLoadingState, Message,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::drm::{DRM, EDID, Video, VideoInputParams};

use iced::{
    Alignment::Center,
    widget::{column, container, row, rule, scrollable, text},
};

pub fn drm_page<'a>(video: &'a DataLoadingState<Video>) -> container::Container<'a, Message> {
    match video {
        DataLoadingState::Loaded(video) => {
            let mut layout = column![].spacing(5);
            let mut i = 1;
            for device in &video.devices {
                layout = layout.push(screen_subpage(device, i));
                i += 1;
            }
            container(scrollable(layout))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn screen_subpage<'a>(drm: &'a DRM, idx: usize) -> container::Container<'a, Message> {
    let mut layout = column![
        row![text(format!("Screen #{idx}")).size(16), rule::horizontal(1),]
            .spacing(5)
            .align_y(Center),
    ]
    .spacing(5);

    layout = if drm.enabled {
        match &drm.edid {
            Some(edid) => layout.push(
                column![
                    text("Summary:").style(text::warning),
                    edid_summary_table(edid),
                    text("Video params:").style(text::warning),
                    edid_video_params_table(edid),
                ]
                .spacing(5),
            ),
            None => layout.push(text(format!("\tEDID data for monitor #{idx} not found!"))),
        }
    } else {
        layout.push(text(format!("Monitor #{idx} isn't enabled!")).style(text::danger))
    };

    if drm.enabled {
        layout = layout.push(text("Support modes").style(text::warning));
        layout = layout.push(support_modes_table(&drm.modes));
    }

    container(layout)
}

fn support_modes_table<'a>(modes: &'a [String]) -> container::Container<'a, Message> {
    let mut rows = Vec::with_capacity(modes.len());
    for mode in modes {
        rows.push(InfoRow::new("Mode", fmt_val(Some(mode))));
    }
    container(kv_info_table(rows)).style(container::rounded_box)
}

fn edid_summary_table<'a>(edid: &'a EDID) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new("Manufacturer", Some(edid.manufacturer.clone())),
        InfoRow::new("Product Code", fmt_val(Some(edid.product_code))),
        InfoRow::new("Serial Number", Some(format!("{:X}", edid.serial_number))),
        InfoRow::new("Week/Year", Some(format!("{}/{}", edid.week, edid.year))),
        InfoRow::new("EDID Version", fmt_val(Some(edid.edid_version))),
        InfoRow::new("EDID Revision", fmt_val(Some(edid.edid_revision))),
        InfoRow::new(
            "Screen size, cm",
            Some(format!("{}x{}", edid.hscreen_size, edid.vscreen_size)),
        ),
        InfoRow::new("Display gamma (default)", fmt_val(Some(edid.display_gamma))),
    ];
    container(kv_info_table(rows)).style(container::rounded_box)
}

fn edid_video_params_table<'a>(edid: &'a EDID) -> container::Container<'a, Message> {
    let rows = match &edid.video_input {
        VideoInputParams::Digital(val) => vec![
            InfoRow::new("Signal type", Some("Digital".to_string())),
            InfoRow::new("Bit depth", Some(format!("{}", val.bit_depth))),
            InfoRow::new("Video interface", Some(format!("{}", val.video_interface))),
        ],
        VideoInputParams::Analog(val) => vec![
            InfoRow::new("Signal type", Some("Analogue".to_string())),
            InfoRow::new("White sync levels", fmt_val(Some(val.white_sync_levels))),
            InfoRow::new(
                "Blank to black setup",
                fmt_val(Some(val.blank_to_black_setup)),
            ),
            InfoRow::new(
                "Separate sync supported",
                fmt_val(Some(val.separate_sync_supported)),
            ),
            InfoRow::new(
                "Composite sync supported",
                fmt_val(Some(val.composite_sync_supported)),
            ),
            InfoRow::new(
                "Sync on green supported",
                fmt_val(Some(val.sync_on_green_supported)),
            ),
            InfoRow::new(
                "Sync on green issued",
                fmt_val(Some(val.sync_on_green_isused)),
            ),
        ],
    };
    container(kv_info_table(rows)).style(container::rounded_box)
}
