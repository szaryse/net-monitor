#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::chart_view::ChartViewProps;

#[derive(PartialEq, Props, Clone)]
pub struct FlexboxProps {
    #[props(default = "row".to_string())]
    direction: String,
    #[props(default = "center".to_string())]
    justify_content: String,
    #[props(default = "center".to_string())]
    align_items: String,
    #[props(default = "0".to_string())]
    padding: String,
    #[props(default = "auto".to_string())]
    height: String,
    #[props(default = "100%".to_string())]
    width: String,
    #[props(default = "1".to_string())]
    flex_grow: String,
    #[props(default = "0".to_string())]
    outline: String,
    children: Element,
}

pub fn Flexbox(props: FlexboxProps) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "{ props.direction }",
            justify_content: "{ props.justify_content }",
            align_items: "{ props.align_items }",
            flex_grow: "{ props.flex_grow }",
            width: "{ props.width }",
            height: "{ props.height }",
            padding: "{ props.padding }",
            outline: "{props.outline}",
            {props.children}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct TextProps {
    #[props(default = "18px".to_string())]
    font_size: String,
    text: String,
    #[props(default = "inherit".to_string())]
    color: String,
    #[props(default = "center".to_string())]
    text_align: String,
    #[props(default = "auto".to_string())]
    line_height: String,
}

pub fn Text(props: TextProps) -> Element {
    rsx! {
        div {
            text_align: "{ props.text_align }",
            font_size: "{ props.font_size }",
            color: "{ props.color }",
            line_height: "{ props.line_height }",
            white_space: "nowrap",
            {props.text}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct WrapperProps {
    #[props(default = "100%".to_string())]
    width: String,
    #[props(default = "auto".to_string())]
    height: String,
    children: Element,
}

pub fn Wrapper(props: WrapperProps) -> Element {
    rsx! {
        div {
            width: "{ props.width }",
            height: "{ props.height }",
            flex_shrink: 0,
            {props.children}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct TransferProps {
    text: String,
    value: String,
    #[props(default = "inherit".to_string())]
    color: String,
    #[props(default = "auto".to_string())]
    height: String,
    #[props(default = "18px".to_string())]
    font_size: String,
}

pub fn Transfer(props: TransferProps) -> Element {
    rsx! {
        Flexbox {
            justify_content: "flex-start",
            Wrapper{
                width: "44px",
                Text {
                    text: "{ props.text }",
                    color: "{ props.color }",
                    text_align: "left",
                    line_height: "{ props.height }",
                    font_size: "{ props.font_size }",
                }
            }
            Text {
                text: "{ props.value }",
                color: "{ props.color }",
                text_align: "left",
                line_height: "{ props.height }",
                font_size: "{ props.font_size }",
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ChartViewWrapperProps {
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

pub fn ChartViewWrapper(props: ChartViewWrapperProps) -> Element {
    rsx! {
        button {
            background_color: "rgba(0,0,0,0.8)",
            border: 0,
            display: "block",
            margin: "4px",
            width: "calc(100% - 8px)",
            padding: "4px",
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
        }
    }
}

pub fn CloseIcon() -> Element {
    let contents = r"M480-433.33 274.67-228q-9.67 9.67-23.34 9.67-13.66
        0-23.33-9.67-9.67-9.67-9.67-23.33 0-13.67 9.67-23.34L433.33-480
        228-685.33q-9.67-9.67-9.67-23.34 0-13.66 9.67-23.33 9.67-9.67 23.33-9.67 13.67 0 23.34
        9.67L480-526.67 685.33-732q9.67-9.67 23.34-9.67 13.66 0 23.33 9.67 9.67 9.67 9.67 23.33
        0 13.67-9.67 23.34L526.67-480 732-274.67q9.67 9.67 9.67 23.34 0 13.66-9.67 23.33-9.67
        9.67-23.33 9.67-13.67 0-23.34-9.67L480-433.33Z";

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: "40",
            width: "40",
            view_box: "0 -960 960 960",
            path {
                d: "{contents}",
                fill: "#bf94ff",
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TextButtonProps {
    onclick: EventHandler<MouseEvent>,
    label: String,
    active: bool,
}

pub fn TextButton(props: TextButtonProps) -> Element {
    let mut color = "#c0c0c0";
    if props.active {
        color = "#bf94ff";
    }

    rsx! {
        button {
            display: "block",
            width: "96px",
            height: "18px",
            margin: "0 8px",
            background_color: "transparent",
            border: 0,
            color,
            line_height: "18px",
            font_size: "16px",
            letter_spacing: "1px",
            onclick: move |evt| props.onclick.call(evt),
            "{props.label}"
        }
    }
}
