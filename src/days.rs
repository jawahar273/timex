use std::cmp::Ordering;

use anyhow::{bail, Result};
use chrono::{offset, DateTime, Datelike, Days, TimeZone, Timelike, Utc};
use serde::de;

use crate::errors::ScheduleError;
use crate::model::{self, ScheduleDetails};
use crate::utils::check_date_with_given_range;

pub fn for_days(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> Result<Vec<DateTime<Utc>>> {
    let repeat_times = detail.repeat_every_number;
    if repeat_times >= 32 {
        bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
    }

    // FIX: check the start date greater than today
    let mut schedule_start = scheduled_start_date_time + Days::new(repeat_times.try_into()?);

    // today compare with old date is "greater"
    // today compare with future date is "less"
    // match today.cmp(&scheduled_start_date_time) {
    //     Ordering::Less => {
    //         println!("less");
    //         schedule_start = start_range_date.checked_add_days(
    //             Days::new(repeat_times.try_into()?)
    //         ).unwrap();
    //     },
    //     Ordering::Greater => {
    //         println!("greater");
    //         // skip to the date on to start
    //         schedule_start = start_range_date.checked_add_days(
    //             Days::new(repeat_times.try_into()?)
    //         ).unwrap();
    //     },
    //     Ordering::Equal => {
    //         println!("equal");
    //         schedule_start = scheduled_start_date_time.checked_add_days(
    //             Days::new(repeat_times.try_into()?)
    //         ).unwrap();
    //     }
    // }

    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => {
            // TODO: confirm this logic works or not
            let possible_date: DateTime<Utc> = offset::Utc::now()
                .checked_add_days(Days::new(detail.occurrence_value.unwrap().try_into()?))
                .unwrap();

            let result =
                check_date_with_given_range(&possible_date, &start_range_date, &end_range_date);

            if result || allow_max_occurrences.unwrap_or(true) {
                possible_date
            } else {
                end_range_date
            }
        }
        model::EndOption::OnThe => {
            let t = DateTime::parse_from_rfc3339(
                // detail.end_date.as_ref().unwrap().as_str()
                detail.end_date.as_ref().unwrap().as_str(),
            )?
            .with_timezone(&Utc);
            t
        }
        model::EndOption::Never => {
            // let t = offset::Utc::now().checked_add_days(Days::new(range_of_days)).unwrap();
            // t
            end_range_date
        }
    };

    let is_with_in_range =
        check_date_with_given_range(&schedule_start, &start_range_date, &end_range_date);

    if detail.end_date.is_none() && !is_with_in_range {
        return Ok(Vec::new());
    }

    // FIX: only if the scheduled_start_date_time equal or past from current date for the different calculation to work
    let diff = end_date - schedule_start;
    let num_days = diff.num_days() / (repeat_times as i64);
    println!("start {} and end {}", &schedule_start, &end_date);

    let mut temp: Vec<DateTime<Utc>> = vec![];

    for _ in 0..num_days {
        temp.push(
            Utc.with_ymd_and_hms(
                schedule_start.year(),
                schedule_start.month(),
                schedule_start.day(),
                scheduled_start_date_time.hour(),
                scheduled_start_date_time.minute(),
                scheduled_start_date_time.second(),
            )
            .unwrap()
            .with_timezone(&Utc),
        );

        schedule_start = schedule_start
            .checked_add_days(Days::new(repeat_times.try_into()?))
            .unwrap();
    }

    temp.push(
        Utc.with_ymd_and_hms(
            schedule_start.year(),
            schedule_start.month(),
            schedule_start.day(),
            scheduled_start_date_time.hour(),
            scheduled_start_date_time.minute(),
            scheduled_start_date_time.second(),
        )
        .unwrap()
        .with_timezone(&Utc),
    );

    Ok(temp)
}
