use crate::common::{
    assert_diff_between_dates_with_repeated_time, get_start_end_date_week, num_of_diff,
};
use chrono::{DateTime, Days, Duration, Timelike, Utc};
use common::add_repeat_time;
use timex::model::ScheduleDetails;
use timex::{
    schedule_date_times,
    unstable_for_details,
    unstable_for_days,
    unstable_get_start_and_last_date_of_month_for_given_date as get_start_and_last_date_of_month_for_given_date,
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
    let actual2 = unstable_for_days(
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
fn it_day_today() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 1,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
    // let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);
    let scheduled_start_date_time = Utc::now();

    let range_date = Utc::now()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
    dbg!(range_date);
    let actual = schedule_date_times(
        &job_details,
        range_date - Duration::days(1),
        range_date,
        range_date,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time)
    // println!("{result}");
}

#[test]
fn it_daily_never_stop() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 1,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
    // let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);
    let scheduled_start_date_time = Utc::now();

    let range_date = get_start_end_date_week();
    dbg!(range_date);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    
    assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);

    // println!("{result}");
}

#[test]
fn it_daily_never_stop_repeat_every_2() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-12T08:00:44.939Z",
      "repeatEveryNumber": 2,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();

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
    dbg!(&scheduled_start_date_time);
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);


assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);

}

#[test]
fn it_daily_stop_n_occurrence() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-11T08:00:44.939Z",
      "repeatEveryNumber": 2,
      "repeatEvery": "day",
      "endOption": "after",
      "occurrenceValue": 4
    }
   "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();

    dbg!(format!("{job_details}"));

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


    assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);


    // FIX: check why the occurrenceValue is not satisfied
    // assert_eq!(t.len() as u64, job_details.occurrence_value.unwrap());
}

#[test]
fn it_daily_with_end_date() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T06:54:20.447Z",
      "repeatEveryNumber": 2,
      "repeatEvery": "day",
      "endOption": "onThe",
      "endDate": "2024-01-01T18:29:59.999Z"
    }
   "#;
    let mut job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();

    let original_schedule =
        chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time)
            .unwrap()
            .with_timezone(&Utc);
    let scheduled_start_date_time = add_repeat_time(
        job_details.repeat_every_number,
        &original_schedule,
        &job_details.repeat_every,
    );

    let range_date = get_start_and_last_date_of_month_for_given_date(&scheduled_start_date_time);
    job_details.end_date = Some(range_date.1.to_rfc3339());
    let end_Date = range_date.1;

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);
    dbg!("--=========================================================-");
    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);



    dbg!(&actual);


    let i = num_of_diff(
        &(end_Date - scheduled_start_date_time),
        &job_details.repeat_every,
        &scheduled_start_date_time,
    );

    assert_eq!(
        actual.len() as i64,
        i / (job_details.repeat_every_number as i64),
        "start and end date occurrence should be equal"
    );

    assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);

}
