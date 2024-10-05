use dioxus::prelude::*;

use crate::Transfer;

const PIXELS_PER_MBIT: u64 = 4;
const MAX_TRANSFER_MBIT: f64 = 8.0;

const CHART_HEIGHT: u64 = (MAX_TRANSFER_MBIT as u64) * PIXELS_PER_MBIT;
const CHART_WIDTH: u64 = 180;
const BAR_WIDTH: u64 = 4;
const BAR_MARGIN: u64 = 2;

// recommended transfer in Mbits
const TRANSFER_720P: f64 = 3.0;
const TRANSFER_1080P: f64 = 4.5;

#[allow(non_snake_case)]
pub fn Chart(cx: Scope) -> Element {
    let transfers = use_shared_state::<Transfer>(cx).unwrap();
    let upload = transfers.read().upload.clone();

    let bars = upload.iter().enumerate().map(|(index, transfer)| {
        let transfer_mbits = *transfer / 1000.0;

        let bar_height = (transfer_mbits * (PIXELS_PER_MBIT as f64)) as i64;
        let y_position = (CHART_HEIGHT as i64) - bar_height;
        let x_position = CHART_WIDTH - ((index + 1) as u64 * (BAR_WIDTH + BAR_MARGIN));

        let color = match transfer_mbits {
            x if x < TRANSFER_720P => "red",
            x if x < 3.5 => "orange",
            x if x < 4.0 => "yellow",
            x if x < TRANSFER_1080P => "yellowgreen",
            _ => "green",
        };

        rsx!(rect {
            x: "{x_position}",
            y: "{y_position}",
            width: "{BAR_WIDTH}",
            height: "{bar_height}",
            fill: "{color}",
        })
    });

    cx.render(rsx! {
        svg {
            version: "1.1",
            height: "{CHART_HEIGHT}",
            width: "{CHART_WIDTH}",
            xmlns: "http://www.w3.org/2000/svg",
            line {
                x1: "0",
                x2: "{CHART_WIDTH}",
                y1: "{CHART_HEIGHT - (5 * PIXELS_PER_MBIT)}",
                y2: "{CHART_HEIGHT - (5 * PIXELS_PER_MBIT)}",
                stroke: "#bf94ff",
                stroke_width: "1",
                fill: "#bf94ff",
            }
            bars,
        }
    })
}
