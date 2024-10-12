// todo #![windows_subsystem = "windows"]

use dioxus::desktop::{Config, WindowBuilder, LogicalSize, tao::dpi::PhysicalPosition, tao};
use dioxus::prelude::*;
use sysinfo::{NetworkExt, NetworksExt, SystemExt};

use crate::app::App;
use crate::helpers::set_global_styles;

mod chart;
mod components;
mod helpers;
mod app;

fn main() {
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(340, 68))
        .with_always_on_top(true)
        .with_title("Net Monitor 0.2")
        .with_resizable(true)
        // INFO: position only for development
        .with_position(PhysicalPosition::new(3220, 20)
        );

    LaunchBuilder::desktop()
        .with_cfg(Config::new()
            .with_window(window)
            .with_custom_head(set_global_styles())
        )
        .launch(App);
}
