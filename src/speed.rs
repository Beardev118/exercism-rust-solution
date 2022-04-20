pub fn production_rate_per_hour(speed: u8) -> f64 {
    let amount_per_hour: f64 = match speed {
        1..=4 => 221.0 * speed as f64,
        5..=8 => 221.0 * 0.9 * speed as f64,
        9 | 10 => 221.0 * 0.77 * speed as f64,
        _ => 0.0,
    };

    return amount_per_hour;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let items_per_minute = match speed {
        0 => 0,
        _ => ((production_rate_per_hour(speed) as u32) / (60 * speed as u32)),
    };

    return items_per_minute;
}
