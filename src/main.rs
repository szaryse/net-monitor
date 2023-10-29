use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};
use std::collections::VecDeque;
use std::time::Duration;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};
use tokio::time::sleep;

use crate::chart::Chart;
use crate::components::{Flexbox, Transfer};
use crate::helpers::{count_new_transfer, format_transfer, global_styles};

mod chart;
mod components;
mod helpers;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(400, 80))
                    .with_always_on_top(true)
                    .with_title("Net Monitor 0.1")
                    .with_resizable(false)
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(3220, 20)),
            )
            .with_custom_head(global_styles().to_string()),
    );
}

#[derive(PartialEq)]
struct Transfer {
    dr: f64,
    dt: f64,
    upload: VecDeque<f64>,
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
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Transfer {
        dr: 0.0,
        dt: 0.0,
        upload: VecDeque::new(),
    });
    let transfers = use_shared_state::<Transfer>(cx).unwrap();

    let received = use_state::<f64>(cx, || 0.0);
    let transmitted = use_state::<f64>(cx, || 0.0);

    use_future(
        cx,
        (received, transmitted, transfers),
        |(received, transmitted, transfers)| async move {
            let s = System::new_all();
            let networks = s.networks();

            loop {
                sleep(Duration::from_millis(1000)).await;

                for network in networks.iter() {
                    let (interface_name, network) = network;

                    if interface_name == "Wi-Fi" {
                        let received_bytes = network.total_received() as f64;
                        let transmitted_bytes = network.total_transmitted() as f64;

                        received.set(received_bytes);
                        transmitted.set(transmitted_bytes);

                        let dt = count_new_transfer(transmitted_bytes, *transmitted);

                        transfers.write().dr = count_new_transfer(received_bytes, *received);
                        transfers.write().dt = dt;
                        transfers.write().push_front(dt);

                        if transfers.read().upload.len() > 30 {
                            transfers.write().pop_back();
                        }
                    }
                }
            }
        },
    );

    let transmitted = format_transfer(transfers.read().dt);
    let received = format_transfer(transfers.read().dr);

    cx.render(rsx! {
        Flexbox {
            padding: "8px",
            justify_content: "space-between",
            Flexbox{
                direction: "column",
                align_items: "flex-start",
                width: "200px",
                flex_grow: "0",
                Transfer {
                    text: "U\u{02191}",
                    value: "{transmitted}",
                    color: "#bf94ff",
                    height: "40px",
                    font_size: "32px"
                }
                Transfer {
                    text: "D\u{02193}",
                    value: "{received}",
                    height: "24px",
                    font_size: "20px"
                }
            }
            Chart{}
        }
    })
}
