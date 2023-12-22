use chrono::{DateTime, Duration, Utc, Days};
use schedule::{
    get_week_bounded_days_for_given_date,
    model::{RepeatEvery, ScheduleDetails},
};



pub fn get_start_end_date_week() -> (DateTime<Utc>, DateTime<Utc>) {
    let bounded_weekdays = get_week_bounded_days_for_given_date(&Utc::now());

    let start_range_date = bounded_weekdays[0];
    let end_range_date = bounded_weekdays[6];

    return (start_range_date, end_range_date);
}

pub fn num_of_diff(
    diff_duration: &Duration,
    repeat_every: &RepeatEvery,
    _previous_scheduled_start: &DateTime<Utc>,
) -> i64 {
    match repeat_every {
        RepeatEvery::Day => diff_duration.num_days(),
        RepeatEvery::Week => diff_duration.num_weeks(),
        // FIX: confirm month
        RepeatEvery::Month => {
            todo!();
            //    (previous_scheduled_start.month() - _date.month()) as i64
        }
        RepeatEvery::Year => {
            todo!();

            //    (previous_scheduled_start.year() - _date.year()) as i64
        }
    }
}

pub fn assert_diff_between_dates_with_repeated_time(
    actual_dates: &Vec<DateTime<Utc>>,
    details: &ScheduleDetails,
    previous_scheduled_start: &DateTime<Utc>,
) {
    let repeat_time = details.repeat_every_number as i64;

    if actual_dates.len() == 0 {
        return;
    }

    let mut previous_date: DateTime<Utc> = actual_dates[0];

    for inx in 1..actual_dates.len() {
        let diff_duration = actual_dates[inx] - previous_date;
        let num_diff = num_of_diff(
            &diff_duration,
            &details.repeat_every,
            previous_scheduled_start,
        );
        
        // FIX: ignore today on compare
        assert_eq!(repeat_time, num_diff,);
        previous_date = actual_dates[inx]
    }
}


pub fn add_repeat_time(
    repeat_time: u64,
    original_schedule: &DateTime<Utc>,
    repeat: &RepeatEvery,
) -> DateTime<Utc> {
    let r = (Utc::now() - original_schedule).abs();
    match repeat {
        RepeatEvery::Day => {
            Utc::now() - Days::new((r.num_days() / (repeat_time as i64)).try_into().unwrap())
        },
        RepeatEvery::Week => {
            Utc::now() - Duration::weeks((r.num_weeks() / (repeat_time as i64)).try_into().unwrap())
        },
        RepeatEvery::Month => todo!(),
        RepeatEvery::Year => todo!(),
    }
}