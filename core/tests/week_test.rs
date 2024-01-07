use chrono::{Utc, DateTime};
use timex::model::ScheduleDetails;
use timex::{
    schedule_date_times,
    for_week as unstable_for_week,
};

use crate::common::{
    add_repeat_time, assert_diff_between_dates_with_repeated_time, get_start_end_date_week,
};

#[path = "./common.rs"]
mod common;

use serde_json;


fn assert_with_old_api(
    actual: &Vec<DateTime<Utc>>,
        job_details: &ScheduleDetails,
        scheduled_start_date_time: DateTime<Utc>,
            range_date: (DateTime<Utc>, DateTime<Utc>),
) {
    let actual2 = unstable_for_week(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
        Some(true),
    ).unwrap();

    dbg!(&actual);
    dbg!(&actual2);
    assert_eq!(actual, &actual2, "new api wrong value",); 
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
    assert_with_old_api(
        &actual,
        &job_details,
        scheduled_start_date_time,
        range_date,
    )
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
    assert_with_old_api(
        &actual,
        &job_details,
        scheduled_start_date_time,
        range_date,
    );
}

#[test]
#[ignore] // TODO: find better test for find weekdays.
fn it_week_occurrence_specific_day_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-08T09:08:44.939Z",
        "repeatEveryNumber": 2,
        "repeatEvery": "week",
        "endOption": "never",
        "weekDaysForRepeatEvery": [
          "monday",
          "wednesday",
          "thursday",
          "friday"
        ]
      }
    "#;

    let job_details: ScheduleDetails = serde_json::from_str(sc).unwrap();
    let scheduled_start_date_time =
        chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time)
            .unwrap()
            .with_timezone(&Utc);
    dbg!(format!("{job_details}"));

    // let scheduled_start_date_time = add_repeat_time(
    //     job_details.repeat_every_number,
    //     &original_schedule,
    //     &job_details.repeat_every
    //  );

    let range_date = get_start_end_date_week();
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);
    dbg!(&actual.len());
    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);

    assert_eq!(
        job_details.week_days_for_repeat_every.clone().unwrap().len() as i64,
        actual.len() as i64,
    );
    
    assert_with_old_api(
        &actual,
        &job_details,
        scheduled_start_date_time,
        range_date,
    );
}
