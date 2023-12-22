use crate::model::{ScheduleDetails, self};
use chrono::{DateTime, Days, offset, Utc, TimeZone, Datelike, Timelike, IsoWeek, Duration};
use anyhow::Result;

pub fn for_month(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    // end_date: DateTime<Utc>,
    range_of_days: u64,
) -> Result<Vec<DateTime<Utc>>> {
    todo!(); 
    let repeat_times = detail.repeat_every_number;
}