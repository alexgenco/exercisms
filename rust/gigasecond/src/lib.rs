extern crate chrono;
use chrono::*;

const GIGA: i64 = 1000000000;

pub fn after(dt: DateTime<UTC>) -> DateTime<UTC> {
    dt + Duration::seconds(GIGA)
}
