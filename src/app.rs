use dioxus::desktop::{Config, WindowBuilder, LogicalSize, tao::dpi::PhysicalPosition, tao, use_window};
use dioxus::prelude::*;

use std::collections::VecDeque;
use std::time::Duration;
use sysinfo::{Networks};
use tokio::time::sleep;

use crate::chart::Chart;
use crate::chart_view::ChartView;
use crate::components::{ChartViewWrapper, Flexbox, Transfer};
use crate::helpers::{count_new_transfer, format_transfer, get_kbit_to_set, get_max_value};
use crate::settings::Settings;

pub const UPDATE_TIME_IN_S: u64 = 1;
pub const TICK_NUMBER: usize = 60;

#[derive(PartialEq, Clone, Debug)]
struct CurrentTransfer {
    delta_received: f64,
    delta_transmitted: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PreviousTransfer {
    pub total_received: f64,
    pub total_transmitted: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct TransferQueue {
    pub upload: VecDeque<f64>,
    pub download: VecDeque<f64>,
}
impl TransferQueue {
    pub fn push_upload_front(&mut self, dt: f64) {
        self.upload.push_front(dt);
    }
    pub fn pop_upload_back(&mut self) {
        self.upload.pop_back();
    }
    pub fn push_download_front(&mut self, dt: f64) {
        self.download.push_front(dt);
    }
    pub fn pop_download_back(&mut self) {
        self.download.pop_back();
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
        download: VecDeque::new(),
    });
    let mut is_settings_open = use_signal(|| false);
    let mut interface = use_signal(|| String::from("Ethernet"));
    let mut transfer_type = use_signal(|| String::from("Upload"));
    let mut kbits_per_pixel = use_signal(|| 25);
    let mut max_y_mbits = use_signal(|| 1);

    use_effect(move || window.set_inner_size(LogicalSize::new(340, 56)));

    let _ = use_resource(
        move || async move {
            let networks = Networks::new_with_refreshed_list();

            loop {
                sleep(Duration::from_secs(UPDATE_TIME_IN_S)).await;

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
                        new_chart_data.push_upload_front(delta_transmitted);
                        new_chart_data.push_download_front(delta_received);

                        if new_chart_data.upload.len() > TICK_NUMBER {
                            new_chart_data.pop_upload_back();
                        }
                        if new_chart_data.download.len() > TICK_NUMBER {
                            new_chart_data.pop_download_back();
                        }

                        if transfer_type() == "Download" {
                            let max = get_max_value(new_chart_data.download.clone());
                            let (kbit_to_set, max_y) = get_kbit_to_set(max);

                            kbits_per_pixel.set(kbit_to_set);
                            max_y_mbits.set(max_y);
                        } else {
                            kbits_per_pixel.set(200);
                            max_y_mbits.set(8);
                        }

                        chart_data.set(new_chart_data);

                        let transfer = PreviousTransfer {
                            total_received: received_bytes,
                            total_transmitted: transmitted_bytes,
                        };
                        previous_transfer.set(transfer);
                    } else {
                        let no_transfer = CurrentTransfer {
                            delta_received: 0.0,
                            delta_transmitted: 0.0,
                        };
                        current_transfer.set(no_transfer);
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
                previous_transfer,
                chart_data,
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
                    kbits_per_pixel: kbits_per_pixel(),
                    max_y_mbits: max_y_mbits(),
                }
            }
        }
    }
}
