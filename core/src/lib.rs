

#![forbid(unsafe_code)] 
#![doc = include_str!("../../README.md")]

use anyhow::Result;
use chrono::{DateTime, Utc};

use log::debug;
use model::ScheduleDetails;
use wasm_bindgen::{prelude::wasm_bindgen, *};
use crate::extract::for_details;

pub mod errors;
pub mod model;
use crate::model::RepeatEvery;

pub use self::utils::{
    date_diff,
    get_start_and_last_date_of_month_for_given_date as unstable_get_start_and_last_date_of_month_for_given_date,
    get_week_bounded_days_for_given_date as unstable_get_week_bounded_days_for_given_date,
};

pub use self::extract::for_details as unstable_for_details;
pub use self::weeks::for_week;
pub use self::days::for_days;
pub use self::months::for_month;

mod days;
mod extract;
mod months;
mod utils;
mod weeks;

fn generate_schedule_date_time(
    detail: &ScheduleDetails,
    previous_scheduled_date: DateTime<Utc>,
    start_range_date: DateTime<Utc>,
    end_range_date: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>> {
    let t = match detail.repeat_every {
        model::RepeatEvery::Day => for_details(
            detail,
            previous_scheduled_date,
            start_range_date,
            end_range_date,
            Some(true),
        ),
        model::RepeatEvery::Week => {
            // for_week(
            //     detail,
            //     // DateTime::parse_from_rfc3339("2023-12-14T14:08:15.223Z")
            //     //     .unwrap()
            //     //     .with_timezone(&Utc),
            //     // DateTime::parse_from_rfc3339("2023-12-25T14:08:15.223Z")
            //     //     .unwrap()
            //     //     .with_timezone(&Utc),
            //     // DateTime::parse_from_rfc3339("2023-12-31T14:08:15.223Z")
            //     //     .unwrap()
            //     //     .with_timezone(&Utc),
            //     previous_scheduled_date,
            //     start_range_date,
            //     end_range_date,
            //     Some(true),
            // )
            for_details(
                detail,
                previous_scheduled_date,
                start_range_date,
                end_range_date,
                Some(true),
            )
        }
        model::RepeatEvery::Month => for_details(
            detail,
            previous_scheduled_date,
            start_range_date,
            end_range_date,
            Some(true),
        ),
        model::RepeatEvery::Year => todo!(),
    };
    t
    // match t {
    //     Ok(v) => {
    //         let r = v
    //             .into_iter()
    //             .map(|x| concat_time(x, previous_scheduled_date))
    //             .collect::<Vec<_>>();
    //         dbg!(&r);
    //         dbg!(&previous_scheduled_date);
    //         Ok(r)
    //     }
    //     Err(e) => Err(e),
    // }
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
    return match detail.repeat_every {
        RepeatEvery::Day => {
            unstable_for_details(detail, previous_scheduled_date, start_range_date, end_range_date, Some(true))
        }
        _ => {
            generate_schedule_date_time(
                detail,
                previous_scheduled_date,
                start_range_date,
                end_range_date,
            )
        }
    }
}

#[wasm_bindgen]
// #[no_marg]
pub fn find_schedule_date_time(
    detail: &ScheduleDetails,
    previous_scheduled_date: String,
    start_range_date: String,
    end_range_date: String,
) -> std::result::Result<Vec<String>, JsValue> {
    let mut _previous_scheduled_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&previous_scheduled_date) {
        Ok(v) => _previous_scheduled_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.to_string().into()),
    };

    let mut _start_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&start_range_date) {
        Ok(v) => _start_range_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.to_string().into()),
    };

    let mut _end_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&end_range_date) {
        Ok(v) => _end_range_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.to_string().into()),
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
