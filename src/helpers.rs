use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();

    let secs = total_secs % 60;
    let total_mins = total_secs / 60;
    let mins = total_mins % 60;
    let total_hours = total_mins / 60;
    let hours = total_hours % 24;
    let days = total_hours / 24;

    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if days > 0 || hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if days > 0 || hours > 0 || mins > 0 {
        parts.push(format!("{}m", mins));
    }
    
    parts.push(format!("{}s", secs));

    parts.join(" ")
}