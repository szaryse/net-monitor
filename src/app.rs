use dioxus::desktop::{Config, WindowBuilder, LogicalSize, tao::dpi::PhysicalPosition, tao};
use dioxus::prelude::*;

use std::collections::VecDeque;
use std::time::Duration;
use sysinfo::{Networks};
use tokio::time::sleep;

use crate::chart::Chart;
use crate::components::{Flexbox, Transfer};
use crate::helpers::{count_new_transfer, format_transfer};

pub const UPDATE_TIME: u64 = 2;

#[derive(PartialEq, Clone, Debug)]
struct CurrentTransfer {
    delta_received: f64,
    delta_transmitted: f64,
}

#[derive(PartialEq, Clone, Debug)]
struct PreviousTransfer {
    total_received: f64,
    total_transmitted: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct TransferQueue {
    pub upload: VecDeque<f64>,
}
impl TransferQueue {
    pub fn push_front(&mut self, dt: f64) {
        self.upload.push_front(dt);
    }
    pub fn pop_back(&mut self) {
        self.upload.pop_back();
    }
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let mut previous_transfer = use_signal(|| PreviousTransfer {
        total_received: 0.0,
        total_transmitted: 0.0,
    });
    let mut current_transfer = use_signal(|| CurrentTransfer {
        delta_received: 0.0,
        delta_transmitted: 0.0,
    });
    let mut chart_data = use_signal(|| TransferQueue {
        upload: VecDeque::new(),
    });

    let _ = use_resource(
        move || async move {
            let networks = Networks::new_with_refreshed_list();

            loop {
                sleep(Duration::from_secs(UPDATE_TIME)).await;

                for network in networks.iter() {
                    let (interface_name, network) = network;

                    // todo "Wi-Fi"
                    if interface_name == "Ethernet" {
                        let received_bytes = network.total_received() as f64;
                        let transmitted_bytes = network.total_transmitted() as f64;

                        let delta_transmitted = count_new_transfer(transmitted_bytes, previous_transfer().total_transmitted);
                        let delta_received = count_new_transfer(received_bytes, previous_transfer().total_received);

                        let current_data = CurrentTransfer {
                            delta_received,
                            delta_transmitted,
                        };
                        current_transfer.set(current_data);

                        let mut new_chart_data = chart_data();
                        new_chart_data.push_front(delta_transmitted);
                        if new_chart_data.upload.len() > 30 {
                            new_chart_data.pop_back();
                        }
                        chart_data.set(new_chart_data);

                        let transfer = PreviousTransfer {
                            total_received: received_bytes,
                            total_transmitted: transmitted_bytes,
                        };
                        previous_transfer.set(transfer);
                    }
                }
            }
        },
    );

    let transmitted = format_transfer(current_transfer().delta_transmitted);
    let received = format_transfer(current_transfer().delta_received);

    rsx! {
        Flexbox {
            padding: "4px",
            justify_content: "space-between",
            Flexbox{
                direction: "column",
                align_items: "flex-start",
                width: "140px",
                flex_grow: "0",
                Transfer {
                    text: "U\u{02191}",
                    value: "{transmitted}",
                    color: "#bf94ff",
                    height: "22px",
                    font_size: "20px"
                }
                Transfer {
                    text: "D\u{02193}",
                    value: "{received}",
                    height: "18px",
                    font_size: "16px"
                }
            }
            Chart{
                chart_data: chart_data(),
            }
        }
    }
}
