use dioxus::desktop::{Config, WindowBuilder, LogicalSize, tao::dpi::PhysicalPosition, tao};
use dioxus::prelude::*;

use std::collections::VecDeque;
use std::time::Duration;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};
use tokio::time::sleep;

use crate::chart::Chart;
use crate::components::{Flexbox, Transfer};
use crate::helpers::{count_new_transfer, format_transfer};

pub const UPDATE_TIME: u64 = 2;

#[derive(PartialEq, Clone, Debug)]
pub struct Transfer {
    dr: f64,
    dt: f64,
    rb: f64,
    tb: f64,
    pub upload: VecDeque<f64>,
}
impl Transfer {
    pub fn push_front(&mut self, dt: f64) {
        self.upload.push_front(dt);
    }
    pub fn pop_back(&mut self) {
        self.upload.pop_back();
    }
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let mut transfer = use_signal(|| Transfer {
        dr: 0.0,
        dt: 0.0,
        rb: 0.0,
        tb: 0.0,
        upload: VecDeque::new(),
    });

    let _ = use_resource(
        move || async move {
            let s = System::new_all();
            let networks = s.networks();

            loop {
                sleep(Duration::from_secs(UPDATE_TIME)).await;

                for network in networks.iter() {
                    let (interface_name, network) = network;

                    // "Wi-Fi"
                    if interface_name == "Ethernet" {
                        let received_bytes = network.total_received() as f64;
                        let transmitted_bytes = network.total_transmitted() as f64;


                        let oldTransfer = transfer.read().clone();

                        let dt = count_new_transfer(transmitted_bytes, oldTransfer.tb);
                        let dr = count_new_transfer(received_bytes, oldTransfer.rb);

                        transfer.write().dr = dr;
                        transfer.write().dt = dt;
                        transfer.write().push_front(dt);

                        if transfer.read().upload.len() > 30 {
                            transfer.write().pop_back();
                        }

                        transfer.write().rb = received_bytes;
                        transfer.write().tb = transmitted_bytes;
                    }
                }
            }
        },
    );

    let transmitted = format_transfer(transfer.read().dt);
    let received = format_transfer(transfer.read().dr);

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
                transfer: transfer.read().clone(),
            }
        }
    }
}
