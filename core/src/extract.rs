use std::cmp::Ordering;

use crate::model::WeekDayForMonth;
use crate::utils::{
    check_date_with_given_range,
    concat_time,
    get_start_and_last_date_of_month_for_given_date,
    get_week_bounded_days_for_given_date,
    num_diff_i64
};
use crate::{
    errors::ScheduleError,
    model::{
        self,
        RepeatEvery,
        ScheduleDetails
    },
    weeks::find_all_weekday_for_give_month
};
use anyhow::{bail, Ok, Result};
use chrono::{
    offset, DateTime,
    Datelike, Days,
    Duration, Months,
    TimeZone, Timelike,
    Utc,
};

use log::info;


// fn non_stop_repeat_every_time(detail: &ScheduleDetails) -> bool {
//     if detail.end_option == model::EndOption::Never {
//         return true;
//     }
//     false
// }

fn get_end_option_after_based_on_repeat(
    detail: &ScheduleDetails,
    start_range_date: &DateTime<Utc>,
    end_range_date: &DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> Result<DateTime<Utc>> {
    match detail.repeat_every {
        RepeatEvery::Day => {
            let possible_date: DateTime<Utc> = offset::Utc::now()
                .checked_add_days(Days::new(detail.occurrence_value.unwrap().try_into()?))
                .unwrap();

            let result =
                check_date_with_given_range(&possible_date, &start_range_date, &end_range_date);

            if result || allow_max_occurrences.unwrap_or(false) {
                Ok(possible_date)
            } else {
                Ok(*end_range_date)
            }
        }
        RepeatEvery::Week => {
            let possible_date =
                offset::Utc::now() + Duration::weeks(detail.occurrence_value.unwrap().try_into()?);

            let result =
                check_date_with_given_range(&possible_date, &start_range_date, &end_range_date);

            if result || allow_max_occurrences.unwrap_or(false) {
                Ok(possible_date)
            } else {
                Ok(*end_range_date)
            }
        },
        RepeatEvery::Month => {
            let possible_date =
                offset::Utc::now() + Months::new(detail.occurrence_value.unwrap().try_into()?);
            //  Duration::weeks();

            let result =
                check_date_with_given_range(&possible_date, &start_range_date, &end_range_date);

            if result {
                Ok(possible_date)
            } else {
                Ok(*end_range_date)
            }
        },
        RepeatEvery::Year => todo!(),
    }
}

fn set_date(detail: &ScheduleDetails, scheduled_date: &DateTime<Utc>) -> DateTime<Utc> {
    let y = get_start_and_last_date_of_month_for_given_date(&scheduled_date);
    let end_date_of_month = y.1;
    let on_day_value_for_month = detail.on_day_value_for_month.unwrap_or(0) as u32;

    let mut day = detail.on_day_value_for_month.unwrap_or(0) as u32;

    if day >= end_date_of_month.day() {
        day = end_date_of_month.day();
    } else {
        day = scheduled_date.day();
        
        // when it has week days date are change accounting to select week day date
        if detail.week_day_for_month.is_some() {
            let temp = find_all_weekday_for_give_month(
                scheduled_date,
                detail.week_day_for_month.as_ref().unwrap(),
            );
            let y = detail
                .day_category_for_month
                .as_ref()
                .unwrap()
                .to_week_in_month();
            // dbg!(&scheduled_date);
            // dbg!(&y);
            // dbg!(&temp);
            // dbg!(&temp.get(y as usize));

            let i = match y {
                -1 => temp.last(),
                v => temp.get(v as usize),
            };

            day = i.unwrap().day();
        } else if on_day_value_for_month > end_date_of_month.day() {
            day = end_date_of_month.day();
        } else if on_day_value_for_month <= end_date_of_month.day() {
            day = on_day_value_for_month
        }
    }

    Utc.with_ymd_and_hms(
        scheduled_date.year(),
        scheduled_date.month(),
        day as u32,
        scheduled_date.hour(),
        scheduled_date.minute(),
        scheduled_date.second(),
    )
    .unwrap()
}

fn get_schedule_start(
    detail: &ScheduleDetails,
    scheduled_start_date_time: &DateTime<Utc>,
) -> Result<Box<DateTime<Utc>>, anyhow::Error> {
    match detail.repeat_every {
        RepeatEvery::Day => {
            return Ok(Box::new(
                *scheduled_start_date_time + Days::new(detail.repeat_every_number.try_into()?),
            ));
        }
        RepeatEvery::Week => {
            return Ok(Box::new(
                *scheduled_start_date_time
                    + Duration::weeks(detail.repeat_every_number.try_into()?),
            ));
        }
        RepeatEvery::Month => {
            return Ok(Box::new(
                *scheduled_start_date_time + Months::new(detail.repeat_every_number.try_into()?),
            ));
        }
        RepeatEvery::Year => todo!(),
    }
}


/// Generate schedule date based on the given parameter
/// [`ScheduleDetails`], 
///
/// # Panics
///
/// Panics if unexpected or [`Result::expect`] function are trigger
///
/// # Errors
///
/// This function will return an error if conversion or unexpected value for parameter are
/// given
pub fn for_details(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> Result<Vec<DateTime<Utc>>> {

    // preprocessing step
    match detail.repeat_every {
        RepeatEvery::Day => {
            if detail.repeat_every_number >= 32 {
                bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
            }
        }
        RepeatEvery::Week => {}
        RepeatEvery::Month => {
            if detail.on_day_value_for_month.is_some()
                && detail.on_day_value_for_month.unwrap() >= 32
            {
                bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
            }
        }
        RepeatEvery::Year => {}
    }

    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => get_end_option_after_based_on_repeat(
            detail,
            &start_range_date,
            &end_range_date,
            allow_max_occurrences,
        )?,
        model::EndOption::OnThe => {
            
            let end_date = DateTime::parse_from_rfc3339(
                // detail.end_date.as_ref().unwrap().as_str()
                detail.end_date.as_ref().unwrap().as_str(),
            )?
            .with_timezone(&Utc);
        
            match end_range_date.cmp(&end_date) {
                Ordering::Equal => {
                    end_date
                },
                Ordering::Greater => {
                    end_date
                }
                Ordering::Less => {
                    // when future date
                    end_range_date
                }
            }
            // dbg!(t);
            // end_date
        }
        model::EndOption::Never => end_range_date,
    };

    let schedule_start: DateTime<Utc> =
        *get_schedule_start(detail, &scheduled_start_date_time)?;
    dbg!(&schedule_start);
    let is_with_in_range =
        check_date_with_given_range(&schedule_start, &start_range_date, &end_range_date);

    if !is_with_in_range {
        info!("scheduled date '{schedule_start}'  range not with given of '{start_range_date}' and '{end_range_date}'");
        return Ok(Vec::new());
    }

    // let mut result = Vec::new();

    match detail.repeat_every {
        RepeatEvery::Day => {
            let result = non_stop(
                detail,
                &scheduled_start_date_time,
                schedule_start,
                &end_date,
            );
            return result;
        }
        RepeatEvery::Week => {

            let week_days_for_repeat_every: Vec<WeekDayForMonth> =
                match detail.week_days_for_repeat_every.clone().is_none() {
                    true => vec![],
                    false => detail.week_days_for_repeat_every.clone().unwrap(),
                };
        
            if week_days_for_repeat_every.len() >= 1 {

                return week_day_loop(
                    detail,
                    &schedule_start,
                    &scheduled_start_date_time,
                    &end_date,
                    week_days_for_repeat_every,
                );
            } else {
                return non_stop(
                    detail,
                    &scheduled_start_date_time,
                    schedule_start,
                    &end_date,
                );
            }
        }
        RepeatEvery::Month => {
            let result = non_stop(
                detail,
                &scheduled_start_date_time,
                schedule_start,
                &end_date,
            );
            return result;
        },
        RepeatEvery::Year => todo!(),
    }

}

fn post_processing_output(
    detail: &ScheduleDetails,
    schedule_start: &DateTime<Utc>,
    scheduled_start_date_time: &DateTime<Utc>,
) -> DateTime<Utc> {
    match detail.repeat_every {
        RepeatEvery::Day => concat_time(*schedule_start, *scheduled_start_date_time),
        RepeatEvery::Week => concat_time(*schedule_start, *scheduled_start_date_time),
        RepeatEvery::Month => set_date(detail, &schedule_start),
        RepeatEvery::Year => todo!(),
    }
}

fn non_stop(
    detail: &ScheduleDetails,
    scheduled_start_date_time: &DateTime<Utc>,
    _schedule_start: DateTime<Utc>,
    end_date: &DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    let mut result = Vec::new();
    let mut schedule_start = _schedule_start;

    let diff = num_diff_i64(detail, &schedule_start, &end_date);

    for _ in 0..diff {
        result.push(
            // schedule_start
            post_processing_output(detail, &schedule_start, scheduled_start_date_time),
        );
        schedule_start = *get_schedule_start(detail, &schedule_start)?;
        
    }

    result.push(post_processing_output(
        detail,
        &schedule_start,
        scheduled_start_date_time,
    ));
    Ok(result)
}


fn week_day_loop(
    detail: &ScheduleDetails,
    _schedule_start: &DateTime<Utc>,
    scheduled_start_date_time: &DateTime<Utc>,
    end_date: &DateTime<Utc>,
    week_days_for_repeat_every: Vec<WeekDayForMonth>,
) -> Result<Vec<DateTime<Utc>>> {
    let mut result = Vec::new();
    let mut schedule_start: DateTime<Utc> = *_schedule_start;

    let diff = num_diff_i64(detail, &schedule_start, &end_date);
    for _ in 0..diff {        
        for week_day in 0..week_days_for_repeat_every.len() {
            let w = &week_days_for_repeat_every[week_day]
                .to_chrono();

            let u = get_week_bounded_days_for_given_date(&schedule_start);
            let num = w.num_days_from_monday() as usize;
            result.push(
                post_processing_output(
                    detail,
                    &u[num],
                    scheduled_start_date_time,
                )
            );
        }
        schedule_start = *get_schedule_start(detail, &schedule_start)?;
    }

   return Ok(result);
    
}