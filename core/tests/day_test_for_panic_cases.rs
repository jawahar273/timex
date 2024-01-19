use chrono::{Duration, Utc, Timelike};
use timex::{model::ScheduleDetails, schedule_date_times};

use crate::common::generate_happy_flow_arguments;



#[path = "./common.rs"]
mod common;



#[test]
#[should_panic]
fn it_more_than_31_days() {
    let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 32,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;
    let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();

    let range_date = Utc::now()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();

     schedule_date_times(
        &job_details,
        range_date - Duration::days(1),
        range_date,
        range_date,
    )
    .unwrap();
}

#[test]
fn it_scheduled_date_time_lie_out_of_start_and_end_range_date_past() {
    let sc = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 1,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;

   
   let t = generate_happy_flow_arguments(
    sc,
);

let range_date = t.range_date;
let job_details = t.job_details;
// let original_schedule = t.original_schedule;

    let actual = schedule_date_times(
        &job_details,
        range_date.1 + Duration::days(10),
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);
    assert_eq!(
        actual.len(),
        0,
        "previous schedule date range should lies out side of start and end date range",
    )

}



#[test]
fn it_scheduled_date_time_lie_out_of_start_and_end_range_date_future() {
    let sc = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 1,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;

   
   let t = generate_happy_flow_arguments(
    sc,
);

let range_date = t.range_date;
let mut job_details = t.job_details;
job_details.scheduled_start_date_time = (range_date.1 + Duration::days(1)).to_rfc3339();

    let actual = schedule_date_times(
        &job_details,
        range_date.1 + Duration::days(10),
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);
    assert_eq!(
        actual.len(),
        0,
        "previous schedule date range should lies out side of start and end date range",
    )

}
