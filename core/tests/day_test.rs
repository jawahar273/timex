use crate::common::{
    assert_diff_between_dates_with_repeated_time,
    get_start_end_date_week, num_of_diff,
    
};
use chrono::{DateTime, Days, Timelike, Utc, Duration};
use common::add_repeat_time;
use timex::model::ScheduleDetails;
use timex::schedule_date_times;

#[path = "./common.rs"]
mod common;

// ::get_start_end_date_week;

use serde_json;

fn temp(scheduled_start_date_time: DateTime<Utc>, times: u64) -> DateTime<Utc> {
    scheduled_start_date_time
        .checked_add_days(Days::new(times))
        .unwrap()
        .with_minute(scheduled_start_date_time.minute())
        .unwrap()
        .with_hour(scheduled_start_date_time.hour())
        .unwrap()
        .with_second(scheduled_start_date_time.second())
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
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

    let range_date = Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap();
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
    
    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time)
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
    let scheduled_start_date_time =
        add_repeat_time(
         job_details.repeat_every_number,
         &original_schedule,
         &job_details.repeat_every
      );

    let range_date = get_start_end_date_week();
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();
    
    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time)
    // println!("{result}");
}

#[test]
fn it_daily_stop_n_occurrence() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-11T08:00:44.939Z",
      "repeatEveryNumber": 2,
      "repeatEvery": "day",
      "endOption": "after",
      "occurrenceValue": 100
    }
   "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
    
    dbg!(format!("{job_details}"));
    
    let original_schedule =
        chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time)
            .unwrap()
            .with_timezone(&Utc);
    let scheduled_start_date_time =
        add_repeat_time(
         job_details.repeat_every_number,
         &original_schedule,
         &job_details.repeat_every
         
      );

    let range_date = get_start_end_date_week();
    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time)
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
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();

    let scheduled_start_date_time =
        chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time)
            .unwrap()
            .with_timezone(&Utc);
    // let scheduled_start_date_time = add_repeat_time(job_details.repeat_every_number, original_schedule);
    let end_Date =
        chrono::DateTime::parse_from_rfc3339(&job_details.end_date.clone().unwrap().clone())
            .unwrap()
            .with_timezone(&Utc);

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
}
