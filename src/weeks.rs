
use std::{ops::Sub, result};

use crate::model::{ScheduleDetails, self};
use chrono::{DateTime, Days, offset, Utc, TimeZone, Datelike, Timelike, IsoWeek, Duration, Weekday, NaiveDateTime, NaiveDate};
use anyhow::{Result, Ok};


fn concatDate(schedule_start: DateTime<Utc>, original_scheduled_start_date_time: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(
        schedule_start.year(),
        schedule_start.month(),
        schedule_start.day(),
        original_scheduled_start_date_time.hour(),
        original_scheduled_start_date_time.minute(),
        original_scheduled_start_date_time.second()).unwrap()
        .with_timezone(&Utc)
}

fn temp_result(
    sr: &Box< DateTime<Utc> >,
    end_date: DateTime<Utc>,
    original_scheduled_start_date_time: DateTime<Utc>,
    repeat_times: i64
) ->  Vec<DateTime<Utc>> {
    
    let mut schedule_start = **sr;
    let diff = end_date - Utc::now();
    let num_of_diff = diff.num_weeks();
    
    let mut temp: Vec< DateTime<Utc> >= vec![];
    
    for _ in 0..num_of_diff {

        temp.push(
            concatDate(schedule_start, original_scheduled_start_date_time)
        );
        
        // linear case
        schedule_start = schedule_start + Duration::weeks(repeat_times as i64);

    }
    
        temp.push(
            concatDate(schedule_start, original_scheduled_start_date_time)
        );
    temp
}

fn get_week_bounded_days_for_given_date(date: &DateTime<Utc>) -> Vec<DateTime<Utc>> {
    let year = date.year();
    let weekNumber = date.iso_week().week();
    let mut result: Vec<DateTime<Utc>> = vec![];
    
    for inx in 0..7 {
        let o = NaiveDate::from_isoywd_opt(year, weekNumber, Weekday::try_from(inx).unwrap()).unwrap().and_hms_nano_opt(0, 0, 0, 0).unwrap().and_utc();
        result.push(
            o
        )
    }   
    result
}

fn get_week_bounded_days_for_given_weekend(weekends: &Vec<String>, weekNumber: &u32, schedule_start: DateTime<Utc>) -> Vec<DateTime<Utc>> {
    let year = schedule_start.year();
    let mut result: Vec<DateTime<Utc>> = vec![];
    
    for inx in 0..7 {
        let o = NaiveDate::from_isoywd_opt(year, *weekNumber, Weekday::try_from(inx).unwrap()).unwrap().and_hms_nano_opt(0, 0, 0, 0).unwrap().and_utc();
        result.push(
            o
        )
    }   
    result
}

pub fn for_week(
    detail: &ScheduleDetails,
    scheduled_start_date_time: DateTime<Utc>,
    // end_date: DateTime<Utc>,
    range_of_days: u64,
) -> Result<Vec<DateTime<Utc>>> {
    
    let repeat_times = detail.repeat_every_number;

    let end_date: DateTime<Utc> = match detail.end_option {
        model::EndOption::After => {
            // TODO: confirm this logic works or not
            // let t = offset::Utc::now().checked_add_days(IsoWeek::new(detail.occurrence_value.unwrap().try_into()?)).unwrap();
            let t = offset::Utc::now() + Duration::weeks(detail.occurrence_value.unwrap().try_into()?);
            t
        },
        model::EndOption::OnThe => {
            let t = DateTime::parse_from_rfc3339(
                // detail.end_date.as_ref().unwrap().as_str()
                detail.end_date.as_ref().unwrap().as_str(),
            )?.with_timezone(&Utc);
            t
        },
        model::EndOption::Never => {
            let t = offset::Utc::now().checked_add_days(Days::new(range_of_days)).unwrap();
            t
        },
    };

    let schedule_start = Box::new(scheduled_start_date_time + Duration::weeks(repeat_times as i64));

    let week_days_for_repeat_every: Vec<String> = detail.week_days_for_repeat_every.clone().unwrap();
    // if week_days_for_repeat_every.len() == 0 {
    //     week_days_for_repeat_every = vec![
    //         schedule_start.weekday()
    //         .to_string()
    //         .parse::<Weekday>().unwrap().to_string()
    //     ];
    // }
    let mut result: Vec< DateTime<Utc> >= vec![];
    // let mut result: Vec< Vec< DateTime<Utc>> >= vec![];
    
    if week_days_for_repeat_every.len() == 0 {
        let u = temp_result(
            &schedule_start,
            end_date,
            scheduled_start_date_time,
            repeat_times as i64,
        );
        return Ok(u);
    }
    

    for  week_day in 0..week_days_for_repeat_every.len() {

        let w = &week_days_for_repeat_every[week_day].parse::<Weekday>().unwrap();
        let week_of_the_year = &(schedule_start.clone()).naive_utc().iso_week().week();
        
        let u = get_week_bounded_days_for_given_weekend(
            &week_days_for_repeat_every,
            week_of_the_year,
            *schedule_start);
        let num = w.num_days_from_monday() as usize;
        result.push(
            u[num]
        );
        
        // *schedule_start = *schedule_start + Duration::weeks(repeat_times as i64);
       
    }
    // for  week_day in 0..week_days_for_repeat_every.len() {
    //     let w = &week_days_for_repeat_every[week_day].parse::<Weekday>().unwrap();
    //     let num = w.num_days_from_monday();
    //     let diff_abs = (Utc::now().weekday().num_days_from_monday() - w.num_days_from_monday()) as i64;
        
    //     let sr = *schedule_start + Duration::days(
    //         diff_abs
    //     );
        
    //     let y = temp_result(
    //         sr,
    //         end_date, 
    //         scheduled_start_date_time,
    //         repeat_times as i64,
    //     );
    //     result.extend(
    //         y
    //     );

    // }
    
    Ok(
        result
    )
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_bounded_week_based_date() {
        let input_date = DateTime::parse_from_rfc3339("2023-12-17T00:00:00Z").unwrap().with_timezone(&Utc);
        let actual = get_week_bounded_days_for_given_date(
            &input_date
        );
        
        let monday_expect_date: DateTime<Utc> = DateTime::parse_from_rfc3339("2023-12-11T00:00:00Z").unwrap().with_timezone(&Utc);
        let sunday_expect_date: DateTime<Utc> = DateTime::parse_from_rfc3339("2023-12-17T00:00:00Z").unwrap().with_timezone(&Utc);
        
        assert_eq!(
            monday_expect_date,
            actual[0],
            "Monday's are expected"
        );
        
        assert_eq!(
            sunday_expect_date,
            actual[actual.len()-1],
            "Sunday's are expected"
        );
    }
}