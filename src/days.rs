
use chrono::{DateTime, Days, offset, Utc, TimeZone, Datelike, Timelike};
use anyhow::{Result, bail};

use crate::errors::ScheduleError;
use crate::model::{ScheduleDetails, self};

pub fn for_days(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    // end_date: DateTime<Utc>,
    range_of_days: u64,
) -> Result<Vec<DateTime<Utc>>> {

    let repeat_times = detail.repeat_every_number;
    if repeat_times >= 32 {
        bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
    }
    
    let mut schedule_start = scheduled_start_date_time.checked_add_days(
        Days::new(repeat_times.try_into()?)
    ).unwrap();
    
    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => {
            // TODO: confirm this logic works or not
            let t = offset::Utc::now().checked_add_days(Days::new(detail.occurrence_value.unwrap().try_into()?)).unwrap();
            t
        },
        model::EndOption::OnThe => {
            let t = DateTime::parse_from_rfc3339(
                // detail.end_date.as_ref().unwrap().as_str()
                detail.end_date.as_ref().unwrap().as_str(),
            )?.with_timezone(&Utc);
            t
        },
        model::EndOption::Never => {
            let t = offset::Utc::now().checked_add_days(Days::new(range_of_days)).unwrap();
            t
        },
    };

    // FIX: only if the scheduled_start_date_time equal or past from current date for the different calculation to work
    let diff = end_date - Utc::now();
    let num_days = diff.num_days();
    
    let mut temp: Vec< DateTime<Utc> >= vec![];
    
    for _ in 0..num_days {

        temp.push(
            Utc.with_ymd_and_hms(
                schedule_start.year(),
                schedule_start.month(),
                schedule_start.day(),
                scheduled_start_date_time.hour(),
                scheduled_start_date_time.minute(),
                scheduled_start_date_time.second()).unwrap()
                .with_timezone(&Utc)
        );

        schedule_start = schedule_start.checked_add_days(
            Days::new(repeat_times.try_into()?)
        ).unwrap();

    }

    temp.push(
        Utc.with_ymd_and_hms(
            schedule_start.year(),
            schedule_start.month(),
            schedule_start.day(),
            scheduled_start_date_time.hour(),
            scheduled_start_date_time.minute(),
            scheduled_start_date_time.second()).unwrap()
            .with_timezone(&Utc)
    );
    
    Ok(temp)
}
