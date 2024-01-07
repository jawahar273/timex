use crate::date_diff;
use crate::model::WeekDayForMonth;
use crate::utils::{
    check_date_with_given_range, concat_time, get_start_and_last_date_of_month_for_given_date,
    get_week_bounded_days_for_given_date,
};
use crate::{
    errors::ScheduleError,
    model::{self, RepeatEvery, ScheduleDetails},
};
use anyhow::{bail, Ok, Result as AnyhowResult};
use chrono::{
    offset, DateTime, Datelike, Days, Duration, Months, NaiveDate, TimeZone, Timelike, Utc, Weekday,
};

use log::info;

fn find_all_weekday_for_give_month(
    start: &DateTime<Utc>,
    week_day: &WeekDayForMonth,
) -> Vec<DateTime<Utc>> {
    let month_range = get_start_and_last_date_of_month_for_given_date(start);

    let mut temp = NaiveDate::from_ymd_opt(
        month_range.0.year(),
        month_range.0.month(),
        month_range.0.day(),
    )
    .unwrap();

    let mut result: Vec<DateTime<Utc>> = vec![];
    let mut num_diff = (month_range.1 - month_range.0).num_days();
    if month_range.1.day() == 31 {
        num_diff += 1;
    }

    for _ in 0..num_diff {
        // temp = temp
        //     .checked_add_days(Days::new(inx.try_into().unwrap()))
        //     .unwrap();
        if temp.weekday() == week_day.to_chrono() {
            result.push(temp.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
        temp = temp.succ_opt().unwrap();
    }

    result
}

fn num_diff_i64(
    detail: &ScheduleDetails,
    schedule_start: &DateTime<Utc>,
    end_date: &DateTime<Utc>,
) -> i64 {
    let repeat_times = detail.repeat_every_number;
    match detail.repeat_every {
        RepeatEvery::Day => {
            let diff = *end_date - *schedule_start;
            let num_days = diff.num_days() / (repeat_times as i64);
            num_days
        }
        RepeatEvery::Week => {
            let diff = *end_date - Utc::now();
            let num_of_diff = diff.num_weeks();
            num_of_diff
        }
        RepeatEvery::Month => {
            let diff = date_diff(&Utc::now(), &end_date);
            diff.months
        }
        RepeatEvery::Year => todo!(),
    }
}

fn non_stop_repeat_every_time(detail: &ScheduleDetails) -> bool {
    if detail.end_option == model::EndOption::Never {
        return true;
    }
    false
}

fn get_end_option_after_based_on_repeat(
    detail: &ScheduleDetails,
    start_range_date: &DateTime<Utc>,
    end_range_date: &DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> AnyhowResult<DateTime<Utc>> {
    match detail.repeat_every {
        RepeatEvery::Day => {
            let possible_date: DateTime<Utc> = offset::Utc::now()
                .checked_add_days(Days::new(detail.occurrence_value.unwrap().try_into()?))
                .unwrap();

            let result =
                check_date_with_given_range(&possible_date, &start_range_date, &end_range_date);

            if result || allow_max_occurrences.unwrap_or(true) {
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

            if result || allow_max_occurrences.unwrap_or(true) {
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

    let mut day = detail.on_day_value_for_month.unwrap_or(0) as u32;

    if day >= end_date_of_month.day() {
        day = end_date_of_month.day();
    } else {
        day = scheduled_date.day();

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


pub fn for_details(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> AnyhowResult<Vec<DateTime<Utc>>> {
    let repeat_times = detail.repeat_every_number;

    info!("repeat for {:?}", &detail.repeat_every);
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
            let t = DateTime::parse_from_rfc3339(
                // detail.end_date.as_ref().unwrap().as_str()
                detail.end_date.as_ref().unwrap().as_str(),
            )?
            .with_timezone(&Utc);
            t
        }
        model::EndOption::Never => end_range_date,
    };
    dbg!(end_date);

    let schedule_start: DateTime<Utc> =
        *get_schedule_start(detail, &scheduled_start_date_time)?;

    let is_with_in_range =
        check_date_with_given_range(&schedule_start, &start_range_date, &end_range_date);

    dbg!(is_with_in_range);
    dbg!(&schedule_start);
    if !is_with_in_range {
        info!("scheduled date '{schedule_start}'  range not with given of '{start_range_date}' and '{end_range_date}'");
        return Ok(Vec::new());
    }

    let week_days_for_repeat_every: Vec<String> =
        match detail.week_days_for_repeat_every.clone().is_none() {
            true => vec![],
            false => detail.week_days_for_repeat_every.clone().unwrap(),
        };

    let mut result = Vec::new();

    match detail.repeat_every {
        RepeatEvery::Day => {
            dbg!("on stop non_stop_repeat_every_time");
            let result = non_stop(
                detail,
                &scheduled_start_date_time,
                schedule_start,
                &end_date,
            );
            return result;
        }
        RepeatEvery::Week => {
            if week_days_for_repeat_every.len() >= 1 {
                for week_day in 0..week_days_for_repeat_every.len() {
                    let w = &week_days_for_repeat_every[week_day]
                        .parse::<Weekday>()
                        .unwrap();

                    let u = get_week_bounded_days_for_given_date(&schedule_start);
                    let num = w.num_days_from_monday() as usize;
                    result.push(u[num]);
                }
                return Ok(result);
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

    Ok(result)
}

fn update_before_push(
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
) -> AnyhowResult<Vec<DateTime<Utc>>> {
    let mut result = Vec::new();
    let mut schedule_start = _schedule_start;

    let diff = num_diff_i64(detail, &schedule_start, &end_date);
    dbg!(diff);
    for _ in 0..diff {
        result.push(
            // schedule_start
            update_before_push(detail, &schedule_start, scheduled_start_date_time),
        );
        schedule_start = *get_schedule_start(detail, &schedule_start)?;
    }
    result.push(update_before_push(
        detail,
        &schedule_start,
        scheduled_start_date_time,
    ));
    Ok(result)
}
