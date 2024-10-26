use std::collections::VecDeque;
use dioxus::prelude::*;
use crate::app::{TransferQueue, PreviousTransfer};
use crate::components::{CloseIcon, Flexbox, TextButton};

#[derive(PartialEq, Clone, Props)]
pub struct SettingsProps {
    onclick: EventHandler<MouseEvent>,
    interface: Signal<String>,
    transfer_type: Signal<String>,
    previous_transfer: Signal<PreviousTransfer>,
    chart_data: Signal<TransferQueue>,
}

#[allow(non_snake_case)]
pub fn Settings(props: SettingsProps) -> Element {
    let mut transfer_type = props.transfer_type;
    let mut interface = props.interface;
    let mut previous_transfer = props.previous_transfer;
    let mut chart_data = props.chart_data;

    rsx! {
        div {
            background_color: "rgba(0,0,0,0.8)",
            margin: "4px",
            width: "calc(100% - 8px)",
            height: "calc(100vh - 8px)",
            padding: "4px",
            color: "#adadb8",
            Flexbox {
                justify_content: "space-between",
                height: "100%",
                Flexbox {
                    direction: "column",
                    justify_content: "space-between",
                    align_items: "center",
                    height: "40px",
                    Flexbox {
                        align_items: "flex-start",
                        TextButton {
                            label: "Upload",
                            active: transfer_type() == "Upload",
                            onclick: move |_| {
                                transfer_type.set(String::from("Upload"));
                                previous_transfer.set(PreviousTransfer {
                                    // todo reset max_y and kbits_per_pixel
                                    total_received: 0.0,
                                    total_transmitted: 0.0,
                                });
                            },
                        },
                        TextButton {
                            label: "Download",
                            active: transfer_type() == "Download",
                            onclick: move |_| {
                                transfer_type.set(String::from("Download"));
                                previous_transfer.set(PreviousTransfer {
                                    total_received: 0.0,
                                    total_transmitted: 0.0,
                                });
                            },
                        },
                    },
                    Flexbox {
                        align_items: "flex-end",
                        TextButton {
                            label: "Wi-Fi",
                            active: interface() == "Wi-Fi",
                            onclick: move |_| {
                                interface.set(String::from("Wi-Fi"));
                                previous_transfer.set(PreviousTransfer {
                                    total_received: 0.0,
                                    total_transmitted: 0.0,
                                });
                                chart_data.set(TransferQueue {
                                    upload: VecDeque::new(),
                                    download: VecDeque::new(),
                                });
                            },
                        },
                        TextButton {
                            label: "Ethernet",
                            active:  interface() == "Ethernet",
                            onclick: move |_| {
                                interface.set(String::from("Ethernet"));
                                previous_transfer.set(PreviousTransfer {
                                    total_received: 0.0,
                                    total_transmitted: 0.0,
                                });
                                chart_data.set(TransferQueue {
                                    upload: VecDeque::new(),
                                    download: VecDeque::new(),
                                });
                            },
                        },

                    },
                },
                button {
                    background_color: "transparent",
                    border: 0,
                    padding: 0,
                    display: "block",
                    onclick: move |evt| props.onclick.call(evt),
                    CloseIcon {}
                }
            }
        }
    }
}
