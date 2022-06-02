
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
    /// Return the plural noun for this time unit
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// A timestamp that has been deliberately rounded off,
/// so our program says "6 months ago" instead of "february 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

enum Shape {
    Sphere {center: Point3d, radius: f32},
    CUboid {corner1: Point3d, corner2: Point3d},
}

use std::collections::HashMap;

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

enum Option<T> {
    None,
    Some(T),
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn main() {
    let four_score_and_seven_years_ago =
        RoughTime::InThePast(TimeUnit::Years, 4*20+7);
    let three_hours_from_now =
        RoughTime::InTheFuture(TimeUnit::Hours, 3);
}

