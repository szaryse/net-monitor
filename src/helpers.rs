use std::collections::VecDeque;
use crate::app::UPDATE_TIME;

pub fn set_global_styles() -> String {
    r"<style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        html {
            font-family: 'Consolas', sans-serif;
        }
        body {
            background-color: transparent
            color: #adadb8;
        }
    </style>".to_string()
}

pub fn format_transfer(transfer: f64) -> String {
    if transfer > 1000.0 {
        format!("{:.1} Mb/s", transfer / 1000.0)
    } else {
        format!("{:.0} kb/s", transfer)
    }
}

pub fn count_new_transfer(total_bytes: f64, bytes: f64) -> f64 {
    if bytes == 0.0 {
        return 0.0;
    }

    (total_bytes - bytes) * 8.0 / (1_000.0 * UPDATE_TIME as f64)
}

pub fn get_kbit_to_set(transfer_value: f64) -> (i32, i32) {
    let (kbits_per_pixel, max_y) = match transfer_value {
        0.0..=1000.0 => (25, 1),
        1000.0..=4000.0 => (100, 4),
        4000.0..=8000.0 => (200, 8),
        8000.0..=16000.0 => (400, 16),
        16000.0..=32000.0 => (800, 32),
        32000.0..=64000.0 => (1600, 64),
        _ => (3200, 128)
    };

    (kbits_per_pixel, max_y)
}

pub fn get_max_value(queue: VecDeque<f64>) -> f64 {
    queue.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}
