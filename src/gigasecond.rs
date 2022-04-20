use time::PrimitiveDateTime as DateTime;
use time::{ext::NumericalDuration, Date};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let cur_date = start.checked_add(1000000000.seconds());
    match cur_date {
        Some(x) => x,
        _ => start,
    }
}
