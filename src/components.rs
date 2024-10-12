#![allow(non_snake_case)]
use dioxus::prelude::*;

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
