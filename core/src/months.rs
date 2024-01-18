use crate::{
    errors::ScheduleError,
    model::{self, EndOption, ScheduleDetails, WeekDayForMonth},
    utils::{check_date_with_given_range, date_diff},
};

use crate::utils::get_start_and_last_date_of_month_for_given_date;

use anyhow::{bail, Ok, Result};
use chrono::{offset, DateTime, Datelike, Months, NaiveDate, TimeZone, Timelike, Utc};

fn non_stop_repeat_every_time(detail: &ScheduleDetails) -> bool {
    if detail.end_option == EndOption::Never {
        return true;
    }
    false
}

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

    for inx in 0..num_diff {
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

#[deprecated(since = "0.2.0", note = "duplicate logic")]
fn unstable_set_date(detail: &ScheduleDetails, scheduled_date: &DateTime<Utc>) -> DateTime<Utc> {
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

#[deprecated(since = "0.2.0", note = "moved to unified api logic")]
pub fn for_month(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    let repeat_times = detail.repeat_every_number;
    if detail.on_day_value_for_month.is_some() && detail.on_day_value_for_month.unwrap() >= 32 {
        bail!(ScheduleError::DaysWithMoreThan31AreNotAllowed());
    }

    let mut schedule_for = scheduled_start_date_time + Months::new(repeat_times as u32);
    dbg!(&schedule_for);

    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => {
            // let t = offset::Utc::now().checked_add_days(IsoWeek::new(detail.occurrence_value.unwrap().try_into()?)).unwrap();
            let possible_date =
                offset::Utc::now() + Months::new(detail.occurrence_value.unwrap().try_into()?);
            //  Duration::weeks();

            let result =
                check_date_with_given_range(&possible_date, &start_range_date, &end_range_date);

            if result {
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
        model::EndOption::Never => end_range_date,
    };

    let is_with_in_range =
        check_date_with_given_range(&schedule_for, &start_range_date, &end_range_date);

    // dbg!(&is_with_in_range);
    // dbg!(&start_range_date);
    // dbg!(&schedule_start);
    // dbg!(&end_range_date);

    if !is_with_in_range {
        return Ok(Vec::new());
    }

    if non_stop_repeat_every_time(detail) {
        // let diff =  - Utc::now();
        let diff = date_diff(&Utc::now(), &end_date);
        let mut temp = Vec::new();

        for _ in 0..diff.months {
            temp.push(unstable_set_date(detail, &schedule_for));
            schedule_for = schedule_for + Months::new(repeat_times as u32);
        }

        temp.push(unstable_set_date(detail, &schedule_for));

        return Ok(temp);
    }

    return Ok(Vec::new());
}


pub fn set_date(detail: &ScheduleDetails, scheduled_date: &DateTime<Utc>) -> DateTime<Utc> {
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
