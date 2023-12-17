


use chrono::{Days, Timelike, Utc, DateTime};
use schedule::model::ScheduleDetails;
use schedule::{schedule_date_times};

use serde_json;

fn temp(scheduled_start_date_time: DateTime<Utc>, times: u64) -> DateTime<Utc> {
   scheduled_start_date_time
   .checked_add_days(Days::new(times))
      .unwrap()
      .with_minute(scheduled_start_date_time.minute()).unwrap()
      .with_hour(scheduled_start_date_time.hour()).unwrap()
      .with_second(scheduled_start_date_time.second()).unwrap()
      .with_nanosecond(0).unwrap()
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
   let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);
   
   let t = schedule_date_times(
      &job_details,
   ).unwrap();

   dbg!(&t);

   assert_eq!(
      temp(scheduled_start_date_time, job_details.repeat_every_number),
      t[0],
   );
   // println!("{result}");
}

#[test]
fn it_daily_never_stop_repeat_every_3() {
   
   let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 3,
      "repeatEvery": "day",
      "endOption": "never"
    }
   "#;
   let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
   let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);
   
   let t = schedule_date_times(
      &job_details,
   ).unwrap();

   dbg!(&t);

   assert_eq!(
      temp(scheduled_start_date_time, job_details.repeat_every_number),
      t[0],
   );
   // println!("{result}");
}


#[test]
fn it_daily_stop_n_occurrence() {
   
   let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T08:00:44.939Z",
      "repeatEveryNumber": 2,
      "repeatEvery": "day",
      "endOption": "after",
      "occurrenceValue": 3
    }
   "#;
   let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
   let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);
   
   let t = schedule_date_times(
      &job_details,
   ).unwrap();

   dbg!(&t);

   assert_eq!(
      temp(scheduled_start_date_time, job_details.repeat_every_number),
      t[0],
   );

   assert_eq!(t.len() as u64, job_details.occurrence_value.unwrap());
}


#[test]
fn it_daily_with_end_date() {
   
   let t = r#"
   {
      "scheduledStartDateTime": "2023-12-14T06:54:20.447Z",
      "repeatEveryNumber": 3,
      "repeatEvery": "day",
      "endOption": "onThe",
      "endDate": "2023-12-30T18:29:59.999Z"
    }
   "#;
   let job_details: ScheduleDetails = serde_json::from_str(&t).unwrap();
   let scheduled_start_date_time = chrono::DateTime::parse_from_rfc3339(&job_details.scheduled_start_date_time).unwrap().with_timezone(&Utc);
   
   let t = schedule_date_times(
      &job_details,
   ).unwrap();

   dbg!(&t);

   assert_eq!(
      temp(scheduled_start_date_time, job_details.repeat_every_number),
      t[0],
   );
   // println!("{result}");
}
