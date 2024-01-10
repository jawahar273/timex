use chrono::{DateTime, Days, Duration, Months, Utc};
use timex::{
    date_diff,
    model::{RepeatEvery, ScheduleDetails},
    unstable_get_start_and_last_date_of_month_for_given_date as get_start_and_last_date_of_month_for_given_date,
    unstable_get_week_bounded_days_for_given_date as get_week_bounded_days_for_given_date,
    unstable_get_start_and_last_of_year as get_start_and_last_of_year
};

pub fn get_start_end_date_week() -> (DateTime<Utc>, DateTime<Utc>) {
    let bounded_weekdays = get_week_bounded_days_for_given_date(&Utc::now());

    let start_range_date = bounded_weekdays[0];
    let end_range_date = bounded_weekdays[6];

    return (start_range_date, end_range_date);
}

pub fn get_start_end_date_month() -> (DateTime<Utc>, DateTime<Utc>) {
    let a = get_start_and_last_date_of_month_for_given_date(&Utc::now());
    let b = get_start_and_last_date_of_month_for_given_date(&(a.1 + Days::new(1)));
    let c = get_start_and_last_date_of_month_for_given_date(&(b.1 + Days::new(1)));
    let d = get_start_and_last_date_of_month_for_given_date(&(c.1 + Days::new(1)));
    let e = get_start_and_last_date_of_month_for_given_date(&(d.1 + Days::new(1)));

    dbg!(&d.0);
    dbg!(&d.1);

    (a.0, e.1)
}

pub fn get_start_end_date_year() -> (DateTime<Utc>, DateTime<Utc>) {
    let a = get_start_and_last_of_year(&Utc::now());
    let b = get_start_and_last_of_year(&(a.1 + Days::new(1)));
    let c = get_start_and_last_of_year(&(b.1 + Days::new(1)));
    let d = get_start_and_last_of_year(&(c.1 + Days::new(1)));
    
    (a.0, d.1)
}

pub fn num_of_diff(
    diff_duration: &Duration,
    repeat_every: &RepeatEvery,
    previous_scheduled_start: &DateTime<Utc>,
) -> i64 {
    match repeat_every {
        RepeatEvery::Day => diff_duration.num_days(),
        RepeatEvery::Week => diff_duration.num_weeks(),
        // FIX: confirm month
        RepeatEvery::Month => {
            date_diff(previous_scheduled_start, &Utc::now()).months
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
        assert_eq!(
            repeat_time,
            num_diff,
            "{}",
            format!(
                "date between is not consistent expected one is {}",
                &details.repeat_every_number
            )
        );
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
        RepeatEvery::Day => Utc::now() - Days::new(repeat_time.try_into().unwrap()),
        RepeatEvery::Week => Utc::now() - Duration::weeks((repeat_time) as i64),
        RepeatEvery::Month => Utc::now() - Months::new(repeat_time.try_into().unwrap()),
        RepeatEvery::Year => todo!(),
    }
}

pub struct CTemp {
    pub job_details: ScheduleDetails,
    pub range_date: (DateTime<Utc>, DateTime<Utc>),
    pub scheduled_start_date_time: DateTime<Utc>,
    // original_schedule: DateTime<Utc>,
}

pub fn common_para_for_test(sc: &str) -> CTemp{
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
    return CTemp{
        job_details,
        range_date,
        scheduled_start_date_time,
        // original_schedule
    };
}
