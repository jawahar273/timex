
use common::generate_happy_flow_arguments as common_para_for_test;
use timex::schedule_date_times;

#[path = "./common.rs"]
mod common;

#[test]
#[should_panic]
fn it_month_32_days() {
    let sc = r#"
    {
        "scheduledStartDateTime": "2023-11-30T09:08:44.939Z",
        "repeatEveryNumber": 2,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onDay",
        "onDayValueForMonth": 32,
        "--": "Occur every 2 month on day 32 starting on 2023/12/14 14:38"
      }
    "#;

    let t = common_para_for_test(
        sc,
    );
    
    let range_date = t.range_date;
    let job_details = t.job_details;
    // let original_schedule = t.original_schedule;
    let scheduled_start_date_time = t.scheduled_start_date_time;
    
    schedule_date_times(
        &job_details,
        scheduled_start_date_time,
        range_date.0,
        range_date.1,
    )
    .unwrap();

    
}
