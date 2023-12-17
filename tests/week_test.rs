use chrono::{Days, Timelike, Utc, DateTime, TimeZone, IsoWeek, Duration, Datelike};
use schedule::model::ScheduleDetails;
use schedule::{schedule_date_times};
use chrono::prelude::*;

use serde_json;

fn temp(
    scheduled_start_date_time: DateTime<Utc>,
        repeat_every_times: u64,
) -> DateTime<Utc> {
    let t = scheduled_start_date_time+ Duration::weeks(repeat_every_times as i64);

    t
       .with_minute(scheduled_start_date_time.minute()).unwrap()
       .with_hour(scheduled_start_date_time.hour()).unwrap()
       .with_second(scheduled_start_date_time.second()).unwrap()
       .with_nanosecond(0).unwrap()   
}


#[test]
fn it_week_never_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "week",
        "endOption": "never",
        "weekDaysForRepeatEvery": []
      }
    "#;
    
    let job_details: ScheduleDetails = serde_json::from_str(sc).unwrap();
    let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);

    let actual = schedule_date_times(
       &job_details,
    ).unwrap();
    
    let expect = temp(scheduled_start_date_time, job_details.repeat_every_number);

    dbg!(&actual);
    
    assert_eq!(
        expect,
        actual[0],
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
        "occurrenceValue": 3,
        "weekDaysForRepeatEvery": []
      }
    "#;
    
    let job_details: ScheduleDetails = serde_json::from_str(sc).unwrap();
    let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);

    let actual = schedule_date_times(
       &job_details,
    ).unwrap();
    let expect = temp(scheduled_start_date_time, job_details.repeat_every_number);

    dbg!(&actual);
    
    assert_eq!(
        expect,
        actual[0],
    )
    
}



#[test]
fn it_week_occurrence_specific_day_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
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
    let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);

    let actual = schedule_date_times(
       &job_details,
    ).unwrap();
    let expect = temp(scheduled_start_date_time, job_details.repeat_every_number);

    dbg!(&actual);
    dbg!(&actual.len());
    
    assert_eq!(
        expect,
        actual[0],
    )

}