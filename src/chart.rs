use dioxus::prelude::*;

const CHART_HEIGHT: f64 = 64.0;
const CHART_WIDTH: f64 = 180.0;
const BAR_WIDTH: f64 = 5.0;

const PIXEL_PER_MBIT: f64 = 8.0;

// recommended transfer in Mbits
const TRANSFER_720P: f64 = 3.0;
const TRANSFER_1080P: f64 = 4.5;

#[allow(non_snake_case)]
pub fn Chart(cx: Scope) -> Element {
    let data = vec![
        1.0, 2.0, 3.6, 4.2, 10.0, 6.0, 1.0, 2.0, 4.0, 8.0, 10.0, 6.0, 1.0, 2.0, 4.0, 8.0, 10.0,
        6.0, 1.0, 2.0, 4.0, 8.0, 10.0, 6.0, 1.0, 2.0, 4.0, 8.0, 10.0, 6.0,
    ];

    let bars = data.iter().rev().enumerate().map(|(index, transfer)| {
        let bar_height = *transfer * PIXEL_PER_MBIT;
        let y_position = CHART_HEIGHT - bar_height;
        let x_position = index as f64 * (BAR_WIDTH + 1.0);

        let color = match transfer {
            x if x < &TRANSFER_720P => 0,
            x if x < &TRANSFER_1080P => ((transfer - 3.0) * 80.0) as i32,
            _ => 120,
        };

        rsx!(rect {
            x: "{x_position}",
            y: "{y_position}",
            width: "{BAR_WIDTH}",
            height: "{bar_height}",
            fill: "hsl({color}, 100%, 50%)",
        })
    });

    cx.render(rsx! {
        svg {
            version: "1.1",
            height: "{CHART_HEIGHT}",
            width: "{CHART_WIDTH}",
            xmlns: "http://www.w3.org/2000/svg",
            bars,
            line {
                x1: "0",
                x2: "{CHART_WIDTH}",
                y1: "{CHART_HEIGHT - (5.0 * PIXEL_PER_MBIT)}",
                y2: "{CHART_HEIGHT - (5.0 * PIXEL_PER_MBIT)}",
                stroke: "hsl(120, 100%, 50%)",
                stroke_width: "2",
                fill: "hsl(120, 100%, 50%)",
            }
        }
    })
}
