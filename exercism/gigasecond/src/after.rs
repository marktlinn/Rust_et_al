// Returns a DateTime one billion seconds after start.
use time::{PrimitiveDateTime as DateTime, Duration};

pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::seconds(1_000_000_000);
    let result = start + duration;
    result
}