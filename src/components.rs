#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct FlexboxProps<'a> {
    #[props(default = "row")]
    direction: &'a str,
    #[props(default = "center")]
    justify_content: &'a str,
    #[props(default = "center")]
    align_items: &'a str,
    #[props(default = "0")]
    padding: &'a str,
    #[props(default = "auto")]
    height: &'a str,
    #[props(default = "100%")]
    width: &'a str,
    #[props(default = "1")]
    flex_grow: &'a str,
    children: Element<'a>,
}

pub fn Flexbox<'a>(cx: Scope<'a, FlexboxProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: cx.props.direction,
            justify_content: cx.props.justify_content,
            align_items: cx.props.align_items,
            flex_grow: cx.props.flex_grow,
            width: cx.props.width,
            height: cx.props.height,
            padding: cx.props.padding,
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct TextProps<'a> {
    #[props(default = "18px")]
    font_size: &'a str,
    text: &'a str,
    #[props(default = "inherit")]
    color: &'a str,
    #[props(default = "center")]
    text_align: &'a str,
    #[props(default = "auto")]
    line_height: &'a str,
}

pub fn Text<'a>(cx: Scope<'a, TextProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            text_align: cx.props.text_align,
            font_family: "'Consolas', sans-serif",
            font_size: cx.props.font_size,
            color: cx.props.color,
            line_height: cx.props.line_height,
            white_space: "nowrap",
            "{cx.props.text}"
        }
    })
}

#[derive(Props)]
pub struct WrapperProps<'a> {
    #[props(default = "100%")]
    width: &'a str,
    #[props(default = "auto")]
    height: &'a str,
    children: Element<'a>,
}

pub fn Wrapper<'a>(cx: Scope<'a, WrapperProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            width: cx.props.width,
            height: cx.props.height,
            flex_shrink: 0,
            &cx.props.children
        }
    })
}

#[derive(PartialEq, Props)]
pub struct TransferProps<'a> {
    text: &'a str,
    value: &'a str,
    #[props(default = "inherit")]
    color: &'a str,
    #[props(default = "auto")]
    height: &'a str,
    #[props(default = "18px")]
    font_size: &'a str,
}

pub fn Transfer<'a>(cx: Scope<'a, TransferProps<'a>>) -> Element {
    cx.render(rsx! {
        Flexbox {
            justify_content: "flex-start",
            Wrapper{
                width: "44px",
                Text {
                    text: "{cx.props.text}",
                    color: "{cx.props.color}",
                    text_align: "left",
                    line_height: "{cx.props.height}",
                    font_size: "{cx.props.font_size}",
                }
            }
            Text {
                text: "{cx.props.value}",
                color: "{cx.props.color}",
                text_align: "left",
                line_height: "{cx.props.height}",
                font_size: "{cx.props.font_size}",
            }
        }
    })
}
