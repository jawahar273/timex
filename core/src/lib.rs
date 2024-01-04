//! # Schedule Time Generator
//! 

//! Generate range of schedule date and time based on the
//! parameter provided.
//! 
//! ## Feature:
//! - frequency
//! - Time range
//! - Occurrence
//! - Based on daily, weekly, monthly(WIP) and yearly(soon)..
//! 
//! 
//! ## Limitation
//! - Not good with nano second as this project rely on ([`Chrono`](https://docs.rs/chrono/latest/chrono/index.html#limitations)) which has know limitation.
//! - Recommenced to use a persistance database to store the schedule date and time.
//! 

use log::debug;

use anyhow::Result;
use chrono::{DateTime, Utc};

use days::for_days;


use wasm_bindgen::{*, prelude::wasm_bindgen};
use model::ScheduleDetails;
use weeks::for_week;
use months::for_month;

pub mod errors;
pub mod model;
pub use self::utils::{
    get_start_and_last_date_of_month_for_given_date as unstable_get_start_and_last_date_of_month_for_given_date,
    get_week_bounded_days_for_given_date as unstable_get_week_bounded_days_for_given_date,
    date_diff,
};
mod days;
mod utils;
mod weeks;
mod months;

fn generate_schedule_date_time(
    detail: &ScheduleDetails,
    previous_scheduled_date: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    match detail.repeat_every {
        model::RepeatEvery::Day => for_days(
            detail,
            previous_scheduled_date,
            start_range_date,
            end_range_date,
            Some(true),
        ),
        model::RepeatEvery::Week => {
            for_week(
                detail,
                // DateTime::parse_from_rfc3339("2023-12-14T14:08:15.223Z")
                //     .unwrap()
                //     .with_timezone(&Utc),
                // DateTime::parse_from_rfc3339("2023-12-25T14:08:15.223Z")
                //     .unwrap()
                //     .with_timezone(&Utc),
                // DateTime::parse_from_rfc3339("2023-12-31T14:08:15.223Z")
                //     .unwrap()
                //     .with_timezone(&Utc),
                previous_scheduled_date,
                start_range_date,
                end_range_date,
                Some(true),
            )
        }
        model::RepeatEvery::Month => {
            for_month(
                detail,
                previous_scheduled_date,
                start_range_date,
                end_range_date,
            )
        },
        model::RepeatEvery::Year => todo!(),
    }
}

pub fn schedule_date_times(
    detail: &ScheduleDetails,
    previous_scheduled_date: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    // TODO: add validation
    debug!("trigger schedule date time");
    // let previous_scheduled_date: DateTime<Utc> = DateTime::parse_from_rfc3339(
    //     detail.scheduled_start_date_time.as_str()
    // )?.with_timezone(&Utc);
    return generate_schedule_date_time(
        detail,
        previous_scheduled_date,
        start_range_date,
        end_range_date,
    );
}


#[wasm_bindgen]
// #[no_marg]
pub fn find_schedule_date_time(
    detail: &ScheduleDetails,
    previous_scheduled_date: String,
    start_range_date: String,
    end_range_date: String,
) -> std::result::Result<Vec<String>, JsValue> {
    
    let mut _previous_scheduled_date : DateTime<Utc>;
        
     match DateTime::parse_from_rfc3339(&previous_scheduled_date) {
        Ok(v) =>  _previous_scheduled_date = v.with_timezone(&Utc),
        Err(e) => {
            return Err(e.to_string().into())
        },
    }; 

    let mut  _start_range_date: DateTime<Utc>;
        
     match DateTime::parse_from_rfc3339(&start_range_date) {
        Ok(v) => _start_range_date = v.with_timezone(&Utc),
        Err(e) => {
            return Err(e.to_string().into())
        },
    };
    
    let mut _end_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&end_range_date) {
        Ok(v) => _end_range_date = v.with_timezone(&Utc),
        Err(e) => {
            return Err(e.to_string().into())
        },
    };
    
    let t = generate_schedule_date_time(
        detail,
        _previous_scheduled_date,
        _start_range_date,
        _end_range_date,
    );
    match t {
        Ok(v) => Ok(v.into_iter().map(|x| x.to_rfc3339()).collect::<Vec<_>>()),
        Err(e) => Err(e.to_string().into()),
    }
}
