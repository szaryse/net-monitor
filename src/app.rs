use dioxus::desktop::{Config, WindowBuilder, LogicalSize, tao::dpi::PhysicalPosition, tao, use_window};
use dioxus::prelude::*;

use std::collections::VecDeque;
use std::time::Duration;
use sysinfo::{Networks};
use tokio::time::sleep;

use crate::chart::Chart;
use crate::chart_view::ChartView;
use crate::components::{ChartViewWrapper, Flexbox, Transfer};
use crate::helpers::{count_new_transfer, format_transfer};
use crate::settings::Settings;

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
    let window = use_window();
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
    let mut is_settings_open = use_signal(|| false);
    let mut interface = use_signal(|| String::from("Ethernet"));
    let mut transfer_type = use_signal(|| String::from("Upload"));

    use_effect(move || window.set_inner_size(LogicalSize::new(340, 56)));

    let _ = use_resource(
        move || async move {
            let networks = Networks::new_with_refreshed_list();

            loop {
                sleep(Duration::from_secs(UPDATE_TIME)).await;

                for network in networks.iter() {
                    let (interface_name, network) = network;

                    if interface_name == &interface() {
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

    if is_settings_open() {
        rsx! {
            Settings {
                onclick: move |_| { is_settings_open.set(!is_settings_open()); },
                interface,
                transfer_type,
            }
        }
    } else {
        rsx! {
            ChartViewWrapper {
                onclick: move |_| { is_settings_open.set(!is_settings_open()); },
                ChartView {
                    transmitted,
                    received,
                    chart_data,
                    transfer_type: transfer_type(),
                }
            }
        }
    }
}
