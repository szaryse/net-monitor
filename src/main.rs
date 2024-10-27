#![windows_subsystem = "windows"]

// INFO: PhysicalPosition only for development
// use dioxus::desktop::{Config, WindowBuilder, LogicalSize, tao::dpi::PhysicalPosition, tao};
use dioxus::desktop::{Config, WindowBuilder, LogicalSize};
use dioxus::prelude::*;

use crate::app::App;
use crate::helpers::set_global_styles;

mod chart;
mod components;
mod helpers;
mod app;
mod chart_view;
mod settings;

fn main() {
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(340, 0))
        .with_always_on_top(true)
        .with_title("Net Monitor 0.2")
        .with_resizable(false);
    // INFO: position only for development
    // .with_position(PhysicalPosition::new(3220, 20)
    // );

    LaunchBuilder::desktop()
        .with_cfg(Config::new()
            .with_window(window)
            .with_custom_head(set_global_styles())
        )
        .launch(App);
}
