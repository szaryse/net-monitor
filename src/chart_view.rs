use dioxus::prelude::*;
use crate::app::TransferQueue;
use crate::chart::Chart;
use crate::components::{Flexbox, Transfer};
use crate::helpers::{count_new_transfer, format_transfer};

#[derive(PartialEq, Props, Clone)]
pub struct ChartViewProps {
    transmitted: String,
    received: String,
    chart_data: Signal<TransferQueue>,
    transfer_type: String,
}


#[allow(non_snake_case)]
pub fn ChartView(props: ChartViewProps) -> Element {
    let chart_data = props.chart_data;

    rsx! {
        Flexbox {
            justify_content: "space-between",
            Flexbox{
                direction: "column",
                align_items: "flex-start",
                width: "140px",
                flex_grow: "0",
                if props.transfer_type == "Upload" {
                    Transfer {
                        // text: "U\u{02191}",
                        text: "UL",
                        value: "{props.transmitted}",
                        color: "#bf94ff",
                        height: "24px",
                        font_size: "20px"
                    }
                    Transfer {
                        // text: "D\u{02193}",
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
                        color: "green",
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
            if props.transfer_type == "Upload" {
                Chart{
                    chart_data: chart_data().upload,
                }
            } else {
                Chart{
                    chart_data: chart_data().download,
                }
            }
        }
    }
}
