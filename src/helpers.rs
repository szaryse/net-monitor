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
