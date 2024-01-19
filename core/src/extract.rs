use std::cmp::Ordering;

use crate::model::WeekDayForMonth;

use crate::utils::{
    check_date_with_given_range,
    concat_time,
    num_diff_i64,
    get_week_bounded_days_for_given_date
};
use crate::{
    model::{
        self,
        RepeatEvery,
        ScheduleDetails
    },
    months::set_date,
    preprocessor::simple_preprocessing,
};
use anyhow::{ Ok, Result};
use chrono::{
    offset, DateTime,
    Days,
    Duration, Months,
    Utc,
};

use log::info;


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



fn get_schedule_start(
    detail: &ScheduleDetails,
    scheduled_date: &DateTime<Utc>,
) -> Result<Box<DateTime<Utc>>, anyhow::Error> {
    match detail.repeat_every {
        RepeatEvery::Day => {
            return Ok(Box::new(
                *scheduled_date + Days::new(detail.repeat_every_number.try_into()?),
            ));
        }
        RepeatEvery::Week => {
            return Ok(Box::new(
                *scheduled_date
                    + Duration::weeks(detail.repeat_every_number.try_into()?),
            ));
        }
        RepeatEvery::Month => {
            return Ok(Box::new(
                *scheduled_date + Months::new(detail.repeat_every_number.try_into()?),
            ));
        }
        RepeatEvery::Year => todo!(),
    }
}


/// Generate schedule date based on the given parameter
/// [`ScheduleDetails`].
/// 
/// FYI: naming this function yet to finalized.
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
    previous_scheduled_date: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> Result<Vec<DateTime<Utc>>> {
    
    simple_preprocessing(detail)?;

    // End date equivalent to stop at this date
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

    let schedule_at: DateTime<Utc> = *get_schedule_start(
        detail,
        &previous_scheduled_date
    )?;

    let is_with_in_range =
        check_date_with_given_range(&schedule_at, &start_range_date, &end_range_date);

    if !is_with_in_range {
        info!("scheduled date '{schedule_at}'  range not with given of '{start_range_date}' and '{end_range_date}'");
        return Ok(Vec::new());
    }

    match detail.repeat_every {
        RepeatEvery::Day => {
            let result = non_stop(
                detail,
                &previous_scheduled_date,
                schedule_at,
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
                    &schedule_at,
                    &previous_scheduled_date,
                    &end_date,
                    week_days_for_repeat_every,
                );
            } else {
                return non_stop(
                    detail,
                    &previous_scheduled_date,
                    schedule_at,
                    &end_date,
                );
            }
        }
        RepeatEvery::Month => {
            let result = non_stop(
                detail,
                &previous_scheduled_date,
                schedule_at,
                &end_date,
            );
            return result;
        },
        RepeatEvery::Year => todo!(),
    }

}

fn post_processing_output(
    detail: &ScheduleDetails,
    schedule_at: &DateTime<Utc>,
    previous_scheduled_date: &DateTime<Utc>,
) -> DateTime<Utc> {
    match detail.repeat_every {
        RepeatEvery::Day => concat_time(*schedule_at, *previous_scheduled_date),
        RepeatEvery::Week => concat_time(*schedule_at, *previous_scheduled_date),
        RepeatEvery::Month => set_date(detail, &schedule_at),
        RepeatEvery::Year => todo!(),
    }
}

fn non_stop(
    detail: &ScheduleDetails,
    previous_scheduled_date: &DateTime<Utc>,
    _schedule_at: DateTime<Utc>,
    end_date: &DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    let mut result = Vec::new();
    let mut schedule_at = _schedule_at;

    let diff = num_diff_i64(detail, &schedule_at, &end_date);

    for _ in 0..diff {
        result.push(
            // schedule_start
            post_processing_output(detail, &schedule_at, previous_scheduled_date),
        );
        schedule_at = *get_schedule_start(detail, &schedule_at)?;
        
    }

    result.push(post_processing_output(
        detail,
        &schedule_at,
        previous_scheduled_date,
    ));
    Ok(result)
}


fn week_day_loop(
    detail: &ScheduleDetails,
    _schedule_at: &DateTime<Utc>,
    previous_scheduled_date: &DateTime<Utc>,
    end_date: &DateTime<Utc>,
    week_days_for_repeat_every: Vec<WeekDayForMonth>,
) -> Result<Vec<DateTime<Utc>>> {
    let mut result = Vec::new();
    let mut schedule_at: DateTime<Utc> = *_schedule_at;

    let diff = num_diff_i64(detail, &schedule_at, &end_date);
    for _ in 0..diff {        
        for week_day in 0..week_days_for_repeat_every.len() {
            let w = &week_days_for_repeat_every[week_day]
                .to_chrono();

            let u = get_week_bounded_days_for_given_date(&schedule_at);
            let num = w.num_days_from_monday() as usize;
            result.push(
                post_processing_output(
                    detail,
                    &u[num],
                    previous_scheduled_date,
                )
            );
        }
        schedule_at = *get_schedule_start(detail, &schedule_at)?;
    }

   return Ok(result);
    
}