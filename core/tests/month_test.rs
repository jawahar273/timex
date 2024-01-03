use crate::common::{
    assert_diff_between_dates_with_repeated_time, get_start_end_date_month, num_of_diff,
};
use chrono::Utc;
use common::add_repeat_time;
use timex_core::model::ScheduleDetails;
use timex_core::schedule_date_times;

#[path = "./common.rs"]
mod common;

#[test]
fn it_month_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-11-30T09:08:44.939Z",
        "repeatEveryNumber": 2,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onDay",
        "onDayValueForMonth": 1,
        "--": "Occur every 2 month on day 1 starting on 2023/12/14 14:38"
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

    let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);
    dbg!(format!("{job_details}"));

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    assert_ne!(actual.len(), 0, "every month a date has be produced");
}

#[test]
fn it_month_first_monday_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onThe",
        "dayCategoryForMonth": "first",
        "weekDayForMonth": "monday",
        "--": "Occur every month on first Monday starting on 2023/12/14 14:38"
      }
    "#;
    
    
    // let sc = r#"
    // {
    //         "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
    //         "repeatEveryNumber": 1,
    //         "repeatEvery": "month",
    //         "endOption": "never",
    //         "monthOptions": "onThe",
    //         "dayCategoryForMonth": "last",
    //         "weekDayForMonth": "monday",
    //         "--": "Occur every month on first Monday starting on 2023/12/14 14:38"
    //       }
    // "#;

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

    let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    // TODO: check for weekday is correct
}


#[test]
fn it_month_special_case_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onDay",
        "onDayValueForMonth": 31,
        "---": "special case"
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

    let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    // TODO: add test to check end date of the month is equal to given date's end date of the month
}




#[test]
fn it_n_month_on_n_day_non_stop() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
        "repeatEveryNumber": 2,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onDay",
        "onDayValueForMonth": 1
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

    let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
}

// TODO: add test case for start and range on month
// TODO: add test case for occurrences on month