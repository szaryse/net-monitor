use dioxus::prelude::*;
use crate::app::TransferQueue;
use crate::chart::Chart;
use crate::components::{Flexbox, Transfer, Text};

#[derive(PartialEq, Props, Clone)]
pub struct ChartViewProps {
    transmitted: String,
    received: String,
    chart_data: Signal<TransferQueue>,
    transfer_type: String,
    kbits_per_pixel: i32,
    max_y_mbits: i32,
}

#[allow(non_snake_case)]
pub fn ChartView(props: ChartViewProps) -> Element {
    let chart_data = props.chart_data;
    let mut data = chart_data().upload;
    if props.transfer_type == "Download" {
        data = chart_data().download;
    }

    rsx! {
        Flexbox {
            justify_content: "space-between",
            Flexbox{
                direction: "column",
                align_items: "flex-start",
                width: "116px",
                flex_grow: "0",
                if props.transfer_type == "Upload" {
                    Transfer {
                        text: "UL",
                        value: "{props.transmitted}",
                        color: "#bf94ff",
                        height: "24px",
                        font_size: "20px"
                    }
                    Transfer {
                        text: "DL",
                        value: "{props.received}",
                        height: "16px",
                        font_size: "14px",
                        color: "#c0c0c0",
                    }
                } else {
                    Transfer {
                        text: "D\u{02193}",
                        value: "{props.received}",
                        color: "skyblue",
                        height: "24px",
                        font_size: "20px"
                    }
                    Transfer {
                        text: "U\u{02191}",
                        value: "{props.transmitted}",
                        height: "16px",
                        font_size: "14px",
                        color: "#c0c0c0",
                    }
                }
            }
            Flexbox {
                height: "40px",
                width: "max-content",
                justify_content: "space-between",
                Flexbox  {
                    justify_content: "space-between",
                    direction: "column",
                    align_items: "flex-end",
                    padding: "0 4px 0 0",
                    height: "100%",
                    width: "24px",
                    if props.transfer_type == "Upload" {
                        Text {
                            color: "green",
                            font_size: "16px",
                            text: "{ props.max_y_mbits }",
                        }
                        Text {
                            color: "green",
                            font_size: "16px",
                            line_height: "12px",
                            text: "0",
                        }
                    } else {
                        Text {
                            color: "skyblue",
                            font_size: "16px",
                            text: "{ props.max_y_mbits }",
                        }
                        Text {
                            color: "skyblue",
                            font_size: "16px",
                            line_height: "12px",
                            text: "0",
                        }
                    }

                },
                Chart{
                    chart_data: data,
                    transfer_type: props.transfer_type,
                    kbits_per_pixel: props.kbits_per_pixel,
                }
            }
        }
    }
}
