use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};
use std::time::Duration;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};
use tokio::time::sleep;

fn global_styles() -> &'static str {
    r"
        <style>
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }
            html {
                font-family: 'Consolas', sans-serif;
            }
            body {
                background-color: #222;
                color: #DDD;
            }
        </style>"
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(240, 100))
                    .with_always_on_top(true)
                    .with_title("Net Monitor 0.1")
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(3070, 20)),
            )
            .with_custom_head(global_styles().to_string()),
    );
}

#[derive(PartialEq)]
struct Transfer {
    dr: f64,
    dt: f64,
}

fn format_transfer(transfer: f64) -> String {
    if transfer > 1000.0 {
        format!("{:.1} Mb/s", transfer / 1000.0)
    } else {
        format!("{:.0} kb/s", transfer)
    }
}

// #[allow(non_snake_case)] TODO: use it

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Transfer { dr: 0.0, dt: 0.0 });
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

                        let next_dr = (received_bytes - *received) * 8.0 / (1_000.0);
                        let next_dt = (transmitted_bytes - *transmitted) * 8.0 / (1_000.0);

                        received.set(received_bytes);
                        transmitted.set(transmitted_bytes);

                        transfers.write().dr = next_dr;
                        transfers.write().dt = next_dt;
                    }
                }
            }
        },
    );

    let transmitted = format_transfer(transfers.read().dt);
    let received = format_transfer(transfers.read().dr);

    cx.render(rsx! {
        div { "Upload: {transmitted}" }
        div { "Download: {received}" }
    })
}
