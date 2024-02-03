use chrono::{DateTime, Utc};
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::result::Result::Ok;
use timex::{model::ScheduleDetails, schedule_date_times};

#[derive(Deserialize)]
struct Props {
    detail: ScheduleDetails,
    previous_scheduled_date: String,
    start_range_date: String,
    end_range_date: String,
}

#[plugin_fn]
pub fn find_schedule_date_time(Json(props): Json<Props>) -> FnResult<Json<Vec<String>>> {
    let mut _previous_scheduled_date: DateTime<Utc>;
    let detail: ScheduleDetails = props.detail;

    match DateTime::parse_from_rfc3339(&props.previous_scheduled_date) {
        Ok(v) => _previous_scheduled_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.into()),
    };

    let mut _start_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&props.start_range_date) {
        Ok(v) => _start_range_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.into()),
    };

    let mut _end_range_date: DateTime<Utc>;

    match DateTime::parse_from_rfc3339(&props.end_range_date) {
        Ok(v) => _end_range_date = v.with_timezone(&Utc),
        Err(e) => return Err(e.into()),
    };

    let t = schedule_date_times(
        &detail,
        _previous_scheduled_date,
        _start_range_date,
        _end_range_date,
    );

    match t {
        Ok(v) => {
            let temp = v
                .into_iter()
                .map(|x| x.to_rfc3339().into())
                .collect::<Vec<_>>();
            Ok(Json(temp))
        }
        Err(e) => Err(e.into()),
    }

    // return Ok(Json(vec![
    //     "sd1".to_string(),
    //     props.start_range_date,
    //     props.end_range_date,
    //     // _start_range_date.to_rfc3339(),
    // ]));
}
