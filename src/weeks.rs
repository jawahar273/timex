use crate::utils::{check_date_with_given_range, concat_time};
use crate::{
    model::{self, ScheduleDetails},
    utils::get_week_bounded_days_for_given_date,
};
use anyhow::{Ok, Result};
use chrono::{
    offset, DateTime, Duration, Utc,
    Weekday,
};

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

pub fn for_week(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
    allow_max_occurrences: Option<bool>,
    // end_date: DateTime<Utc>,
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

    let schedule_start = Box::new(scheduled_start_date_time + Duration::weeks(repeat_times as i64));

    let is_with_in_range =
        check_date_with_given_range(&schedule_start, &start_range_date, &end_range_date);

    dbg!(&is_with_in_range);
    dbg!(&start_range_date);
    dbg!(&schedule_start);
    dbg!(&end_range_date);

    if !is_with_in_range {
        return Ok(Vec::new());
    }

    let week_days_for_repeat_every: Vec<String> =
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
            .parse::<Weekday>()
            .unwrap();

        let u = get_week_bounded_days_for_given_date(&schedule_start);
        let num = w.num_days_from_monday() as usize;
        result.push(u[num]);

        // *schedule_start = *schedule_start + Duration::weeks(repeat_times as i64);
    }
    // for  week_day in 0..week_days_for_repeat_every.len() {
    //     let w = &week_days_for_repeat_every[week_day].parse::<Weekday>().unwrap();
    //     let num = w.num_days_from_monday();
    //     let diff_abs = (Utc::now().weekday().num_days_from_monday() - w.num_days_from_monday()) as i64;

    //     let sr = *schedule_start + Duration::days(
    //         diff_abs
    //     );

    //     let y = temp_result(
    //         sr,
    //         end_date,
    //         scheduled_start_date_time,
    //         repeat_times as i64,
    //     );
    //     result.extend(
    //         y
    //     );

    // }

    Ok(result)
}
