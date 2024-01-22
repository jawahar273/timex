use std::collections::HashSet;

use chrono::{Utc, DateTime, Datelike};
use timex::model::{ScheduleDetails, WeekDayForMonth};
use timex::schedule_date_times;

use crate::common::{
    add_repeat_time, assert_diff_between_dates_with_repeated_time, get_start_end_date_week, generate_happy_flow_arguments as common_para_for_test,
};

#[path = "./common.rs"]
mod common;

use serde_json;



fn assert_check_week_day_for_given_date(
    actual: &Vec<DateTime<Utc>>,
    week_days_for_repeat_every: &Vec<WeekDayForMonth>,
) {
    
    let mut week_day_set = HashSet::new();
    
    for week_day in week_days_for_repeat_every {
        week_day_set.insert(week_day.to_chrono().to_string());
    }

    for a in actual {
        assert_eq!(
            week_day_set.contains(&a.weekday().to_string()),
            true,
            "{}", format!("given date {} was not present in the job detail weekdays", a)
        )
    }
}


#[test]
fn it_week_never_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-10T09:08:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "week",
        "endOption": "never",
        "weekDaysForRepeatEvery": []
      }
    "#;

    let job_details: ScheduleDetails = serde_json::from_str(sc).unwrap();

    let original_schedule =
        chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time)
            .unwrap()
            .with_timezone(&Utc);
    let scheduled_start_date_time = add_repeat_time(
        job_details.repeat_every_number,
        &original_schedule,
        &job_details.repeat_every,
    );

    let range_date = get_start_end_date_week();
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);

    assert_ne!(actual.len(), 0, "every week a date has be produced");

}

#[test]
fn it_week_stop_at_occurrence_of_n() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "week",
        "endOption": "after",
        "occurrenceValue": 20,
        "weekDaysForRepeatEvery": []
      }
    "#;

    let job_details: ScheduleDetails = serde_json::from_str(sc).unwrap();
    let original_schedule =
        chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time)
            .unwrap()
            .with_timezone(&Utc);
    let scheduled_start_date_time = add_repeat_time(
        job_details.repeat_every_number,
        &original_schedule,
        &job_details.repeat_every,
    );
    dbg!(format!("{job_details}"));

    let range_date = get_start_end_date_week();
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    // FIX: check why the occurrenceValue is not satisfied

}

/// Assert between two date might not work as the weekdays involves for the specific week
#[test]
fn it_week_occurrence_specific_day_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-08T09:08:44.939Z",
        "repeatEveryNumber": 2,
        "repeatEvery": "week",
        "endOption": "never",
        "weekDaysForRepeatEvery": [
          "monday",
          "friday"
        ]
      }
    "#;


    // let scheduled_start_date_time = add_repeat_time(
    //     job_details.repeat_every_number,
    //     &original_schedule,
    //     &job_details.repeat_every
    //  );

    let t = common_para_for_test(
        sc,
    );
    
    let job_details: ScheduleDetails = t.job_details;
    let scheduled_start_date_time = t.scheduled_start_date_time;

    let range_date = t.range_date;
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(format!("{job_details}"));
    
    dbg!(&actual);
    dbg!(&actual.len());
    assert_diff_between_dates_with_repeated_time(
        &actual
        .clone()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i % job_details.week_days_for_repeat_every.clone().unwrap().len() != 0)
            .map(|(_, v)| v)
            .collect(),
        &job_details,
        &scheduled_start_date_time
    );

    // assert_eq!(
    //     job_details.week_days_for_repeat_every.clone().unwrap().len() as i64,
    //     actual.len() as i64,
    // );
    
    assert_check_week_day_for_given_date(
        &actual,
        &job_details.week_days_for_repeat_every.clone().unwrap()
    );

}
