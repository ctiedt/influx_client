use crate::Precision;
use std::fmt::Display;

/// This struct is used in a `ReadQuery`
/// to determine the time range from which data will be read.
pub struct Range {
    start: Option<(i32, Precision)>,
    stop: Option<(i32, Precision)>,
}

impl Range {
    pub fn new(start: Option<(i32, Precision)>, stop: Option<(i32, Precision)>) -> Self {
        Self { start, stop }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let (Some((start_time, start_precision)), Some((stop_time, stop_precision))) =
            (&self.start, &self.stop)
        {
            write!(
                f,
                "range(start: {start_time}{start_precision}, stop: {stop_time}{stop_precision}",
            )
        } else if let (Some((start_time, start_precision)), None) = (&self.start, &self.stop) {
            write!(f, "range(start: {start_time}{start_precision})")
        } else if let (None, Some((stop_time, stop_precision))) = (&self.start, &self.stop) {
            write!(f, "range(stop: {stop_time}{stop_precision})")
        } else {
            Err(std::fmt::Error)
        }
    }
}
