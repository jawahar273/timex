use crate::utils::{
    check_date_with_given_range, concat_time,
    get_start_and_last_date_of_month_for_given_date
};
use crate::{
    model::{self, ScheduleDetails, WeekDayForMonth},
    utils::get_week_bounded_days_for_given_date,
};
use anyhow::{Ok, Result};
use chrono::{
    offset, DateTime, Duration, Utc,
     Datelike, NaiveDate, Weekday,
    
};
use log::info;


#[deprecated(since = "0.2.0", note = "duplicate logic")]
pub fn find_all_weekday_for_give_month(
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

#[deprecated(since = "0.2.0", note = "duplicate logic")]
fn temp_result(
    sr: &Box<DateTime<Utc>>,
    end_date: DateTime<Utc>,
    original_scheduled_start_date_time: DateTime<Utc>,
    repeat_times: i64,
) -> Vec<DateTime<Utc>> {
    let mut schedule_start = **sr;
    let diff = end_date - Utc::now();
    let num_of_diff = diff.num_weeks();

    let mut temp: Vec<DateTime<Utc>> = vec![];

    for _ in 0..num_of_diff {
        temp.push(concat_time(
            schedule_start,
            original_scheduled_start_date_time,
        ));

        // linear case
        schedule_start = schedule_start + Duration::weeks(repeat_times as i64);
    }

    temp.push(concat_time(
        schedule_start,
        original_scheduled_start_date_time,
    ));
    temp
}

#[deprecated(since = "0.2.0", note = "moved to unified api logic")]
pub fn for_week(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
) -> Result<Vec<DateTime<Utc>>> {
    let repeat_times = detail.repeat_every_number;

    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => {
            // let t = offset::Utc::now().checked_add_days(IsoWeek::new(detail.occurrence_value.unwrap().try_into()?)).unwrap();
            let possible_date =
                offset::Utc::now() + Duration::weeks(detail.occurrence_value.unwrap().try_into()?);

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
        model::EndOption::Never => end_range_date,
    };
    dbg!(&scheduled_start_date_time);
    let schedule_start = Box::new(scheduled_start_date_time + Duration::weeks(repeat_times as i64));

    let is_with_in_range =
        check_date_with_given_range(&schedule_start, &start_range_date, &end_range_date);

    if !is_with_in_range {
        info!("scheduled date '{schedule_start}'  range not with given of '{start_range_date}' and '{end_range_date}'");
        return Ok(Vec::new());
    }

    let week_days_for_repeat_every: Vec<WeekDayForMonth> =
        detail.week_days_for_repeat_every.clone().unwrap();

    let mut result: Vec<DateTime<Utc>> = vec![];
    // let mut result: Vec< Vec< DateTime<Utc>> >= vec![];

    if week_days_for_repeat_every.len() == 0 {
        let u = temp_result(
            &schedule_start,
            end_date,
            scheduled_start_date_time,
            repeat_times as i64,
        );
        return Ok(u);
    }

    for week_day in 0..week_days_for_repeat_every.len() {
        let w = &week_days_for_repeat_every[week_day]
            .to_chrono();

        let u = get_week_bounded_days_for_given_date(&schedule_start).unwrap();
        let num = w.num_days_from_monday() as usize;
        result.push(u[num]);
    }

    Ok(result)
}
