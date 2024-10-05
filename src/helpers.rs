use crate::UPDATE_TIME;

pub fn global_styles() -> &'static str {
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
                background-color: #0e0e10;
                color: #adadb8;
            }
        </style>"
}

pub fn format_transfer(transfer: f64) -> String {
    if transfer > 1000.0 {
        format!("{:.1} Mb/s", transfer / 1000.0)
    } else {
        format!("{:.0} kb/s", transfer)
    }
}

pub fn count_new_transfer(total_bytes: f64, bytes: f64) -> f64 {
    (total_bytes - bytes) * 8.0 / (1_000.0 * UPDATE_TIME as f64)
}
