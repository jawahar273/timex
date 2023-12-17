use log::debug;

use chrono::{DateTime, Utc};
use anyhow::{Result, bail};

use model::ScheduleDetails;
use days::for_days;
use weeks::for_week;

pub mod model;
pub mod errors;
mod days;
mod weeks;

fn generate_schedule_date_time(
    detail: &ScheduleDetails,
) -> Result<Vec<DateTime<Utc>>> {
    
    let default_range_of_days = 7;
    
    let scheduled_start_date_time: DateTime<Utc> = DateTime::parse_from_rfc3339(
        detail.scheduled_start_date_time.as_str()
    )?.with_timezone(&Utc);
    
    match detail.repeat_every {
        model::RepeatEvery::Day => {
            for_days(detail, scheduled_start_date_time, default_range_of_days)
        },
        model::RepeatEvery::Week => {
            for_week(detail, scheduled_start_date_time, 14)
        },
        model::RepeatEvery::Month => todo!(),
        model::RepeatEvery::Year => todo!(),
    }
}

pub fn schedule_date_times(detail: &ScheduleDetails) -> Result<Vec<DateTime<Utc>>> {
    // TODO: add validation
    debug!("trigger schedule date time");
    return generate_schedule_date_time(detail);
}

