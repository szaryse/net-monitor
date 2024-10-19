use dioxus::prelude::*;
use crate::app::TransferQueue;
use crate::components::{CloseIcon, Flexbox, TextButton};

#[derive(PartialEq, Clone, Props)]
pub struct SettingsProps {
    onclick: EventHandler<MouseEvent>,
    interface: Signal<String>,
    transfer_type: Signal<String>,
}

#[allow(non_snake_case)]
pub fn Settings(props: SettingsProps) -> Element {
    let mut transfer_type = props.transfer_type;
    let mut interface = props.interface;

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
                            onclick: move |_| { transfer_type.set(String::from("Upload")); },
                        },
                        TextButton {
                            label: "Download",
                            active: transfer_type() == "Download",
                            onclick: move |_| { transfer_type.set(String::from("Download")); },
                        },
                    },
                    Flexbox {
                        align_items: "flex-end",
                        TextButton {
                            label: "Wi-Fi",
                            active: interface() == "Wi-Fi",
                            onclick: move |_| { interface.set(String::from("Wi-Fi")); },
                        },
                        TextButton {
                            label: "Ethernet",
                            active:  interface() == "Ethernet",
                            onclick: move |_| { interface.set(String::from("Ethernet")); },
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
