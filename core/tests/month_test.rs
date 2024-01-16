

use crate::common::{
    assert_diff_between_dates_with_repeated_time, 
    get_start_end_date_month, num_of_diff_for_repeat_every as num_of_diff, get_start_end_date_year,
};
use anyhow::bail;
use chrono::{Utc, DateTime, TimeZone, Datelike};
use common::generate_happy_flow_arguments as common_para_for_test;
use timex::model::ScheduleDetails;
use timex::{schedule_date_times, for_month as unstable_for_month};

#[path = "./common.rs"]
mod common;


fn assert_with_old_api(
    actual: &Vec<DateTime<Utc>>,
        job_details: &ScheduleDetails,
        scheduled_start_date_time: DateTime<Utc>,
            range_date: (DateTime<Utc>, DateTime<Utc>),
) {
    let actual2 = unstable_for_month(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    ).unwrap();

    dbg!(&actual);
    dbg!(&actual2);
    assert_eq!(actual, &actual2, "new api wrong value",); 
}




#[test]
fn it_month_non_stop_on_given_day() {
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

    let t = common_para_for_test(
        sc,
    );
    
    let range_date = t.range_date;
    let job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;
    
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
    
    // let mut temp = HashSet::new();
    // temp.insert(w.day());
    for w in actual {
        assert_eq!(
            w.day() as i64,
            job_details.on_day_value_for_month.unwrap(),
            // onDayValueForMonth
        )
    }
    // assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);

    
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

    let t = common_para_for_test(
        sc,
    );
    
    let range_date = t.range_date;
    let job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;
    

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
    assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);
    
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

    let t = common_para_for_test(
        sc,
    );
    
    let range_date = t.range_date;
    let job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;
    

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
    assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);
    
}

// #[test]
// fn it_n_month_on_n_day_non_stop() {
//     let sc = r#"
//     {
//         "scheduledStartDateTime": "2023-12-14T09:08:44.939Z",
//         "repeatEveryNumber": 2,
//         "repeatEvery": "month",
//         "endOption": "never",
//         "monthOptions": "onDay",
//         "onDayValueForMonth": 1
//       }
//     "#;

//     let t = common_para_for_test(
//         sc,
//     );
    
//     let range_date = t.range_date;
//     let job_details = t.job_details;
//     // let original_schedule = t.original_schedule;
//     let scheduled_start_date_time = t.scheduled_start_date_time;
    

//     // let range_date = get_start_end_date_month();
//     dbg!(&range_date.0);
//     dbg!(&range_date.1);

//     let actual = schedule_date_times(
//         &job_details,
//         scheduled_start_date_time,
//         range_date.0,
//         range_date.1,
//     )
//     .unwrap();

//     dbg!(&actual);

//     assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
//     assert_with_old_api(&actual, &job_details, scheduled_start_date_time, range_date);
    
// }

// TODO: add test case for start and range on month
#[test]
fn it_month_end_date() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2024-01-08T05:28:58.508Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "onThe",
        "endDate": "2024-07-01T18:29:59.999Z",
        "monthOptions": "onDay",
        "onDayValueForMonth": 4,
        "---": "Occur every month on day 1 starting on 2024/01/08 10:58 until 2024/06/31"
    }
    "#;

    let t = common_para_for_test(
        sc,
    );
    
    let range_date = t.range_date;
    let job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;
    
    // let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    ).unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    // TODO: check onDayValueForMonth date of actual
    
}

#[test]
fn it_month_end_date_special_case() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2024-01-08T05:28:58.508Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "onThe",
        "endDate": "2024-07-01T18:29:59.999Z",
        "monthOptions": "onDay",
        "onDayValueForMonth": 31,
        "---": "Occur every month on day 1 starting on 2024/01/08 10:58 until 2024/06/31"
    }
    "#;

    let t = common_para_for_test(
        sc,
    );
    
    let range_date = t.range_date;
    let job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;
    
    // let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    ).unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    // TODO: check end of month date is given
    
}

// TODO: add test case for occurrences on month



// {
//     "scheduledStartDateTime": "2024-01-08T05:28:58.508Z",
//     "repeatEveryNumber": 1,
//     "repeatEvery": "month",
//     "endOption": "onThe",
//     "endDate": "2024-01-31T18:29:59.999Z",
//     "monthOptions": "onThe",
//     "dayCategoryForMonth": "first",
//     "weekDayForMonth": "monday",
//     "##": "Occur every month on first Monday starting on 2024/01/08 10:58 until 2024/01/31"
//   }



#[test]
fn it_month_non_next_year() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2024-10-20T09:08:44.939Z",
        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onDay",
        "onDayValueForMonth": 1,
        "--": "Occur every 2 month on day 1 starting on 2023/12/14 14:38"
      }
    "#;

    let t = common_para_for_test(
        sc,
    );
    
    // let range_date = get_start_end_date_year();
    let range_date = (
        Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2027, 2, 20, 0, 0, 0).unwrap()
    );
    let mut job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;

    // let range_date = get_start_end_date_month();
    dbg!(&range_date.0);
    dbg!(&range_date.1);

    let actual = schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    ).unwrap();

    dbg!(&actual);

    assert_diff_between_dates_with_repeated_time(&actual, &job_details, &scheduled_start_date_time);
    // TODO: check end of month date is given
    
}