use std::cmp::Ordering;

use chrono::{DateTime, Datelike, Days, NaiveDate, TimeZone, Timelike, Utc, Weekday};

mod diff;

pub use diff::num_diff_i64;


fn convert_utc_date_time_to_date_with_zero_hms(date: &DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap()
}

// today compare with old date is "greater"
// today compare with future date is "less"

// Confirm the given date is present between the date ranges
pub fn check_date_with_given_range(
    date: &DateTime<Utc>,
    _start: &DateTime<Utc>,
    _end: &DateTime<Utc>,
) -> bool {
    let start = convert_utc_date_time_to_date_with_zero_hms(_start);
    let end = convert_utc_date_time_to_date_with_zero_hms(_end);
    let diff = end - start;
    let y = Utc
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap();
    for i in 0..diff.num_days() + 1 {
        // dbg!(start.checked_add_days(Days::new(i as u64)).unwrap());
        match start.checked_add_days(Days::new(i as u64)).unwrap().cmp(&y) {
            Ordering::Equal => return true,
            Ordering::Greater => {}
            Ordering::Less => {}
        }
    }
    return false;
}

// https://github.com/chronotope/chrono/issues/29#issuecomment-1707225993
fn last_of_month(year: i32, month: u32) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
        .or_else(|| chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1))?
        .pred_opt()
}

fn first_of_month(year: i32, month: u32) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(year, month, 1)
        .or_else(|| chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1))
}

fn last_of_year(year: i32) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(year+1, 1, 1)
        .unwrap()
        .pred_opt()
}

fn first_of_year(year: i32) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(year, 1, 1)
}

pub fn get_start_and_last_of_year(
    date: &DateTime<Utc>,
    ) -> (DateTime<Utc>, DateTime<Utc>) {
        let year = date.year();
        (
            first_of_year(year)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc(),
            last_of_year(year)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc(),
        )
    }

pub fn get_start_and_last_date_of_month_for_given_date(
    date: &DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let year = date.year();
    let month = date.month();

    (
        first_of_month(year, month)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc(),
        last_of_month(year, month)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc(),
    )
}

// Date are generated and return as  vector starting from Monday to Sunday
pub fn get_week_bounded_days_for_given_date(date: &DateTime<Utc>) -> Vec<DateTime<Utc>> {
    let year = date.year();
    let week_number = date.iso_week().week();
    let mut result: Vec<DateTime<Utc>> = vec![];

    for inx in 0..7 {
        let o = NaiveDate::from_isoywd_opt(year, week_number, Weekday::try_from(inx).unwrap())
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 0)
            .unwrap()
            .and_utc();
        result.push(o)
    }
    result
}

pub fn _get_start_and_last_date_of_week_for_given_date(
    date: &DateTime<Utc>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let year = date.year();
    let week_number = date.iso_week().week();
    let start = NaiveDate::from_isoywd_opt(year, week_number, Weekday::try_from(0).unwrap())
        .unwrap()
        .and_hms_nano_opt(0, 0, 0, 0)
        .unwrap()
        .and_utc();

    let end = NaiveDate::from_isoywd_opt(year, week_number, Weekday::try_from(6).unwrap())
        .unwrap()
        .and_hms_nano_opt(0, 0, 0, 0)
        .unwrap()
        .and_utc();

    return (start, end);
}

// nano second are not able to copy
pub fn concat_time(
    scheduled_date_time: DateTime<Utc>,
    previous_scheduled_date: DateTime<Utc>,
) -> DateTime<Utc> {
    let remp = Utc
        .with_ymd_and_hms(
            scheduled_date_time.year(),
            scheduled_date_time.month(),
            scheduled_date_time.day(),
            previous_scheduled_date.hour(),
            previous_scheduled_date.minute(),
            previous_scheduled_date.second(),
        )
        .unwrap()
        .with_timezone(&Utc);
    remp
}
#[derive(Debug)]
pub struct DateDiff {
    pub years: i64,
    pub months: i64,
    pub days: i64,
}

pub fn date_diff(start: &DateTime<Utc>, end: &DateTime<Utc>) -> DateDiff {
    // let start =
    //     DateTime::<Utc>::from_utc(chrono::NaiveDate::parse_from_str(date_string, "%d/%m/%Y")
    //         .unwrap()
    //         .and_hms(0, 0, 0), Utc);

    let diff = end.signed_duration_since(start);
    let days = diff.num_days();
    let years = days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30;
    let days = remaining_days % 30;

    DateDiff {
        days,
        years,
        months,
    }
}

#[cfg(test)]
mod test {
    // use crate::utils::{get_week_bounded_days_for_given_date, concat_date};

    use super::*;

    #[test]
    fn test_bounded_week_based_date() {
        let input_date = DateTime::parse_from_rfc3339("2023-12-17T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let actual = get_week_bounded_days_for_given_date(&input_date);

        let monday_expect_date: DateTime<Utc> =
            DateTime::parse_from_rfc3339("2023-12-11T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc);
        let sunday_expect_date: DateTime<Utc> =
            DateTime::parse_from_rfc3339("2023-12-17T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc);

        assert_eq!(monday_expect_date, actual[0], "Monday's are expected");

        assert_eq!(
            sunday_expect_date,
            actual[actual.len() - 1],
            "Sunday's are expected"
        );
    }

    #[test]
    fn test_concat_date() {
        let expect = DateTime::parse_from_rfc3339("2023-12-21T14:08:15.0Z")
            .unwrap()
            .with_timezone(&Utc);
        let actual = concat_time(
            DateTime::parse_from_rfc3339("2023-12-21T14:08:15.223Z")
                .unwrap()
                .with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2023-12-14T14:08:15.223Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        assert_eq!(expect, actual,);
    }
}
