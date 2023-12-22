use log::debug;

use anyhow::Result;
use chrono::{DateTime, Utc};

use days::for_days;
use model::ScheduleDetails;

use weeks::for_week;

pub mod errors;
pub mod model;
pub use self::utils::{
    get_start_and_last_date_of_month_for_given_date, get_week_bounded_days_for_given_date,
};
mod days;
mod utils;
mod weeks;

fn generate_schedule_date_time(
    detail: &ScheduleDetails,
    previous_scheduled_date: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    dbg!(previous_scheduled_date);
    match detail.repeat_every {
        model::RepeatEvery::Day => for_days(
            detail,
            previous_scheduled_date,
            start_range_date,
            end_range_date,
            Some(true),
        ),
        model::RepeatEvery::Week => {
            for_week(
                detail,
                // DateTime::parse_from_rfc3339("2023-12-14T14:08:15.223Z")
                //     .unwrap()
                //     .with_timezone(&Utc),
                // DateTime::parse_from_rfc3339("2023-12-25T14:08:15.223Z")
                //     .unwrap()
                //     .with_timezone(&Utc),
                // DateTime::parse_from_rfc3339("2023-12-31T14:08:15.223Z")
                //     .unwrap()
                //     .with_timezone(&Utc),
                previous_scheduled_date,
                start_range_date,
                end_range_date,
                Some(true),
            )
        }
        model::RepeatEvery::Month => todo!(),
        model::RepeatEvery::Year => todo!(),
    }
}

pub fn schedule_date_times(
    detail: &ScheduleDetails,
    previous_scheduled_date: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    // TODO: add validation
    debug!("trigger schedule date time");
    // let previous_scheduled_date: DateTime<Utc> = DateTime::parse_from_rfc3339(
    //     detail.scheduled_start_date_time.as_str()
    // )?.with_timezone(&Utc);
    return generate_schedule_date_time(
        detail,
        previous_scheduled_date,
        start_range_date,
        end_range_date,
    );
}
