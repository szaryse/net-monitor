use std::collections::VecDeque;
use dioxus::prelude::*;

use crate::app::{TransferQueue};

const CHART_HEIGHT: u64 = 40; // in pixels
const CHART_WIDTH: u64 = 180; // 60 bars per minute
const BAR_WIDTH: u64 = 2; // total bar width is 3px
const BAR_MARGIN: u64 = 1;

// recommended transfer in Mbits
const TRANSFER_720P_30FPS: f64 = 3.0;
// const TRANSFER_720P_60FPS: f64 = 4.5;
// const TRANSFER_1080P_30FPS: f64 = 4.5;
const TRANSFER_1080P_60FPS: f64 = 6.0;

// for Upload bar chart only
const PIXELS_PER_MBIT: u64 = 5; // CHART_HEIGHT divided by 8 Mbit (max_y_mbits)
const LINE_HEIGHT: u64 = CHART_HEIGHT - (TRANSFER_1080P_60FPS as u64 * PIXELS_PER_MBIT);

#[derive(PartialEq, Props, Clone)]
pub struct ChartProps {
    chart_data: VecDeque<f64>,
    transfer_type: String,
    kbits_per_pixel: i32,
}

#[allow(non_snake_case)]
pub fn Chart(props: ChartProps) -> Element {
    let chart_data = props.chart_data;

    let bars = chart_data.iter().enumerate().map(|(index, transfer)| {
        let bar_height = (*transfer / props.kbits_per_pixel as f64) as i32;
        let y_position = (CHART_HEIGHT as i32) - bar_height;
        let x_position = CHART_WIDTH - ((index + 1) as u64 * (BAR_WIDTH + BAR_MARGIN));

        let mut color = "skyblue";
        if props.transfer_type == "Upload" {
            let transfer_mbits = *transfer / 1000.0;
            color = match transfer_mbits {
                x if x < TRANSFER_720P_30FPS => "red",
                x if x < 4.0 => "orange",
                x if x < 5.0 => "yellow",
                x if x < TRANSFER_1080P_60FPS => "yellowgreen",
                _ => "green",
            };
        }

        rsx!(rect {
            x: "{x_position}",
            y: "{y_position}",
            width: "{BAR_WIDTH}",
            height: "{bar_height}",
            fill: "{color}",
        })
    });

    rsx! {
        svg {
            version: "1.1",
            height: "{CHART_HEIGHT}",
            width: "{CHART_WIDTH}",
            xmlns: "http://www.w3.org/2000/svg",
            if props.transfer_type == "Upload" {
                line {
                    x1: "0",
                    x2: "{CHART_WIDTH}",
                    y1: "{LINE_HEIGHT}",
                    y2: "{LINE_HEIGHT}",
                    stroke: "#bf94ff",
                    stroke_width: "1",
                    fill: "#bf94ff",
                }
            }
            { bars },
        }
    }
}
