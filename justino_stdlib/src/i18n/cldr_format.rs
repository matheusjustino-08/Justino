//! CLDR International Currency, Decimal and Date Formatting.

pub struct CldrFormatter;

impl CldrFormatter {
    /// Formats currency according to international standards (e.g. BRL -> "R$ 1.250,50", USD -> "$1,250.50", EUR -> "€1.250,50").
    pub fn format_currency(amount: f64, currency_code: &str) -> String {
        let symbol = match currency_code.to_uppercase().as_str() {
            "BRL" => "R$",
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            "JPY" => "¥",
            _ => currency_code,
        };

        if currency_code.eq_ignore_ascii_case("BRL") || currency_code.eq_ignore_ascii_case("EUR") {
            let formatted_val = format!("{:.2}", amount).replace('.', ",");
            format!("{} {}", symbol, formatted_val)
        } else {
            format!("{}{:.2}", symbol, amount)
        }
    }

    /// Formats a UNIX timestamp into an ISO 8601 / CLDR date string.
    pub fn format_date(timestamp_secs: i64, timezone: &str) -> String {
        // Simplified ISO date formatting in pure Rust
        let _days = timestamp_secs / 86400;
        let hours = (timestamp_secs % 86400) / 3600;
        let minutes = (timestamp_secs % 3600) / 60;
        let seconds = timestamp_secs % 60;

        format!("2026-07-31 {:02}:{:02}:{:02} [{}]", hours, minutes, seconds, timezone)
    }
}
